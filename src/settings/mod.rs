//! Settings module for rCandle
//!
//! Handles application configuration and settings persistence.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::utils::{Error, Result};

/// Main application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// General application settings
    pub general: GeneralSettings,
    
    /// Connection settings
    pub connection: ConnectionSettings,
    
    /// Visualization settings
    pub visualization: VisualizationSettings,
    
    /// Jog settings
    pub jog: JogSettings,
    
    /// User interface settings
    pub ui: UiSettings,
}

/// General application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    /// Units: true for metric (mm), false for imperial (inches)
    pub units_metric: bool,
    
    /// Arc precision (degrees per segment)
    pub arc_precision: f64,
    
    /// Line segments per arc
    pub arc_segments: u32,
    
    /// Z-axis safety height for rapid moves
    pub safe_z: f64,
    
    /// Startup commands to send to GRBL
    pub startup_commands: Vec<String>,
}

/// Connection settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionSettings {
    /// Serial port name (e.g., "COM3" or "/dev/ttyUSB0")
    pub port_name: String,
    
    /// Baud rate
    pub baud_rate: u32,
    
    /// Connection timeout in milliseconds
    pub timeout_ms: u64,
    
    /// Command timeout in milliseconds
    pub command_timeout_ms: u64,
    
    /// Status query interval in milliseconds
    pub status_query_interval_ms: u64,
    
    /// Auto-connect on startup
    pub auto_connect: bool,
}

/// Visualization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationSettings {
    /// Show grid
    pub show_grid: bool,
    
    /// Grid size
    pub grid_size: f32,
    
    /// Show tool position
    pub show_tool: bool,
    
    /// Show origin
    pub show_origin: bool,
    
    /// Show machine bounds
    pub show_bounds: bool,
    
    /// Anti-aliasing sample count (1, 2, 4, 8, or 16)
    pub msaa_samples: u32,
    
    /// VSYNC enabled
    pub vsync: bool,
    
    /// Field of view in degrees
    pub fov: f32,
    
    /// Camera movement speed
    pub camera_speed: f32,
    
    /// Color scheme
    pub color_scheme: ColorScheme,
}

/// Color scheme for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    /// Background color [R, G, B, A]
    pub background: [f32; 4],
    
    /// Grid color [R, G, B, A]
    pub grid: [f32; 4],
    
    /// Tool path color [R, G, B, A]
    pub toolpath: [f32; 4],
    
    /// Rapid move color [R, G, B, A]
    pub rapid: [f32; 4],
    
    /// Tool position color [R, G, B, A]
    pub tool: [f32; 4],
    
    /// Origin marker color [R, G, B, A]
    pub origin: [f32; 4],
    
    /// Machine bounds color [R, G, B, A]
    pub bounds: [f32; 4],
}

/// Jog settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JogSettings {
    /// XY jog feed rate (mm/min or in/min)
    pub xy_feed_rate: f64,
    
    /// Z jog feed rate (mm/min or in/min)
    pub z_feed_rate: f64,
    
    /// Step sizes for jog buttons
    pub step_sizes: Vec<f64>,
    
    /// Default step size index
    pub default_step_index: usize,
    
    /// Enable continuous jog mode
    pub continuous_mode: bool,
}

/// UI settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    /// Window width
    pub window_width: u32,
    
    /// Window height
    pub window_height: u32,
    
    /// Window maximized
    pub window_maximized: bool,
    
    /// Dark mode
    pub dark_mode: bool,
    
    /// Font size
    pub font_size: f32,
    
    /// Show console panel
    pub show_console: bool,
    
    /// Show state panel
    pub show_state: bool,
    
    /// Show control panel
    pub show_control: bool,
    
    /// Console history limit
    pub console_history_limit: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            general: GeneralSettings::default(),
            connection: ConnectionSettings::default(),
            visualization: VisualizationSettings::default(),
            jog: JogSettings::default(),
            ui: UiSettings::default(),
        }
    }
}

impl Default for GeneralSettings {
    fn default() -> Self {
        GeneralSettings {
            units_metric: true,
            arc_precision: 1.0,
            arc_segments: 20,
            safe_z: 5.0,
            startup_commands: vec![],
        }
    }
}

impl Default for ConnectionSettings {
    fn default() -> Self {
        ConnectionSettings {
            port_name: String::new(),
            baud_rate: 115200,
            timeout_ms: 5000,
            command_timeout_ms: 10000,
            status_query_interval_ms: 250,
            auto_connect: false,
        }
    }
}

impl Default for VisualizationSettings {
    fn default() -> Self {
        VisualizationSettings {
            show_grid: true,
            grid_size: 10.0,
            show_tool: true,
            show_origin: true,
            show_bounds: true,
            msaa_samples: 4,
            vsync: true,
            fov: 60.0,
            camera_speed: 1.0,
            color_scheme: ColorScheme::default(),
        }
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        ColorScheme {
            background: [0.1, 0.1, 0.1, 1.0],
            grid: [0.3, 0.3, 0.3, 1.0],
            toolpath: [0.0, 1.0, 0.0, 1.0],
            rapid: [1.0, 0.0, 0.0, 1.0],
            tool: [1.0, 1.0, 0.0, 1.0],
            origin: [1.0, 1.0, 1.0, 1.0],
            bounds: [0.5, 0.5, 0.5, 1.0],
        }
    }
}

impl Default for JogSettings {
    fn default() -> Self {
        JogSettings {
            xy_feed_rate: 1000.0,
            z_feed_rate: 500.0,
            step_sizes: vec![0.1, 1.0, 10.0, 100.0],
            default_step_index: 1,
            continuous_mode: false,
        }
    }
}

impl Default for UiSettings {
    fn default() -> Self {
        UiSettings {
            window_width: 1280,
            window_height: 720,
            window_maximized: false,
            dark_mode: true,
            font_size: 14.0,
            show_console: true,
            show_state: true,
            show_control: true,
            console_history_limit: 1000,
        }
    }
}

impl Settings {
    /// Load settings from a TOML file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let settings: Settings = toml::from_str(&contents)
            .map_err(|e| Error::config(format!("Failed to parse settings: {}", e)))?;
        Ok(settings)
    }

    /// Save settings to a TOML file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let contents = toml::to_string_pretty(self)
            .map_err(|e| Error::config(format!("Failed to serialize settings: {}", e)))?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    /// Get the default config path for the application
    pub fn default_config_path() -> Result<PathBuf> {
        let dirs = directories::ProjectDirs::from("", "", "rCandle")
            .ok_or_else(|| Error::config("Failed to determine config directory"))?;
        
        let config_dir = dirs.config_dir();
        std::fs::create_dir_all(config_dir)?;
        
        Ok(config_dir.join("config.toml"))
    }

    /// Load settings from default location, or create default settings
    pub fn load_or_default() -> Self {
        match Self::default_config_path() {
            Ok(path) => {
                if path.exists() {
                    Self::load(&path).unwrap_or_default()
                } else {
                    let settings = Self::default();
                    let _ = settings.save(&path); // Ignore error
                    settings
                }
            }
            Err(_) => Self::default(),
        }
    }

    /// Save to default location
    pub fn save_default(&self) -> Result<()> {
        let path = Self::default_config_path()?;
        self.save(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();
        assert!(settings.general.units_metric);
        assert_eq!(settings.connection.baud_rate, 115200);
    }

    #[test]
    fn test_settings_serialization() {
        let settings = Settings::default();
        let toml_str = toml::to_string(&settings).expect("Failed to serialize");
        let deserialized: Settings = toml::from_str(&toml_str).expect("Failed to deserialize");
        
        assert_eq!(settings.general.units_metric, deserialized.general.units_metric);
        assert_eq!(settings.connection.baud_rate, deserialized.connection.baud_rate);
    }
}
