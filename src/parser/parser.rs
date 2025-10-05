//! G-Code Parser
//!
//! This module converts tokens into structured commands and maintains modal state.

use super::segment::Segment;
use super::tokenizer::Token;
use super::types::*;
use crate::utils::error::{Error, Result};
use std::collections::HashMap;

/// Represents a parsed G-Code command with all its parameters
#[derive(Debug, Clone)]
pub struct ParsedCommand {
    /// Line number (if present)
    pub line_number: Option<u32>,
    /// G-code command number (if present)
    pub g_command: Option<u32>,
    /// M-code command number (if present)
    pub m_command: Option<u32>,
    /// T-code (tool) number (if present)
    pub t_command: Option<u32>,
    /// Parameters map (letter -> value)
    pub parameters: HashMap<char, f64>,
    /// Feed rate (F parameter)
    pub feed_rate: Option<f64>,
    /// Spindle speed (S parameter)
    pub spindle_speed: Option<f64>,
    /// Comments
    pub comments: Vec<String>,
}

impl ParsedCommand {
    /// Create a new empty command
    pub fn new() -> Self {
        Self {
            line_number: None,
            g_command: None,
            m_command: None,
            t_command: None,
            parameters: HashMap::new(),
            feed_rate: None,
            spindle_speed: None,
            comments: Vec::new(),
        }
    }

    /// Get parameter value by letter
    pub fn get_param(&self, letter: char) -> Option<f64> {
        self.parameters.get(&letter).copied()
    }

    /// Check if this is a motion command (G0, G1, G2, G3)
    pub fn is_motion_command(&self) -> bool {
        matches!(self.g_command, Some(0) | Some(1) | Some(2) | Some(3))
    }
}

impl Default for ParsedCommand {
    fn default() -> Self {
        Self::new()
    }
}

/// Parser state that tracks modal values
#[derive(Debug, Clone)]
pub struct ParserState {
    /// Current positioning mode (absolute/relative)
    pub positioning_mode: PositioningMode,
    /// Current units
    pub units: Units,
    /// Current plane for arcs
    pub plane: Plane,
    /// Current feed rate mode
    pub feed_rate_mode: FeedRateMode,
    /// Current work coordinate system
    pub coordinate_system: CoordinateSystem,
    /// Current position
    pub position: Point3D,
    /// Current feed rate
    pub feed_rate: f64,
    /// Current spindle speed
    pub spindle_speed: f64,
    /// Spindle state
    pub spindle_state: SpindleState,
    /// Coolant state
    pub coolant_state: CoolantState,
    /// Active tool number
    pub tool: u32,
    /// Modal G-command (for motion)
    pub modal_g_command: Option<u32>,
}

impl ParserState {
    /// Create a new parser state with default values
    pub fn new() -> Self {
        Self {
            positioning_mode: PositioningMode::Absolute,
            units: Units::Metric,
            plane: Plane::XY,
            feed_rate_mode: FeedRateMode::UnitsPerMinute,
            coordinate_system: CoordinateSystem::default(),
            position: Point3D::zero(),
            feed_rate: 0.0,
            spindle_speed: 0.0,
            spindle_state: SpindleState::Off,
            coolant_state: CoolantState::Off,
            tool: 0,
            modal_g_command: None,
        }
    }
}

impl Default for ParserState {
    fn default() -> Self {
        Self::new()
    }
}

/// G-Code parser
pub struct Parser {
    state: ParserState,
}

impl Parser {
    /// Create a new parser with default state
    pub fn new() -> Self {
        Self {
            state: ParserState::new(),
        }
    }

    /// Create a parser with a specific initial state
    pub fn with_state(state: ParserState) -> Self {
        Self { state }
    }

    /// Get the current parser state
    pub fn state(&self) -> &ParserState {
        &self.state
    }

    /// Parse a list of tokens into commands
    pub fn parse_tokens(&mut self, tokens: &[Token]) -> Result<Vec<ParsedCommand>> {
        let mut commands = Vec::new();
        let mut current_command = ParsedCommand::new();
        let mut has_content = false;

        for token in tokens {
            match token {
                Token::GCommand(n) => {
                    if has_content {
                        commands.push(current_command);
                        current_command = ParsedCommand::new();
                    }
                    current_command.g_command = Some(*n);
                    has_content = true;
                }
                Token::MCommand(n) => {
                    if has_content {
                        commands.push(current_command);
                        current_command = ParsedCommand::new();
                    }
                    current_command.m_command = Some(*n);
                    has_content = true;
                }
                Token::TCommand(n) => {
                    if has_content {
                        commands.push(current_command);
                        current_command = ParsedCommand::new();
                    }
                    current_command.t_command = Some(*n);
                    has_content = true;
                }
                Token::SCommand(v) => {
                    current_command.spindle_speed = Some(*v);
                    has_content = true;
                }
                Token::FCommand(v) => {
                    current_command.feed_rate = Some(*v);
                    has_content = true;
                }
                Token::Parameter { letter, value } => {
                    current_command.parameters.insert(*letter, *value);
                    has_content = true;
                }
                Token::LineNumber(n) => {
                    current_command.line_number = Some(*n);
                }
                Token::Comment(s) => {
                    current_command.comments.push(s.clone());
                }
                Token::Checksum(_) => {
                    // Checksums are validated during tokenization, ignore here
                }
                Token::EndOfLine => {
                    if has_content {
                        commands.push(current_command);
                        current_command = ParsedCommand::new();
                        has_content = false;
                    }
                }
            }
        }

        // Push last command if it has content
        if has_content {
            commands.push(current_command);
        }

        Ok(commands)
    }

    /// Parse commands and generate motion segments
    pub fn generate_segments(&mut self, commands: &[ParsedCommand]) -> Result<Vec<Segment>> {
        let mut segments = Vec::new();

        for command in commands {
            // Update modal state
            self.update_state(command)?;

            // Check if this is a motion command or if we should apply modal G command
            let is_motion = command.is_motion_command();
            let has_params = !command.parameters.is_empty();
            let should_generate = is_motion || (has_params && self.state.modal_g_command.is_some());

            if should_generate {
                if let Some(segment) = self.create_segment(command)? {
                    segments.push(segment);
                }
            }
        }

        Ok(segments)
    }

    /// Update parser state based on command
    fn update_state(&mut self, command: &ParsedCommand) -> Result<()> {
        // Update feed rate
        if let Some(f) = command.feed_rate {
            self.state.feed_rate = f;
        }

        // Update spindle speed
        if let Some(s) = command.spindle_speed {
            self.state.spindle_speed = s;
        }

        // Update tool
        if let Some(t) = command.t_command {
            self.state.tool = t;
        }

        // Process G-codes that affect state
        if let Some(g) = command.g_command {
            // Track modal motion commands (G0-G3)
            if matches!(g, 0 | 1 | 2 | 3) {
                self.state.modal_g_command = Some(g);
            }
            
            match g {
                17 => self.state.plane = Plane::XY,
                18 => self.state.plane = Plane::XZ,
                19 => self.state.plane = Plane::YZ,
                20 => self.state.units = Units::Imperial,
                21 => self.state.units = Units::Metric,
                90 => self.state.positioning_mode = PositioningMode::Absolute,
                91 => self.state.positioning_mode = PositioningMode::Relative,
                93 => self.state.feed_rate_mode = FeedRateMode::InverseTime,
                94 => self.state.feed_rate_mode = FeedRateMode::UnitsPerMinute,
                54 => self.state.coordinate_system = CoordinateSystem::G54,
                55 => self.state.coordinate_system = CoordinateSystem::G55,
                56 => self.state.coordinate_system = CoordinateSystem::G56,
                57 => self.state.coordinate_system = CoordinateSystem::G57,
                58 => self.state.coordinate_system = CoordinateSystem::G58,
                59 => self.state.coordinate_system = CoordinateSystem::G59,
                _ => {}
            }
        }

        // Process M-codes that affect state
        if let Some(m) = command.m_command {
            match m {
                3 => self.state.spindle_state = SpindleState::Clockwise,
                4 => self.state.spindle_state = SpindleState::CounterClockwise,
                5 => self.state.spindle_state = SpindleState::Off,
                7 => {
                    self.state.coolant_state = match self.state.coolant_state {
                        CoolantState::Flood => CoolantState::Both,
                        _ => CoolantState::Mist,
                    };
                }
                8 => {
                    self.state.coolant_state = match self.state.coolant_state {
                        CoolantState::Mist => CoolantState::Both,
                        _ => CoolantState::Flood,
                    };
                }
                9 => self.state.coolant_state = CoolantState::Off,
                _ => {}
            }
        }

        Ok(())
    }

    /// Create a motion segment from a command
    fn create_segment(&mut self, command: &ParsedCommand) -> Result<Option<Segment>> {
        // Use command's G code or fall back to modal G command
        let g = match command.g_command.or(self.state.modal_g_command) {
            Some(g) => g,
            None => return Ok(None),
        };

        // Calculate target position
        let target = self.calculate_target_position(command)?;

        let segment = match g {
            0 => {
                // Rapid positioning
                Some(Segment::rapid(self.state.position, target))
            }
            1 => {
                // Linear interpolation
                Some(Segment::linear(
                    self.state.position,
                    target,
                    self.state.feed_rate,
                ))
            }
            2 | 3 => {
                // Arc interpolation
                let center = self.calculate_arc_center(command, target)?;
                let direction = if g == 2 {
                    ArcDirection::Clockwise
                } else {
                    ArcDirection::CounterClockwise
                };
                Some(Segment::arc(
                    self.state.position,
                    target,
                    center,
                    direction,
                    self.state.feed_rate,
                ))
            }
            _ => None,
        };

        // Update position
        self.state.position = target;

        // Add line number and spindle speed if available
        Ok(segment.map(|s| {
            let mut seg = s.with_spindle_speed(self.state.spindle_speed);
            if let Some(ln) = command.line_number {
                seg = seg.with_line_number(ln);
            }
            seg
        }))
    }

    /// Calculate target position from command parameters
    fn calculate_target_position(&self, command: &ParsedCommand) -> Result<Point3D> {
        let mut target = self.state.position;

        // Get coordinate values from parameters
        let x = command.get_param('X');
        let y = command.get_param('Y');
        let z = command.get_param('Z');

        // Apply based on positioning mode
        match self.state.positioning_mode {
            PositioningMode::Absolute => {
                if let Some(x_val) = x {
                    target.x = x_val;
                }
                if let Some(y_val) = y {
                    target.y = y_val;
                }
                if let Some(z_val) = z {
                    target.z = z_val;
                }
            }
            PositioningMode::Relative => {
                if let Some(x_val) = x {
                    target.x += x_val;
                }
                if let Some(y_val) = y {
                    target.y += y_val;
                }
                if let Some(z_val) = z {
                    target.z += z_val;
                }
            }
        }

        Ok(target)
    }

    /// Calculate arc center point from I, J, K or R parameters
    fn calculate_arc_center(&self, command: &ParsedCommand, target: Point3D) -> Result<Point3D> {
        // Try I, J, K parameters first (offset from start point)
        let i = command.get_param('I');
        let j = command.get_param('J');
        let k = command.get_param('K');

        if i.is_some() || j.is_some() || k.is_some() {
            let center = Point3D::new(
                self.state.position.x + i.unwrap_or(0.0),
                self.state.position.y + j.unwrap_or(0.0),
                self.state.position.z + k.unwrap_or(0.0),
            );
            return Ok(center);
        }

        // Try R parameter (radius)
        if let Some(r) = command.get_param('R') {
            return self.calculate_arc_center_from_radius(r, target);
        }

        Err(Error::Parse(
            "Arc command requires I, J, K or R parameters".to_string(),
        ))
    }

    /// Calculate arc center from radius parameter
    fn calculate_arc_center_from_radius(&self, radius: f64, target: Point3D) -> Result<Point3D> {
        let start = self.state.position;
        
        // Calculate midpoint
        let mid_x = (start.x + target.x) / 2.0;
        let mid_y = (start.y + target.y) / 2.0;
        
        // Calculate distance from start to end
        let chord_length = start.distance_to(&target);
        
        if chord_length > 2.0 * radius.abs() {
            return Err(Error::Parse(format!(
                "Arc radius {} is too small for chord length {}",
                radius, chord_length
            )));
        }
        
        // Calculate distance from midpoint to center
        let h = (radius * radius - (chord_length / 2.0).powi(2)).sqrt();
        
        // Calculate perpendicular direction
        let dx = target.x - start.x;
        let dy = target.y - start.y;
        let perp_x = -dy / chord_length;
        let perp_y = dx / chord_length;
        
        // Calculate center (choose side based on radius sign)
        let sign = if radius > 0.0 { 1.0 } else { -1.0 };
        let center = Point3D::new(
            mid_x + sign * h * perp_x,
            mid_y + sign * h * perp_y,
            start.z, // Arc is in the current plane
        );
        
        Ok(center)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tokenizer::Tokenizer;

    #[test]
    fn test_parse_simple_command() {
        let input = "G0 X10 Y20 Z5";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();

        let mut parser = Parser::new();
        let commands = parser.parse_tokens(&tokens).unwrap();

        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].g_command, Some(0));
        assert_eq!(commands[0].get_param('X'), Some(10.0));
        assert_eq!(commands[0].get_param('Y'), Some(20.0));
        assert_eq!(commands[0].get_param('Z'), Some(5.0));
    }

    #[test]
    fn test_generate_linear_segment() {
        let input = "G1 X10 Y10 F1000";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();

        let mut parser = Parser::new();
        let commands = parser.parse_tokens(&tokens).unwrap();
        let segments = parser.generate_segments(&commands).unwrap();

        assert_eq!(segments.len(), 1);
        assert!(segments[0].is_cutting());
        assert_eq!(segments[0].feed_rate, 1000.0);
    }

    #[test]
    fn test_modal_state() {
        let input = "G90\nG1 X10\nX20";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();

        let mut parser = Parser::new();
        let commands = parser.parse_tokens(&tokens).unwrap();
        let segments = parser.generate_segments(&commands).unwrap();

        // Should generate 2 segments (G1 is modal)
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[1].end.x, 20.0);
    }

    #[test]
    fn test_relative_positioning() {
        let input = "G91\nG1 X10\nX10";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();

        let mut parser = Parser::new();
        let commands = parser.parse_tokens(&tokens).unwrap();
        let segments = parser.generate_segments(&commands).unwrap();

        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0].end.x, 10.0);
        assert_eq!(segments[1].end.x, 20.0); // Relative to previous
    }
}
