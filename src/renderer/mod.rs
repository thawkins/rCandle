//! Renderer module
//!
//! Provides 3D visualization of G-Code toolpaths using WGPU.
//! This module handles:
//! - Toolpath rendering as 3D lines
//! - Camera control (pan, zoom, rotate)
//! - Grid rendering
//! - Coordinate system axes
//! - Machine visualization

mod camera;
mod grid;
mod renderer;
mod toolpath;
mod view_presets;

pub use camera::{Camera, CameraController};
pub use renderer::Renderer;
pub use toolpath::ToolpathRenderer;
pub use view_presets::{ViewPreset, calculate_view_distance, calculate_center};
