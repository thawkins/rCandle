//! Motion segment definitions
//!
//! This module defines the different types of motion segments that can be
//! generated from parsed G-Code commands.

pub use super::types::{ArcDirection, Point3D};

/// Type of motion segment
#[derive(Debug, Clone, PartialEq)]
pub enum SegmentType {
    /// Rapid positioning (G0)
    Rapid,
    /// Linear interpolation (G1)
    Linear,
    /// Clockwise arc (G2)
    ArcCW,
    /// Counter-clockwise arc (G3)
    ArcCCW,
}

/// Represents a single motion segment
#[derive(Debug, Clone)]
pub struct Segment {
    /// Type of segment
    pub segment_type: SegmentType,
    /// Starting point
    pub start: Point3D,
    /// Ending point
    pub end: Point3D,
    /// Center point (for arcs only)
    pub center: Option<Point3D>,
    /// Feed rate (units per minute)
    pub feed_rate: f64,
    /// Spindle speed (RPM)
    pub spindle_speed: f64,
    /// Line number in original G-Code (if available)
    pub line_number: Option<u32>,
}

impl Segment {
    /// Create a new rapid positioning segment
    pub fn rapid(start: Point3D, end: Point3D) -> Self {
        Self {
            segment_type: SegmentType::Rapid,
            start,
            end,
            center: None,
            feed_rate: 0.0,
            spindle_speed: 0.0,
            line_number: None,
        }
    }

    /// Create a new linear interpolation segment
    pub fn linear(start: Point3D, end: Point3D, feed_rate: f64) -> Self {
        Self {
            segment_type: SegmentType::Linear,
            start,
            end,
            center: None,
            feed_rate,
            spindle_speed: 0.0,
            line_number: None,
        }
    }

    /// Create a new arc segment
    pub fn arc(
        start: Point3D,
        end: Point3D,
        center: Point3D,
        direction: ArcDirection,
        feed_rate: f64,
    ) -> Self {
        let segment_type = match direction {
            ArcDirection::Clockwise => SegmentType::ArcCW,
            ArcDirection::CounterClockwise => SegmentType::ArcCCW,
        };

        Self {
            segment_type,
            start,
            end,
            center: Some(center),
            feed_rate,
            spindle_speed: 0.0,
            line_number: None,
        }
    }

    /// Get the length of this segment
    pub fn length(&self) -> f64 {
        match self.segment_type {
            SegmentType::Rapid | SegmentType::Linear => self.start.distance_to(&self.end),
            SegmentType::ArcCW | SegmentType::ArcCCW => {
                if let Some(center) = self.center {
                    self.arc_length(center)
                } else {
                    self.start.distance_to(&self.end)
                }
            }
        }
    }

    /// Calculate arc length
    fn arc_length(&self, center: Point3D) -> f64 {
        let radius = self.start.distance_to(&center);
        
        // Calculate angle swept by the arc
        let start_angle = (self.start.y - center.y).atan2(self.start.x - center.x);
        let end_angle = (self.end.y - center.y).atan2(self.end.x - center.x);
        
        let mut angle = end_angle - start_angle;
        
        // Adjust angle based on direction
        match self.segment_type {
            SegmentType::ArcCW => {
                if angle > 0.0 {
                    angle -= 2.0 * std::f64::consts::PI;
                }
                angle = -angle;
            }
            SegmentType::ArcCCW => {
                if angle < 0.0 {
                    angle += 2.0 * std::f64::consts::PI;
                }
            }
            _ => {}
        }
        
        radius * angle
    }

    /// Estimate time to complete this segment (in seconds)
    pub fn estimated_time(&self) -> f64 {
        if self.feed_rate <= 0.0 {
            return 0.0;
        }
        
        let length = self.length();
        // Convert from units per minute to units per second
        length / (self.feed_rate / 60.0)
    }

    /// Check if this is a cutting move (non-rapid)
    pub fn is_cutting(&self) -> bool {
        !matches!(self.segment_type, SegmentType::Rapid)
    }

    /// Set the line number
    pub fn with_line_number(mut self, line_number: u32) -> Self {
        self.line_number = Some(line_number);
        self
    }

    /// Set the spindle speed
    pub fn with_spindle_speed(mut self, speed: f64) -> Self {
        self.spindle_speed = speed;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::types::ArcDirection;

    #[test]
    fn test_rapid_segment() {
        let start = Point3D::new(0.0, 0.0, 0.0);
        let end = Point3D::new(10.0, 0.0, 0.0);
        let seg = Segment::rapid(start, end);

        assert_eq!(seg.segment_type, SegmentType::Rapid);
        assert_eq!(seg.length(), 10.0);
        assert!(!seg.is_cutting());
    }

    #[test]
    fn test_linear_segment() {
        let start = Point3D::new(0.0, 0.0, 0.0);
        let end = Point3D::new(3.0, 4.0, 0.0);
        let seg = Segment::linear(start, end, 1000.0);

        assert_eq!(seg.segment_type, SegmentType::Linear);
        assert_eq!(seg.length(), 5.0);
        assert!(seg.is_cutting());
        assert_eq!(seg.feed_rate, 1000.0);
    }

    #[test]
    fn test_estimated_time() {
        let start = Point3D::new(0.0, 0.0, 0.0);
        let end = Point3D::new(100.0, 0.0, 0.0);
        let seg = Segment::linear(start, end, 1000.0); // 1000 units/min

        // 100 units at 1000 units/min = 0.1 min = 6 seconds
        assert!((seg.estimated_time() - 6.0).abs() < 0.01);
    }

    #[test]
    fn test_arc_segment() {
        let start = Point3D::new(10.0, 0.0, 0.0);
        let end = Point3D::new(0.0, 10.0, 0.0);
        let center = Point3D::new(0.0, 0.0, 0.0);
        let seg = Segment::arc(start, end, center, ArcDirection::CounterClockwise, 1000.0);

        assert_eq!(seg.segment_type, SegmentType::ArcCCW);
        assert!(seg.is_cutting());
        
        // Arc length for a 90-degree arc with radius 10
        let expected_length = 10.0 * std::f64::consts::PI / 2.0;
        assert!((seg.length() - expected_length).abs() < 0.1);
    }

    #[test]
    fn test_segment_with_line_number() {
        let start = Point3D::new(0.0, 0.0, 0.0);
        let end = Point3D::new(10.0, 0.0, 0.0);
        let seg = Segment::rapid(start, end).with_line_number(42);

        assert_eq!(seg.line_number, Some(42));
    }
}
