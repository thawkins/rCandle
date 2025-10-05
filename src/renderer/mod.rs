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

pub use camera::{Camera, CameraController};
pub use renderer::Renderer;
pub use toolpath::ToolpathRenderer;
