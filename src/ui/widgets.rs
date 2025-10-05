//! Custom UI widgets for rCandle
//!
//! This module contains custom egui widgets including G-Code editor and console.

use egui::{Color32, RichText, ScrollArea, TextEdit, Ui};
use std::ops::Range;

/// G-Code editor mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    /// View-only mode (read-only)
    View,
    /// Edit mode (can modify content)
    Edit,
}

/// Find and replace state
#[derive(Debug, Clone, Default)]
pub struct FindReplaceState {
    /// Find text
    pub find_text: String,
    /// Replace text
    pub replace_text: String,
    /// Case sensitive search
    pub case_sensitive: bool,
    /// Show find/replace panel
    pub show_panel: bool,
    /// Current match index
    pub current_match: usize,
    /// Total matches found
    pub total_matches: usize,
}

/// G-Code editor widget with syntax highlighting
pub struct GCodeEditor {
    /// Editor mode (view or edit)
    pub mode: EditorMode,
    /// Current line being executed (for highlighting)
    pub current_line: Option<usize>,
    /// Find and replace state
    pub find_replace: FindReplaceState,
    /// Whether to show line numbers
    pub show_line_numbers: bool,
}

impl Default for GCodeEditor {
    fn default() -> Self {
        Self {
            mode: EditorMode::View,
            current_line: None,
            find_replace: FindReplaceState::default(),
            show_line_numbers: true,
        }
    }
}

impl GCodeEditor {
    /// Create a new G-Code editor
    pub fn new() -> Self {
        Self::default()
    }

    /// Set editor mode
    pub fn mode(mut self, mode: EditorMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set current execution line
    pub fn current_line(mut self, line: Option<usize>) -> Self {
        self.current_line = line;
        self
    }

    /// Toggle find/replace panel
    pub fn toggle_find_replace(&mut self) {
        self.find_replace.show_panel = !self.find_replace.show_panel;
    }

    /// Show the G-Code editor UI
    pub fn show(&mut self, ui: &mut Ui, content: &mut String) {
        ui.horizontal(|ui| {
            ui.label("Mode:");
            if ui.selectable_label(self.mode == EditorMode::View, "View").clicked() {
                self.mode = EditorMode::View;
            }
            if ui.selectable_label(self.mode == EditorMode::Edit, "Edit").clicked() {
                self.mode = EditorMode::Edit;
            }
            
            ui.separator();
            
            if ui.button("🔍 Find").clicked() {
                self.toggle_find_replace();
            }
        });

        ui.separator();

        // Find/Replace panel
        if self.find_replace.show_panel {
            self.show_find_replace_panel(ui, content);
            ui.separator();
        }

        // Main editor area
        ScrollArea::vertical()
            .id_source("gcode_editor_scroll")
            .show(ui, |ui| {
                if content.is_empty() {
                    ui.centered_and_justified(|ui| {
                        ui.label("No G-Code loaded");
                    });
                } else {
                    match self.mode {
                        EditorMode::View => self.show_view_mode(ui, content),
                        EditorMode::Edit => self.show_edit_mode(ui, content),
                    }
                }
            });

        // Status line
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(format!("Lines: {}", content.lines().count()));
            if let Some(line) = self.current_line {
                ui.separator();
                ui.colored_label(Color32::YELLOW, format!("▶ Line {}", line + 1));
            }
        });
    }

    /// Show view mode (read-only with syntax highlighting)
    fn show_view_mode(&self, ui: &mut Ui, content: &str) {
        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
        
        for (line_num, line) in content.lines().enumerate() {
            ui.horizontal(|ui| {
                // Line number
                if self.show_line_numbers {
                    let line_num_text = format!("{:4} ", line_num + 1);
                    let mut color = Color32::DARK_GRAY;
                    
                    // Highlight current execution line
                    if Some(line_num) == self.current_line {
                        ui.painter().rect_filled(
                            ui.available_rect_before_wrap(),
                            0.0,
                            Color32::from_rgba_unmultiplied(255, 255, 0, 30),
                        );
                        color = Color32::YELLOW;
                    }
                    
                    ui.label(RichText::new(line_num_text).color(color));
                }
                
                // Syntax highlighted line
                self.show_highlighted_line(ui, line);
            });
        }
    }

    /// Show edit mode (editable text with syntax highlighting hints)
    fn show_edit_mode(&self, ui: &mut Ui, content: &mut String) {
        let text_edit = TextEdit::multiline(content)
            .code_editor()
            .desired_width(f32::INFINITY)
            .desired_rows(25);
        
        ui.add(text_edit);
    }

    /// Show find and replace panel
    fn show_find_replace_panel(&mut self, ui: &mut Ui, content: &str) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Find:");
                let response = ui.add(
                    TextEdit::singleline(&mut self.find_replace.find_text)
                        .desired_width(200.0)
                );
                
                if ui.button("▼ Next").clicked() || response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.find_next(content);
                }
                
                if ui.button("▲ Prev").clicked() {
                    self.find_prev(content);
                }
                
                ui.checkbox(&mut self.find_replace.case_sensitive, "Case sensitive");
                
                if self.find_replace.total_matches > 0 {
                    ui.label(format!(
                        "{}/{}",
                        self.find_replace.current_match + 1,
                        self.find_replace.total_matches
                    ));
                }
            });
            
            ui.horizontal(|ui| {
                ui.label("Replace:");
                ui.add(
                    TextEdit::singleline(&mut self.find_replace.replace_text)
                        .desired_width(200.0)
                );
                
                if ui.button("Replace").clicked() {
                    // TODO: Implement replace
                }
                
                if ui.button("Replace All").clicked() {
                    // TODO: Implement replace all
                }
            });
        });
    }

    /// Find next occurrence
    fn find_next(&mut self, content: &str) {
        if self.find_replace.find_text.is_empty() {
            return;
        }

        let matches = self.find_matches(content);
        self.find_replace.total_matches = matches.len();
        
        if !matches.is_empty() {
            self.find_replace.current_match = 
                (self.find_replace.current_match + 1) % matches.len();
        }
    }

    /// Find previous occurrence
    fn find_prev(&mut self, content: &str) {
        if self.find_replace.find_text.is_empty() {
            return;
        }

        let matches = self.find_matches(content);
        self.find_replace.total_matches = matches.len();
        
        if !matches.is_empty() {
            if self.find_replace.current_match == 0 {
                self.find_replace.current_match = matches.len() - 1;
            } else {
                self.find_replace.current_match -= 1;
            }
        }
    }

    /// Find all matches in content
    fn find_matches(&self, content: &str) -> Vec<Range<usize>> {
        let mut matches = Vec::new();
        let search_text = if self.find_replace.case_sensitive {
            self.find_replace.find_text.clone()
        } else {
            self.find_replace.find_text.to_lowercase()
        };
        
        let content_to_search = if self.find_replace.case_sensitive {
            content.to_string()
        } else {
            content.to_lowercase()
        };

        let mut start = 0;
        while let Some(pos) = content_to_search[start..].find(&search_text) {
            let abs_pos = start + pos;
            matches.push(abs_pos..abs_pos + search_text.len());
            start = abs_pos + 1;
        }

        matches
    }

    /// Show a single line with syntax highlighting
    fn show_highlighted_line(&self, ui: &mut Ui, line: &str) {
        let trimmed = line.trim();
        
        // Empty line
        if trimmed.is_empty() {
            ui.label("");
            return;
        }

        // Comment line
        if trimmed.starts_with(';') || trimmed.starts_with('(') {
            ui.label(RichText::new(line).color(Color32::DARK_GREEN));
            return;
        }

        // Parse and highlight tokens
        let mut current_pos = 0;
        let chars: Vec<char> = line.chars().collect();
        
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            
            while current_pos < chars.len() {
                let ch = chars[current_pos];
                
                // Skip whitespace
                if ch.is_whitespace() {
                    ui.label(" ");
                    current_pos += 1;
                    continue;
                }
                
                // Comment
                if ch == ';' || ch == '(' {
                    let comment: String = chars[current_pos..].iter().collect();
                    ui.label(RichText::new(comment).color(Color32::DARK_GREEN));
                    break;
                }
                
                // G/M/T/F/S commands
                if ch.is_ascii_alphabetic() {
                    let start = current_pos;
                    current_pos += 1;
                    
                    // Collect the number following the letter
                    while current_pos < chars.len() && 
                          (chars[current_pos].is_ascii_digit() || 
                           chars[current_pos] == '.' || 
                           chars[current_pos] == '-') {
                        current_pos += 1;
                    }
                    
                    let token: String = chars[start..current_pos].iter().collect();
                    let color = self.get_token_color(&token);
                    ui.label(RichText::new(token).color(color));
                    continue;
                }
                
                // Numbers (parameters)
                if ch.is_ascii_digit() || ch == '-' || ch == '.' {
                    let start = current_pos;
                    while current_pos < chars.len() && 
                          (chars[current_pos].is_ascii_digit() || 
                           chars[current_pos] == '.' || 
                           chars[current_pos] == '-') {
                        current_pos += 1;
                    }
                    
                    let number: String = chars[start..current_pos].iter().collect();
                    ui.label(RichText::new(number).color(Color32::LIGHT_BLUE));
                    continue;
                }
                
                // Other characters
                ui.label(ch.to_string());
                current_pos += 1;
            }
        });
    }

    /// Get color for a token based on its type
    fn get_token_color(&self, token: &str) -> Color32 {
        if token.is_empty() {
            return Color32::WHITE;
        }

        let first_char = token.chars().next().unwrap().to_ascii_uppercase();
        
        match first_char {
            'G' => Color32::from_rgb(100, 200, 255), // Light blue for G-codes
            'M' => Color32::from_rgb(255, 150, 100), // Orange for M-codes
            'T' => Color32::from_rgb(255, 200, 100), // Yellow for tool changes
            'F' => Color32::from_rgb(150, 255, 150), // Light green for feed rate
            'S' => Color32::from_rgb(255, 150, 255), // Pink for spindle speed
            'X' | 'Y' | 'Z' => Color32::from_rgb(200, 200, 255), // Light purple for coordinates
            'I' | 'J' | 'K' => Color32::from_rgb(200, 255, 200), // Light green for arc params
            'P' | 'Q' | 'R' => Color32::from_rgb(255, 255, 150), // Light yellow for other params
            _ => Color32::WHITE,
        }
    }
}

/// Log message severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// Debug messages (verbose)
    Debug,
    /// Informational messages
    Info,
    /// Warning messages
    Warning,
    /// Error messages
    Error,
    /// Sent commands (outgoing)
    Sent,
    /// Received responses (incoming)
    Received,
}

impl LogLevel {
    /// Get the display color for this log level
    pub fn color(&self) -> Color32 {
        match self {
            LogLevel::Debug => Color32::from_rgb(128, 128, 128),    // Gray
            LogLevel::Info => Color32::from_rgb(200, 200, 200),     // Light gray
            LogLevel::Warning => Color32::from_rgb(255, 200, 0),    // Yellow/Orange
            LogLevel::Error => Color32::from_rgb(255, 80, 80),      // Red
            LogLevel::Sent => Color32::from_rgb(100, 200, 255),     // Light blue
            LogLevel::Received => Color32::from_rgb(100, 255, 150), // Light green
        }
    }

    /// Get the display prefix for this log level
    pub fn prefix(&self) -> &str {
        match self {
            LogLevel::Debug => "[DEBUG]",
            LogLevel::Info => "[INFO]",
            LogLevel::Warning => "[WARN]",
            LogLevel::Error => "[ERROR]",
            LogLevel::Sent => ">>>",
            LogLevel::Received => "<<<",
        }
    }
}

/// A single console message
#[derive(Debug, Clone)]
pub struct ConsoleMessage {
    /// Message timestamp
    pub timestamp: std::time::SystemTime,
    /// Message severity level
    pub level: LogLevel,
    /// Message text
    pub text: String,
}

impl ConsoleMessage {
    /// Create a new console message
    pub fn new(level: LogLevel, text: String) -> Self {
        Self {
            timestamp: std::time::SystemTime::now(),
            level,
            text,
        }
    }

    /// Format the timestamp for display
    pub fn format_timestamp(&self) -> String {
        use std::time::UNIX_EPOCH;
        
        if let Ok(duration) = self.timestamp.duration_since(UNIX_EPOCH) {
            let secs = duration.as_secs();
            let hours = (secs / 3600) % 24;
            let minutes = (secs / 60) % 60;
            let seconds = secs % 60;
            let millis = duration.subsec_millis();
            format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
        } else {
            "00:00:00.000".to_string()
        }
    }
}

/// Console widget for displaying log messages and command input
pub struct Console {
    /// Console messages history
    messages: Vec<ConsoleMessage>,
    /// Command input buffer
    command_input: String,
    /// Command history (for up/down arrow navigation)
    command_history: Vec<String>,
    /// Current position in command history
    history_index: Option<usize>,
    /// Maximum number of messages to keep
    max_messages: usize,
    /// Auto-scroll to bottom
    auto_scroll: bool,
    /// Show timestamps
    show_timestamps: bool,
    /// Log level filter
    filter_level: Option<LogLevel>,
    /// Whether to show debug messages
    show_debug: bool,
    /// Whether to show info messages
    show_info: bool,
    /// Whether to show warning messages
    show_warning: bool,
    /// Whether to show error messages
    show_error: bool,
    /// Whether to show sent messages
    show_sent: bool,
    /// Whether to show received messages
    show_received: bool,
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

impl Console {
    /// Create a new console widget
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            command_input: String::new(),
            command_history: Vec::new(),
            history_index: None,
            max_messages: 1000,
            auto_scroll: true,
            show_timestamps: true,
            filter_level: None,
            show_debug: false,
            show_info: true,
            show_warning: true,
            show_error: true,
            show_sent: true,
            show_received: true,
        }
    }

    /// Add a message to the console
    pub fn add_message(&mut self, level: LogLevel, text: String) {
        self.messages.push(ConsoleMessage::new(level, text));
        
        // Trim to max messages
        if self.messages.len() > self.max_messages {
            self.messages.drain(0..self.messages.len() - self.max_messages);
        }
    }

    /// Add a debug message
    pub fn debug(&mut self, text: String) {
        self.add_message(LogLevel::Debug, text);
    }

    /// Add an info message
    pub fn info(&mut self, text: String) {
        self.add_message(LogLevel::Info, text);
    }

    /// Add a warning message
    pub fn warning(&mut self, text: String) {
        self.add_message(LogLevel::Warning, text);
    }

    /// Add an error message
    pub fn error(&mut self, text: String) {
        self.add_message(LogLevel::Error, text);
    }

    /// Add a sent command message
    pub fn sent(&mut self, text: String) {
        self.add_message(LogLevel::Sent, text);
    }

    /// Add a received response message
    pub fn received(&mut self, text: String) {
        self.add_message(LogLevel::Received, text);
    }

    /// Clear all messages
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    /// Get the current command input
    pub fn command_input(&self) -> &str {
        &self.command_input
    }

    /// Check if a message should be displayed based on filters
    fn should_display(&self, message: &ConsoleMessage) -> bool {
        // Check log level filter
        if let Some(filter) = self.filter_level {
            if message.level != filter {
                return false;
            }
        }

        // Check individual level filters
        match message.level {
            LogLevel::Debug => self.show_debug,
            LogLevel::Info => self.show_info,
            LogLevel::Warning => self.show_warning,
            LogLevel::Error => self.show_error,
            LogLevel::Sent => self.show_sent,
            LogLevel::Received => self.show_received,
        }
    }

    /// Show the console widget
    pub fn show(&mut self, ui: &mut Ui) -> Option<String> {
        let mut submitted_command = None;

        // Filter controls
        ui.horizontal(|ui| {
            ui.label("Show:");
            ui.checkbox(&mut self.show_debug, "Debug");
            ui.checkbox(&mut self.show_info, "Info");
            ui.checkbox(&mut self.show_warning, "Warn");
            ui.checkbox(&mut self.show_error, "Error");
            ui.separator();
            ui.checkbox(&mut self.show_sent, "Sent");
            ui.checkbox(&mut self.show_received, "Received");
            ui.separator();
            ui.checkbox(&mut self.show_timestamps, "Timestamps");
            ui.checkbox(&mut self.auto_scroll, "Auto-scroll");
            
            if ui.button("Clear").clicked() {
                self.clear();
            }
        });

        ui.separator();

        // Console output area
        let scroll = ScrollArea::vertical()
            .auto_shrink([false, false])
            .stick_to_bottom(self.auto_scroll);

        scroll.show(ui, |ui| {
            ui.vertical(|ui| {
                for message in &self.messages {
                    if !self.should_display(message) {
                        continue;
                    }

                    ui.horizontal(|ui| {
                        // Timestamp
                        if self.show_timestamps {
                            ui.label(
                                RichText::new(message.format_timestamp())
                                    .color(Color32::from_rgb(150, 150, 150))
                                    .monospace(),
                            );
                        }

                        // Level prefix
                        ui.label(
                            RichText::new(message.level.prefix())
                                .color(message.level.color())
                                .monospace(),
                        );

                        // Message text
                        ui.label(RichText::new(&message.text).monospace());
                    });
                }
            });
        });

        ui.separator();

        // Command input area
        ui.horizontal(|ui| {
            ui.label("Command:");
            
            let response = ui.add(
                TextEdit::singleline(&mut self.command_input)
                    .desired_width(ui.available_width() - 100.0)
                    .hint_text("Enter command...")
                    .font(egui::TextStyle::Monospace),
            );

            // Handle up/down arrow keys for command history
            if response.has_focus() {
                if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                    if !self.command_history.is_empty() {
                        match self.history_index {
                            None => {
                                // Start at the most recent command
                                self.history_index = Some(self.command_history.len() - 1);
                                self.command_input = self.command_history[self.history_index.unwrap()].clone();
                            }
                            Some(idx) if idx > 0 => {
                                // Go to previous command
                                self.history_index = Some(idx - 1);
                                self.command_input = self.command_history[self.history_index.unwrap()].clone();
                            }
                            _ => {}
                        }
                    }
                } else if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                    if let Some(idx) = self.history_index {
                        if idx < self.command_history.len() - 1 {
                            // Go to next command
                            self.history_index = Some(idx + 1);
                            self.command_input = self.command_history[self.history_index.unwrap()].clone();
                        } else {
                            // Clear input (past most recent command)
                            self.history_index = None;
                            self.command_input.clear();
                        }
                    }
                } else if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    // Submit command
                    if !self.command_input.trim().is_empty() {
                        submitted_command = Some(self.command_input.clone());
                        
                        // Add to history
                        self.command_history.push(self.command_input.clone());
                        
                        // Trim history to reasonable size
                        if self.command_history.len() > 100 {
                            self.command_history.remove(0);
                        }
                        
                        // Log the sent command
                        self.sent(self.command_input.clone());
                        
                        // Clear input
                        self.command_input.clear();
                        self.history_index = None;
                    }
                }
            }

            if ui.button("Send").clicked() && !self.command_input.trim().is_empty() {
                submitted_command = Some(self.command_input.clone());
                
                // Add to history
                self.command_history.push(self.command_input.clone());
                
                // Trim history to reasonable size
                if self.command_history.len() > 100 {
                    self.command_history.remove(0);
                }
                
                // Log the sent command
                self.sent(self.command_input.clone());
                
                // Clear input
                self.command_input.clear();
                self.history_index = None;
            }
        });

        submitted_command
    }
}
