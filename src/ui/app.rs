//! Main application structure for rCandle

use crate::{
    settings::Settings,
    state::AppState,
};

/// Main rCandle application state
pub struct RCandleApp {
    /// Application settings
    settings: Settings,
    /// Application state (machine, program, etc.)
    app_state: AppState,
    /// Connection status display
    status_message: String,
}

impl RCandleApp {
    /// Create a new rCandle application instance
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Load settings
        let settings = Settings::load_or_default();
        
        // Initialize application state
        let app_state = AppState::new();
        
        tracing::info!("rCandle UI initialized");
        
        Self {
            settings,
            app_state,
            status_message: "Ready".to_string(),
        }
    }
}

impl eframe::App for RCandleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel with menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open G-Code...").clicked() {
                        self.status_message = "Open file dialog (TODO)".to_string();
                        ui.close_menu();
                    }
                    if ui.button("Save...").clicked() {
                        self.status_message = "Save dialog (TODO)".to_string();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                ui.menu_button("Connection", |ui| {
                    if ui.button("Connect").clicked() {
                        self.status_message = "Connecting... (TODO)".to_string();
                        ui.close_menu();
                    }
                    if ui.button("Disconnect").clicked() {
                        self.status_message = "Disconnected".to_string();
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("View", |ui| {
                    if ui.button("Reset Camera").clicked() {
                        self.status_message = "Camera reset (TODO)".to_string();
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.status_message = format!("rCandle v{}", crate::VERSION);
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
                ui.label(format!("Units: {}", 
                    if self.settings.general.units_metric { "mm" } else { "inch" }));
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
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("No G-Code loaded");
                    ui.label("");
                    ui.label("Use File -> Open to load a G-Code file");
                });
            });

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
