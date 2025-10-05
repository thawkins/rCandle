//! G-Code Preprocessor
//!
//! This module provides preprocessing functionality for segments:
//! - Arc expansion (converting arcs to line segments)
//! - Unit conversion
//! - Optimization (removing unnecessary rapids)

use super::segment::{Segment, SegmentType};
#[cfg(test)]
use super::segment::ArcDirection;
use super::types::{Point3D, Units};
use crate::utils::error::Result;

/// Preprocessor for optimizing and transforming segments
pub struct Preprocessor {
    /// Arc precision (maximum deviation in units)
    arc_precision: f64,
    /// Target units for conversion
    target_units: Units,
}

impl Preprocessor {
    /// Create a new preprocessor with default settings
    pub fn new() -> Self {
        Self {
            arc_precision: 0.1,
            target_units: Units::Metric,
        }
    }

    /// Set arc precision
    pub fn with_arc_precision(mut self, precision: f64) -> Self {
        self.arc_precision = precision;
        self
    }

    /// Set target units
    pub fn with_target_units(mut self, units: Units) -> Self {
        self.target_units = units;
        self
    }

    /// Process a list of segments
    pub fn process(&self, segments: &[Segment]) -> Result<Vec<Segment>> {
        let mut result = Vec::new();

        for segment in segments {
            match segment.segment_type {
                SegmentType::ArcCW | SegmentType::ArcCCW => {
                    // Expand arcs into line segments
                    let expanded = self.expand_arc(segment)?;
                    result.extend(expanded);
                }
                _ => {
                    result.push(segment.clone());
                }
            }
        }

        Ok(result)
    }

    /// Expand an arc into line segments
    fn expand_arc(&self, arc: &Segment) -> Result<Vec<Segment>> {
        let center = match arc.center {
            Some(c) => c,
            None => return Ok(vec![arc.clone()]),
        };

        let radius = arc.start.distance_to(&center);
        
        // Calculate number of segments needed based on precision
        let segments_count = self.calculate_arc_segments(radius);
        
        let mut result = Vec::with_capacity(segments_count);
        
        // Calculate angles
        let start_angle = (arc.start.y - center.y).atan2(arc.start.x - center.x);
        let end_angle = (arc.end.y - center.y).atan2(arc.end.x - center.x);
        
        let mut total_angle = end_angle - start_angle;
        
        // Adjust angle based on direction
        match arc.segment_type {
            SegmentType::ArcCW => {
                if total_angle > 0.0 {
                    total_angle -= 2.0 * std::f64::consts::PI;
                }
            }
            SegmentType::ArcCCW => {
                if total_angle < 0.0 {
                    total_angle += 2.0 * std::f64::consts::PI;
                }
            }
            _ => {}
        }
        
        let angle_step = total_angle / segments_count as f64;
        
        // Calculate Z step for helical arcs
        let z_step = (arc.end.z - arc.start.z) / segments_count as f64;
        
        // Generate line segments
        let mut current_pos = arc.start;
        
        for i in 1..=segments_count {
            let angle = start_angle + angle_step * i as f64;
            let next_pos = Point3D::new(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
                arc.start.z + z_step * i as f64,
            );
            
            let segment = Segment::linear(current_pos, next_pos, arc.feed_rate)
                .with_spindle_speed(arc.spindle_speed);
            
            result.push(segment);
            current_pos = next_pos;
        }
        
        Ok(result)
    }

    /// Calculate number of line segments needed for an arc
    fn calculate_arc_segments(&self, radius: f64) -> usize {
        // Use the formula: n = ceil(2π / acos(1 - precision/radius))
        // This ensures the maximum deviation is less than precision
        
        if radius <= 0.0 {
            return 4; // Minimum segments for safety
        }
        
        let ratio = 1.0 - (self.arc_precision / radius);
        let ratio = ratio.max(-1.0).min(1.0); // Clamp to valid range
        
        let angle = ratio.acos();
        let segments = (2.0 * std::f64::consts::PI / angle).ceil() as usize;
        
        // Ensure reasonable bounds
        segments.max(4).min(360)
    }

    /// Convert units for a segment
    pub fn convert_units(&self, segment: &Segment, from: Units, to: Units) -> Segment {
        if from == to {
            return segment.clone();
        }

        let factor = match (from, to) {
            (Units::Imperial, Units::Metric) => 25.4, // inches to mm
            (Units::Metric, Units::Imperial) => 1.0 / 25.4, // mm to inches
            _ => 1.0,
        };

        let mut converted = segment.clone();
        converted.start = self.scale_point(segment.start, factor);
        converted.end = self.scale_point(segment.end, factor);
        if let Some(center) = segment.center {
            converted.center = Some(self.scale_point(center, factor));
        }
        converted.feed_rate *= factor;

        converted
    }

    /// Scale a point by a factor
    fn scale_point(&self, point: Point3D, factor: f64) -> Point3D {
        Point3D::new(point.x * factor, point.y * factor, point.z * factor)
    }

    /// Remove consecutive rapid moves to the same location
    pub fn optimize_rapids(&self, segments: &[Segment]) -> Vec<Segment> {
        let mut result = Vec::new();
        let mut last_pos: Option<Point3D> = None;

        for segment in segments {
            // Skip rapids that go to the same location
            if matches!(segment.segment_type, SegmentType::Rapid) {
                if let Some(last) = last_pos {
                    if Self::points_equal(segment.end, last) {
                        continue;
                    }
                }
            }

            result.push(segment.clone());
            last_pos = Some(segment.end);
        }

        result
    }

    /// Check if two points are equal (within tolerance)
    fn points_equal(p1: Point3D, p2: Point3D) -> bool {
        const TOLERANCE: f64 = 0.0001;
        (p1.x - p2.x).abs() < TOLERANCE
            && (p1.y - p2.y).abs() < TOLERANCE
            && (p1.z - p2.z).abs() < TOLERANCE
    }
}

impl Default for Preprocessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arc_segment_calculation() {
        let preprocessor = Preprocessor::new().with_arc_precision(0.1);
        
        // For radius 10, precision 0.1
        let segments = preprocessor.calculate_arc_segments(10.0);
        assert!(segments >= 4);
        assert!(segments <= 360);
    }

    #[test]
    fn test_arc_expansion() {
        let start = Point3D::new(10.0, 0.0, 0.0);
        let end = Point3D::new(0.0, 10.0, 0.0);
        let center = Point3D::new(0.0, 0.0, 0.0);
        
        let arc = Segment::arc(start, end, center, ArcDirection::CounterClockwise, 1000.0);
        
        let preprocessor = Preprocessor::new().with_arc_precision(0.5);
        let expanded = preprocessor.expand_arc(&arc).unwrap();
        
        // Should generate multiple segments
        assert!(expanded.len() > 1);
        
        // All should be linear segments
        for seg in &expanded {
            assert_eq!(seg.segment_type, SegmentType::Linear);
        }
        
        // First segment should start where arc starts
        assert_eq!(expanded[0].start.x, start.x);
        assert_eq!(expanded[0].start.y, start.y);
        
        // Last segment should end where arc ends
        let last = expanded.last().unwrap();
        assert!((last.end.x - end.x).abs() < 0.01);
        assert!((last.end.y - end.y).abs() < 0.01);
    }

    #[test]
    fn test_unit_conversion() {
        let start = Point3D::new(1.0, 2.0, 3.0);
        let end = Point3D::new(4.0, 5.0, 6.0);
        let segment = Segment::linear(start, end, 100.0);
        
        let preprocessor = Preprocessor::new();
        let converted = preprocessor.convert_units(&segment, Units::Imperial, Units::Metric);
        
        assert_eq!(converted.start.x, 1.0 * 25.4);
        assert_eq!(converted.end.x, 4.0 * 25.4);
        assert_eq!(converted.feed_rate, 100.0 * 25.4);
    }

    #[test]
    fn test_optimize_rapids() {
        let segments = vec![
            Segment::rapid(Point3D::new(0.0, 0.0, 0.0), Point3D::new(10.0, 10.0, 0.0)),
            Segment::rapid(Point3D::new(10.0, 10.0, 0.0), Point3D::new(10.0, 10.0, 0.0)), // Duplicate
            Segment::linear(Point3D::new(10.0, 10.0, 0.0), Point3D::new(20.0, 20.0, 0.0), 1000.0),
        ];
        
        let preprocessor = Preprocessor::new();
        let optimized = preprocessor.optimize_rapids(&segments);
        
        // Should remove the duplicate rapid
        assert_eq!(optimized.len(), 2);
    }
}
