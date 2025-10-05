//! rCandle - GRBL Controller Application
//!
//! A Rust-based GRBL controller with G-Code visualization.

use rcandle::{
    settings::Settings,
    state::AppState,
    utils::init_logging,
};

fn main() -> anyhow::Result<()> {
    // Initialize logging
    let log_dir = directories::ProjectDirs::from("", "", "rCandle")
        .map(|d| d.data_dir().join("logs"));
    init_logging(log_dir)?;

    tracing::info!("rCandle v{} starting...", rcandle::VERSION);

    // Load settings
    let settings = Settings::load_or_default();
    tracing::info!("Settings loaded from: {:?}", Settings::default_config_path()?);

    // Initialize application state
    let _app_state = AppState::new();
    tracing::info!("Application state initialized");

    tracing::info!("rCandle initialized successfully");
    tracing::info!("Units: {}", if settings.general.units_metric { "Metric (mm)" } else { "Imperial (inches)" });
    tracing::info!("Baud rate: {}", settings.connection.baud_rate);

    println!("rCandle v{}", rcandle::VERSION);
    println!("Application initialized. GUI will be implemented in Phase 6.");
    println!("Press Ctrl+C to exit.");

    // TODO: Start UI event loop (Phase 6)
    // For now, just wait
    std::thread::sleep(std::time::Duration::from_secs(2));

    Ok(())
}
