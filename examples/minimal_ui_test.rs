//! Minimal egui test to verify UI interaction works
//!
//! This is a simple test program to verify that egui button clicks
//! and other interactions work correctly with the current setup.
//!
//! Run with: cargo run --example minimal_ui_test

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Enable logging
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Minimal UI Test")
            .with_inner_size([400.0, 300.0])
            .with_focused(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "minimal_ui_test",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    counter: i32,
    text: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Minimal egui Test");
            
            ui.separator();
            
            ui.label(format!("Counter: {}", self.counter));
            
            if ui.button("Increment Counter").clicked() {
                self.counter += 1;
                println!("Button clicked! Counter is now: {}", self.counter);
            }
            
            ui.separator();
            
            ui.label("Type something:");
            ui.text_edit_singleline(&mut self.text);
            
            ui.separator();
            
            if ui.button("Reset").clicked() {
                self.counter = 0;
                self.text.clear();
                println!("Reset clicked!");
            }
            
            ui.separator();
            ui.label("If you can click buttons and type text, egui is working!");
        });
    }
}
