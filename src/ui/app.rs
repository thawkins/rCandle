//! Main application structure for rCandle

use crate::{
    parser::{Parser, Preprocessor, Tokenizer},
    settings::Settings,
    state::AppState,
    ui::widgets::{Console, GCodeEditor},
};
use std::path::PathBuf;

/// Main rCandle application state
pub struct RCandleApp {
    /// Application settings
    settings: Settings,
    /// Application state (machine, program, etc.)
    app_state: AppState,
    /// Connection status display
    status_message: String,
    /// Currently loaded file path
    current_file: Option<PathBuf>,
    /// G-Code content
    gcode_content: String,
    /// Parser instance
    parser: Parser,
    /// Preprocessor instance
    preprocessor: Preprocessor,
    /// G-Code editor widget
    gcode_editor: GCodeEditor,
    /// Console widget
    console: Console,
    /// Show console panel
    show_console: bool,
}

impl RCandleApp {
    /// Create a new rCandle application instance
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Load settings
        let settings = Settings::load_or_default();
        
        // Initialize application state
        let app_state = AppState::new();
        
        // Create parser and preprocessor
        let parser = Parser::new();
        let preprocessor = Preprocessor::new();
        
        // Create G-Code editor
        let gcode_editor = GCodeEditor::new();
        
        // Create console
        let mut console = Console::new();
        console.info("rCandle initialized".to_string());
        console.info("Ready to connect to GRBL controller".to_string());
        
        tracing::info!("rCandle UI initialized");
        
        Self {
            settings,
            app_state,
            status_message: "Ready".to_string(),
            current_file: None,
            gcode_content: String::new(),
            parser,
            preprocessor,
            gcode_editor,
            console,
            show_console: true,
        }
    }

    /// Open a G-Code file
    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("G-Code", &["gcode", "nc", "ngc", "txt"])
            .add_filter("All Files", &["*"])
            .pick_file()
        {
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    self.gcode_content = content;
                    self.current_file = Some(path.clone());
                    self.status_message = format!("Loaded: {}", path.display());
                    self.console.info(format!("Loaded file: {}", path.display()));
                    tracing::info!("Loaded G-Code file: {:?}", path);
                    
                    // Parse the G-Code
                    self.parse_gcode();
                }
                Err(e) => {
                    self.status_message = format!("Error loading file: {}", e);
                    self.console.error(format!("Failed to load file: {}", e));
                    tracing::error!("Failed to load file {:?}: {}", path, e);
                }
            }
        }
    }

    /// Save the current G-Code to a file
    fn save_file(&mut self) {
        if let Some(path) = &self.current_file {
            if let Err(e) = std::fs::write(path, &self.gcode_content) {
                self.status_message = format!("Error saving file: {}", e);
                self.console.error(format!("Failed to save file: {}", e));
                tracing::error!("Failed to save file {:?}: {}", path, e);
            } else {
                self.status_message = format!("Saved: {}", path.display());
                self.console.info(format!("Saved file: {}", path.display()));
                tracing::info!("Saved G-Code file: {:?}", path);
            }
        } else {
            self.save_file_as();
        }
    }

    /// Save the current G-Code to a new file
    fn save_file_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("G-Code", &["gcode", "nc", "ngc"])
            .save_file()
        {
            if let Err(e) = std::fs::write(&path, &self.gcode_content) {
                self.status_message = format!("Error saving file: {}", e);
                self.console.error(format!("Failed to save file: {}", e));
                tracing::error!("Failed to save file {:?}: {}", path, e);
            } else {
                self.current_file = Some(path.clone());
                self.status_message = format!("Saved: {}", path.display());
                self.console.info(format!("Saved file: {}", path.display()));
                tracing::info!("Saved G-Code file: {:?}", path);
            }
        }
    }

    /// Parse the current G-Code content
    fn parse_gcode(&mut self) {
        self.console.info("Parsing G-Code...".to_string());
        
        // Tokenize
        let mut tokenizer = Tokenizer::new(&self.gcode_content);
        let tokens = match tokenizer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                self.status_message = format!("Tokenization error: {}", e);
                self.console.error(format!("Tokenization failed: {}", e));
                tracing::error!("Failed to tokenize G-Code: {}", e);
                return;
            }
        };
        
        self.console.debug(format!("Tokenized {} tokens", tokens.len()));

        // Parse tokens to commands
        let commands = match self.parser.parse_tokens(&tokens) {
            Ok(c) => c,
            Err(e) => {
                self.status_message = format!("Parse error: {}", e);
                self.console.error(format!("Parse failed: {}", e));
                tracing::error!("Failed to parse G-Code: {}", e);
                return;
            }
        };
        
        self.console.debug(format!("Parsed {} commands", commands.len()));

        // Generate segments
        let segments = match self.parser.generate_segments(&commands) {
            Ok(s) => s,
            Err(e) => {
                self.status_message = format!("Segment generation error: {}", e);
                self.console.error(format!("Segment generation failed: {}", e));
                tracing::error!("Failed to generate segments: {}", e);
                return;
            }
        };

        let segment_count = segments.len();
        self.console.info(format!("Generated {} segments", segment_count));
        tracing::info!("Parsed {} segments", segment_count);
        
        // Apply preprocessing
        let processed = match self.preprocessor.process(&segments) {
            Ok(p) => p,
            Err(e) => {
                self.status_message = format!("Preprocessing error: {}", e);
                self.console.error(format!("Preprocessing failed: {}", e));
                tracing::error!("Failed to preprocess segments: {}", e);
                return;
            }
        };
        
        let processed_count = processed.len();
        self.console.info(format!("Preprocessed to {} segments", processed_count));
        tracing::info!("Preprocessed to {} segments", processed_count);
        
        // Update program state with the parsed data
        let mut program = self.app_state.program.write();
        program.total_lines = self.gcode_content.lines().count();
        
        self.status_message = format!(
            "Parsed {} segments ({} after preprocessing)",
            segment_count, processed_count
        );
        self.console.info("G-Code parsing complete".to_string());
    }

    /// Handle console command submission
    fn handle_console_command(&mut self, command: &str) {
        let cmd = command.trim();
        
        if cmd.is_empty() {
            return;
        }
        
        // Log the command response (simulate sending to GRBL)
        self.console.info(format!("Sending command: {}", cmd));
        
        // TODO: Send command to GRBL via connection manager
        // For now, just simulate a response
        self.console.received("ok".to_string());
        
        tracing::info!("Console command: {}", cmd);
    }
}

impl eframe::App for RCandleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard shortcuts
        ctx.input(|i| {
            // Ctrl+F to open find dialog
            if i.modifiers.command && i.key_pressed(egui::Key::F) {
                self.gcode_editor.toggle_find_replace();
            }
            // Ctrl+O to open file
            if i.modifiers.command && i.key_pressed(egui::Key::O) {
                self.open_file();
            }
            // Ctrl+S to save file
            if i.modifiers.command && i.key_pressed(egui::Key::S) {
                self.save_file();
            }
        });
        
        // Top panel with menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("📂 Open G-Code...").clicked() {
                        self.open_file();
                        ui.close_menu();
                    }
                    if ui.button("💾 Save").clicked() {
                        self.save_file();
                        ui.close_menu();
                    }
                    if ui.button("💾 Save As...").clicked() {
                        self.save_file_as();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("🚪 Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                ui.menu_button("Connection", |ui| {
                    if ui.button("🔌 Connect").clicked() {
                        self.status_message = "Connecting... (TODO)".to_string();
                        ui.close_menu();
                    }
                    if ui.button("⏸ Disconnect").clicked() {
                        self.status_message = "Disconnected".to_string();
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Edit", |ui| {
                    if ui.button("🔍 Find... (Ctrl+F)").clicked() {
                        self.gcode_editor.toggle_find_replace();
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("View", |ui| {
                    if ui.button("🎥 Reset Camera").clicked() {
                        self.status_message = "Camera reset (TODO)".to_string();
                        ui.close_menu();
                    }
                    if ui.button("🔍 Zoom to Fit").clicked() {
                        self.status_message = "Zoom to fit (TODO)".to_string();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.checkbox(&mut self.show_console, "📟 Show Console").clicked() {
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Help", |ui| {
                    if ui.button("ℹ About").clicked() {
                        self.status_message = format!("rCandle v{}", crate::VERSION);
                        ui.close_menu();
                    }
                    if ui.button("📖 Documentation").clicked() {
                        self.status_message = "Opening documentation... (TODO)".to_string();
                        ui.close_menu();
                    }
                });
            });
        });

        // Bottom status bar
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.label(&self.status_message);
                ui.separator();
                
                // Display current file
                if let Some(path) = &self.current_file {
                    ui.label(format!("📄 {}", path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown")));
                    ui.separator();
                }
                
                ui.label(format!("Units: {}", 
                    if self.settings.general.units_metric { "mm" } else { "inch" }));
                
                // Connection indicator
                ui.separator();
                let connected = self.app_state.is_connected();
                let color = if connected { 
                    egui::Color32::GREEN 
                } else { 
                    egui::Color32::RED 
                };
                ui.colored_label(color, if connected { "🟢 Connected" } else { "🔴 Disconnected" });
            });
        });

        // Left panel - controls
        egui::SidePanel::left("left_panel")
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.heading("Control Panel");
                ui.separator();
                
                // Connection section
                ui.group(|ui| {
                    ui.label("Connection");
                    ui.horizontal(|ui| {
                        if ui.button("Connect").clicked() {
                            self.status_message = "Connecting...".to_string();
                        }
                        if ui.button("Disconnect").clicked() {
                            self.status_message = "Disconnected".to_string();
                        }
                    });
                });
                
                ui.add_space(10.0);
                
                // Machine state section
                ui.group(|ui| {
                    ui.label("Machine State");
                    let machine_state = self.app_state.machine.read();
                    ui.label(format!("Status: {:?}", machine_state.status));
                    ui.label(format!("Position: X:{:.3} Y:{:.3} Z:{:.3}", 
                        machine_state.machine_position.x,
                        machine_state.machine_position.y,
                        machine_state.machine_position.z));
                });
                
                ui.add_space(10.0);
                
                // Jog controls
                ui.group(|ui| {
                    ui.label("Jog Controls");
                    ui.label("(Not implemented yet)");
                });
                
                ui.add_space(10.0);
                
                // Spindle controls
                ui.group(|ui| {
                    ui.label("Spindle");
                    ui.horizontal(|ui| {
                        if ui.button("On").clicked() {
                            self.status_message = "Spindle on".to_string();
                        }
                        if ui.button("Off").clicked() {
                            self.status_message = "Spindle off".to_string();
                        }
                    });
                });
            });

        // Right panel - G-Code editor/viewer
        egui::SidePanel::right("right_panel")
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.heading("G-Code");
                ui.separator();
                
                // Use the custom GCodeEditor widget
                self.gcode_editor.show(ui, &mut self.gcode_content);
            });

        // Console panel (bottom, before central panel)
        if self.show_console {
            egui::TopBottomPanel::bottom("console_panel")
                .default_height(200.0)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.heading("Console");
                    ui.separator();
                    
                    // Show console widget and handle command submission
                    if let Some(command) = self.console.show(ui) {
                        // Handle command submission
                        self.handle_console_command(&command);
                    }
                });
        }

        // Central panel - 3D viewport
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("3D Viewport");
            
            // Placeholder for 3D rendering
            let available_size = ui.available_size();
            let (rect, _response) = ui.allocate_exact_size(
                available_size,
                egui::Sense::click_and_drag()
            );
            
            ui.painter().rect_filled(
                rect,
                0.0,
                egui::Color32::from_rgb(30, 30, 40)
            );
            
            // Draw placeholder text
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "3D Viewport\n(Rendering not implemented yet)",
                egui::FontId::proportional(20.0),
                egui::Color32::from_rgb(200, 200, 200),
            );
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // Save settings to default location
        if let Err(e) = self.settings.save_default() {
            tracing::error!("Failed to save settings: {}", e);
        }
    }
}
