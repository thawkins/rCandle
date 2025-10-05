//! Logging infrastructure for rCandle
//!
//! Provides tracing-based logging to console and file.

use std::path::PathBuf;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize logging with console and file output
pub fn init_logging(log_dir: Option<PathBuf>) -> anyhow::Result<()> {
    // Create log directory if specified
    if let Some(ref dir) = log_dir {
        std::fs::create_dir_all(dir)?;
    }

    // Configure environment filter (defaults to INFO level)
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,rcandle=debug"));

    // Console layer
    let console_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .compact();

    // File layer (if log directory specified)
    let file_layer = if let Some(dir) = log_dir {
        let log_file = dir.join(format!("rcandle-{}.log", chrono::Local::now().format("%Y%m%d")));
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?;

        Some(
            fmt::layer()
                .with_writer(std::sync::Arc::new(file))
                .with_ansi(false)
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
    } else {
        None
    };

    // Build subscriber
    let registry = tracing_subscriber::registry().with(env_filter).with(console_layer);

    if let Some(file_layer) = file_layer {
        registry.with(file_layer).init();
    } else {
        registry.init();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_logging_no_file() {
        init_logging(None).expect("Failed to initialize logging");
    }
}
