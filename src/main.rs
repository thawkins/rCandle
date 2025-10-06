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

    // Create a Tokio runtime that will be available throughout the application
    let runtime = tokio::runtime::Runtime::new()?;
    let _guard = runtime.enter();

    // Configure and run the egui application
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(&format!("rCandle v{} - GRBL Controller", rcandle::VERSION))
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_active(true)
            .with_visible(true)
            .with_decorations(true)
            .with_resizable(true),
        ..Default::default()
    };

    tracing::info!("Launching UI...");
    
    // eframe 0.28 changed the signature - now returns Result directly
    // The _guard keeps us in the runtime context for the entire duration
    eframe::run_native(
        "rCandle",
        native_options,
        Box::new(|cc| Ok(Box::new(RCandleApp::new(cc)))),
    ).map_err(|e| anyhow::anyhow!("Failed to run eframe: {}", e))?;

    tracing::info!("rCandle shutting down");
    Ok(())
}
