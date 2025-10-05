//! View presets for camera
//!
//! Provides predefined camera positions for common viewing angles.

use nalgebra as na;
use super::Camera;

/// Predefined view presets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewPreset {
    /// Isometric view (default)
    Isometric,
    /// Top view (looking down Z axis)
    Top,
    /// Front view (looking along Y axis)
    Front,
    /// Right side view (looking along X axis)
    Right,
    /// Left side view (looking along -X axis)
    Left,
    /// Back view (looking along -Y axis)
    Back,
    /// Bottom view (looking up Z axis)
    Bottom,
}

impl ViewPreset {
    /// Apply this preset to a camera
    pub fn apply(&self, camera: &mut Camera, center: na::Point3<f32>, distance: f32) {
        camera.target = center;
        
        let position = match self {
            ViewPreset::Isometric => {
                // Standard isometric view: 45° rotation, 35.264° elevation
                let x = distance * 0.707; // cos(45°)
                let y = distance * 0.577; // sin(35.264°)
                let z = distance * 0.707; // cos(45°)
                na::Point3::new(center.x + x, center.y + y, center.z + z)
            },
            ViewPreset::Top => {
                // Looking straight down
                na::Point3::new(center.x, center.y + distance, center.z)
            },
            ViewPreset::Front => {
                // Looking from front (positive Y)
                na::Point3::new(center.x, center.y, center.z + distance)
            },
            ViewPreset::Right => {
                // Looking from right side (positive X)
                na::Point3::new(center.x + distance, center.y, center.z)
            },
            ViewPreset::Left => {
                // Looking from left side (negative X)
                na::Point3::new(center.x - distance, center.y, center.z)
            },
            ViewPreset::Back => {
                // Looking from back (negative Y)
                na::Point3::new(center.x, center.y, center.z - distance)
            },
            ViewPreset::Bottom => {
                // Looking up from below
                na::Point3::new(center.x, center.y - distance, center.z)
            },
        };
        
        camera.position = position;
        
        // Set up vector appropriately for each view
        camera.up = match self {
            ViewPreset::Top | ViewPreset::Isometric => na::Vector3::new(0.0, 0.0, -1.0),
            ViewPreset::Bottom => na::Vector3::new(0.0, 0.0, 1.0),
            _ => na::Vector3::new(0.0, 1.0, 0.0),
        };
    }
    
    /// Get the name of this preset
    pub fn name(&self) -> &'static str {
        match self {
            ViewPreset::Isometric => "Isometric",
            ViewPreset::Top => "Top",
            ViewPreset::Front => "Front",
            ViewPreset::Right => "Right",
            ViewPreset::Left => "Left",
            ViewPreset::Back => "Back",
            ViewPreset::Bottom => "Bottom",
        }
    }
    
    /// Get all available presets
    pub fn all() -> &'static [ViewPreset] {
        &[
            ViewPreset::Isometric,
            ViewPreset::Top,
            ViewPreset::Front,
            ViewPreset::Right,
            ViewPreset::Left,
            ViewPreset::Back,
            ViewPreset::Bottom,
        ]
    }
}

impl std::fmt::Display for ViewPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Calculate reasonable viewing distance for given bounds
pub fn calculate_view_distance(bounds_min: na::Point3<f32>, bounds_max: na::Point3<f32>) -> f32 {
    let diagonal = (bounds_max - bounds_min).norm();
    // Distance should be about 1.5x the diagonal to fit everything in view
    diagonal * 1.5
}

/// Calculate center point of bounds
pub fn calculate_center(bounds_min: na::Point3<f32>, bounds_max: na::Point3<f32>) -> na::Point3<f32> {
    na::Point3::from((bounds_min.coords + bounds_max.coords) * 0.5)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_view_preset_names() {
        assert_eq!(ViewPreset::Isometric.name(), "Isometric");
        assert_eq!(ViewPreset::Top.name(), "Top");
        assert_eq!(ViewPreset::Front.name(), "Front");
    }
    
    #[test]
    fn test_all_presets() {
        let presets = ViewPreset::all();
        assert_eq!(presets.len(), 7);
    }
    
    #[test]
    fn test_calculate_center() {
        let min = na::Point3::new(0.0, 0.0, 0.0);
        let max = na::Point3::new(10.0, 20.0, 30.0);
        let center = calculate_center(min, max);
        
        assert_eq!(center.x, 5.0);
        assert_eq!(center.y, 10.0);
        assert_eq!(center.z, 15.0);
    }
    
    #[test]
    fn test_apply_preset() {
        let mut camera = Camera::new();
        let center = na::Point3::new(0.0, 0.0, 0.0);
        let distance = 10.0;
        
        ViewPreset::Top.apply(&mut camera, center, distance);
        assert_eq!(camera.target, center);
        assert!((camera.position.y - distance).abs() < 0.001);
    }
}
