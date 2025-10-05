//! Main application structure for rCandle

use crate::{
    parser::{Parser, Preprocessor, Segment, SegmentType, Tokenizer},
    renderer::Renderer,
    settings::Settings,
    state::AppState,
    ui::widgets::{Console, GCodeEditor},
};
use std::{path::PathBuf, sync::Arc};

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
    /// 3D renderer (optional until WGPU is initialized)
    renderer: Option<Renderer>,
    /// Parsed segments for rendering
    segments: Vec<Segment>,
}

impl RCandleApp {
    /// Create a new rCandle application instance
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
        
        // Initialize WGPU renderer
        let renderer = Self::init_renderer(cc);
        
        if renderer.is_some() {
            console.info("3D renderer initialized".to_string());
        } else {
            console.warning("Failed to initialize 3D renderer".to_string());
        }
        
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
            renderer,
            segments: Vec::new(),
        }
    }

    /// Initialize WGPU renderer
    fn init_renderer(cc: &eframe::CreationContext<'_>) -> Option<Renderer> {
        // Get WGPU render state from eframe
        let wgpu_render_state = cc.wgpu_render_state.as_ref()?;
        
        let device = wgpu_render_state.device.clone();
        let queue = wgpu_render_state.queue.clone();
        let target_format = wgpu_render_state.target_format;
        
        Some(Renderer::new(device, queue, target_format))
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
        
        // Store segments for rendering
        self.segments = processed.clone();
        
        // Update renderer with new toolpath
        if let Some(ref mut renderer) = self.renderer {
            renderer.set_segments(processed);
            self.console.info("3D view updated with toolpath".to_string());
        }
        
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

    /// Draw toolpath in 2D (XY plane projection)
    fn draw_toolpath_2d(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        use egui::{Color32, Pos2, Stroke};
        
        if self.segments.is_empty() {
            return;
        }
        
        // Calculate bounding box
        let mut min_x = f64::MAX;
        let mut max_x = f64::MIN;
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;
        
        for segment in &self.segments {
            min_x = min_x.min(segment.start.x).min(segment.end.x);
            max_x = max_x.max(segment.start.x).max(segment.end.x);
            min_y = min_y.min(segment.start.y).min(segment.end.y);
            max_y = max_y.max(segment.start.y).max(segment.end.y);
        }
        
        // Add some padding
        let padding = 20.0;
        let width = (max_x - min_x) as f32;
        let height = (max_y - min_y) as f32;
        
        if width == 0.0 || height == 0.0 {
            return;
        }
        
        // Calculate scale to fit in viewport
        let viewport_width = rect.width() - padding * 2.0;
        let viewport_height = rect.height() - padding * 2.0;
        let scale = (viewport_width / width).min(viewport_height / height);
        
        // Center offset
        let offset_x = rect.left() + padding + (viewport_width - width * scale) / 2.0;
        let offset_y = rect.top() + padding + (viewport_height - height * scale) / 2.0;
        
        // Transform function from G-Code coordinates to screen coordinates
        let to_screen = |x: f64, y: f64| {
            Pos2::new(
                offset_x + ((x - min_x) as f32 * scale),
                // Flip Y axis (G-Code Y increases upward, screen Y increases downward)
                offset_y + viewport_height - ((y - min_y) as f32 * scale),
            )
        };
        
        // Draw grid
        let grid_color = Color32::from_rgb(40, 40, 50);
        let grid_spacing = 10.0; // mm
        
        // Vertical grid lines
        let mut x = (min_x / grid_spacing).floor() * grid_spacing;
        while x <= max_x {
            let p1 = to_screen(x, min_y);
            let p2 = to_screen(x, max_y);
            ui.painter().line_segment([p1, p2], Stroke::new(1.0, grid_color));
            x += grid_spacing;
        }
        
        // Horizontal grid lines
        let mut y = (min_y / grid_spacing).floor() * grid_spacing;
        while y <= max_y {
            let p1 = to_screen(min_x, y);
            let p2 = to_screen(max_x, y);
            ui.painter().line_segment([p1, p2], Stroke::new(1.0, grid_color));
            y += grid_spacing;
        }
        
        // Draw axes
        let origin = to_screen(0.0, 0.0);
        if min_x <= 0.0 && max_x >= 0.0 && min_y <= 0.0 && max_y >= 0.0 {
            // X axis (red)
            let x_end = to_screen(max_x, 0.0);
            ui.painter().line_segment(
                [origin, x_end],
                Stroke::new(2.0, Color32::from_rgb(200, 50, 50)),
            );
            
            // Y axis (green)
            let y_end = to_screen(0.0, max_y);
            ui.painter().line_segment(
                [origin, y_end],
                Stroke::new(2.0, Color32::from_rgb(50, 200, 50)),
            );
        }
        
        // Draw toolpath segments
        for segment in &self.segments {
            let start = to_screen(segment.start.x, segment.start.y);
            let end = to_screen(segment.end.x, segment.end.y);
            
            // Color based on segment type
            let (color, width) = match segment.segment_type {
                SegmentType::Rapid => (Color32::from_rgb(255, 100, 100), 1.0), // Red for rapids
                SegmentType::Linear => (Color32::from_rgb(100, 255, 100), 2.0), // Green for cuts
                SegmentType::ArcCW | SegmentType::ArcCCW => {
                    (Color32::from_rgb(100, 150, 255), 2.0) // Blue for arcs
                }
            };
            
            ui.painter().line_segment([start, end], Stroke::new(width, color));
        }
        
        // Draw start point marker
        if let Some(first) = self.segments.first() {
            let start = to_screen(first.start.x, first.start.y);
            ui.painter().circle_filled(start, 4.0, Color32::from_rgb(100, 255, 255));
            ui.painter().circle_stroke(start, 4.0, Stroke::new(1.0, Color32::WHITE));
        }
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
                        if let Some(ref mut renderer) = self.renderer {
                            renderer.reset_camera();
                            self.status_message = "Camera reset".to_string();
                            self.console.info("Camera reset to default view".to_string());
                        }
                        ui.close_menu();
                    }
                    if ui.button("🔍 Zoom to Fit").clicked() {
                        if let Some(ref mut renderer) = self.renderer {
                            renderer.zoom_to_fit();
                            self.status_message = "Zoomed to fit".to_string();
                            self.console.info("Camera zoomed to fit toolpath".to_string());
                        }
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
            ui.heading("Toolpath Viewer");
            
            let available_size = ui.available_size();
            let (rect, response) = ui.allocate_exact_size(
                available_size,
                egui::Sense::click_and_drag()
            );
            
            // Draw background
            ui.painter().rect_filled(
                rect,
                0.0,
                egui::Color32::from_rgb(25, 25, 35)
            );
            
            // Draw toolpath if we have segments
            if !self.segments.is_empty() {
                self.draw_toolpath_2d(ui, rect);
            } else {
                // Show placeholder text
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "Load a G-Code file to view toolpath\n(File > Open G-Code...)",
                    egui::FontId::proportional(18.0),
                    egui::Color32::from_rgb(150, 150, 150),
                );
            }
            
            // Show instructions in corner
            if !self.segments.is_empty() {
                let instructions = format!(
                    "Segments: {} | Use View menu for camera controls",
                    self.segments.len()
                );
                ui.painter().text(
                    rect.left_top() + egui::vec2(10.0, 10.0),
                    egui::Align2::LEFT_TOP,
                    instructions,
                    egui::FontId::monospace(12.0),
                    egui::Color32::from_rgb(180, 180, 180),
                );
            }
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // Save settings to default location
        if let Err(e) = self.settings.save_default() {
            tracing::error!("Failed to save settings: {}", e);
        }
    }
}
