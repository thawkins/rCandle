//! Common types used throughout the parser module

use std::fmt;

/// Represents a 3D point in space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    /// Create a new 3D point
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Create a point at the origin
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Calculate distance to another point
    pub fn distance_to(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.3}, {:.3}, {:.3})", self.x, self.y, self.z)
    }
}

/// Units of measurement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Units {
    /// Metric (millimeters)
    Metric,
    /// Imperial (inches)
    Imperial,
}

/// Positioning mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositioningMode {
    /// Absolute positioning (G90)
    Absolute,
    /// Relative/Incremental positioning (G91)
    Relative,
}

/// Arc direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArcDirection {
    /// Clockwise (G2)
    Clockwise,
    /// Counter-clockwise (G3)
    CounterClockwise,
}

/// Plane selection for arcs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Plane {
    /// XY plane (G17)
    XY,
    /// XZ plane (G18)
    XZ,
    /// YZ plane (G19)
    YZ,
}

/// Feed rate mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeedRateMode {
    /// Units per minute (G94)
    UnitsPerMinute,
    /// Inverse time mode (G93)
    InverseTime,
}

/// Spindle state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpindleState {
    /// Spindle off (M5)
    Off,
    /// Clockwise rotation (M3)
    Clockwise,
    /// Counter-clockwise rotation (M4)
    CounterClockwise,
}

/// Coolant state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoolantState {
    /// All coolant off (M9)
    Off,
    /// Mist coolant on (M7)
    Mist,
    /// Flood coolant on (M8)
    Flood,
    /// Both mist and flood on
    Both,
}

/// Work coordinate system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinateSystem {
    /// G54 coordinate system
    G54,
    /// G55 coordinate system
    G55,
    /// G56 coordinate system
    G56,
    /// G57 coordinate system
    G57,
    /// G58 coordinate system
    G58,
    /// G59 coordinate system
    G59,
}

impl Default for CoordinateSystem {
    fn default() -> Self {
        Self::G54
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point3d_distance() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(3.0, 4.0, 0.0);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }

    #[test]
    fn test_point3d_display() {
        let p = Point3D::new(1.234, 5.678, 9.012);
        assert_eq!(format!("{}", p), "(1.234, 5.678, 9.012)");
    }
}
