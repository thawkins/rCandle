//! rCandle - GRBL Controller Application
//!
//! A Rust-based GRBL controller with G-Code visualization.

use rcandle::{
    ui::RCandleApp,
    utils::init_logging,
};

fn main() -> anyhow::Result<()> {
    // Initialize logging
    let log_dir = directories::ProjectDirs::from("", "", "rCandle")
        .map(|d| d.data_dir().join("logs"));
    init_logging(log_dir)?;

    tracing::info!("rCandle v{} starting...", rcandle::VERSION);

    // Configure and run the egui application
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("rCandle - GRBL Controller")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_active(true)
            .with_visible(true),
        ..Default::default()
    };

    tracing::info!("Launching UI...");
    
    eframe::run_native(
        "rCandle",
        native_options,
        Box::new(|cc| Box::new(RCandleApp::new(cc))),
    ).map_err(|e| anyhow::anyhow!("Failed to run eframe: {}", e))?;

    tracing::info!("rCandle shutting down");
    Ok(())
}
