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
