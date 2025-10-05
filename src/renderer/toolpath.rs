//! Toolpath rendering
//!
//! Renders G-Code toolpaths as 3D lines with different colors for different move types.

use crate::parser::{Segment, SegmentType};
use nalgebra as na;

/// Toolpath renderer
#[derive(Debug, Clone)]
pub struct ToolpathRenderer {
    /// Toolpath segments
    segments: Vec<Segment>,
    /// Whether to show rapids
    pub show_rapids: bool,
    /// Whether to show work moves
    pub show_work_moves: bool,
    /// Color for rapid moves (G0)
    pub rapid_color: [f32; 4],
    /// Color for work moves (G1)
    pub work_color: [f32; 4],
    /// Color for arc moves (G2/G3)
    pub arc_color: [f32; 4],
    /// Current position during rendering
    pub current_line: Option<usize>,
    /// Color for current line
    pub current_color: [f32; 4],
}

impl Default for ToolpathRenderer {
    fn default() -> Self {
        Self {
            segments: Vec::new(),
            show_rapids: true,
            show_work_moves: true,
            rapid_color: [1.0, 0.0, 0.0, 1.0],      // Red
            work_color: [0.0, 1.0, 0.0, 1.0],       // Green
            arc_color: [0.0, 0.5, 1.0, 1.0],        // Blue
            current_line: None,
            current_color: [1.0, 1.0, 0.0, 1.0],    // Yellow
        }
    }
}

impl ToolpathRenderer {
    /// Create a new toolpath renderer
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the segments to render
    pub fn set_segments(&mut self, segments: Vec<Segment>) {
        self.segments = segments;
    }

    /// Get the current segments
    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    /// Clear all segments
    pub fn clear(&mut self) {
        self.segments.clear();
        self.current_line = None;
    }

    /// Set current line (for highlighting during execution)
    pub fn set_current_line(&mut self, line: Option<usize>) {
        self.current_line = line;
    }

    /// Generate vertices for rendering
    pub fn generate_vertices(&self) -> Vec<ToolpathVertex> {
        let mut vertices = Vec::new();

        for (idx, segment) in self.segments.iter().enumerate() {
            let is_current = self.current_line == Some(idx);
            
            match &segment.segment_type {
                SegmentType::Rapid => {
                    if !self.show_rapids {
                        continue;
                    }
                    let color = if is_current { 
                        self.current_color 
                    } else { 
                        self.rapid_color 
                    };
                    
                    vertices.push(ToolpathVertex {
                        position: [segment.start.x as f32, segment.start.y as f32, segment.start.z as f32],
                        color,
                    });
                    vertices.push(ToolpathVertex {
                        position: [segment.end.x as f32, segment.end.y as f32, segment.end.z as f32],
                        color,
                    });
                }
                SegmentType::Linear => {
                    if !self.show_work_moves {
                        continue;
                    }
                    let color = if is_current { 
                        self.current_color 
                    } else { 
                        self.work_color 
                    };
                    
                    vertices.push(ToolpathVertex {
                        position: [segment.start.x as f32, segment.start.y as f32, segment.start.z as f32],
                        color,
                    });
                    vertices.push(ToolpathVertex {
                        position: [segment.end.x as f32, segment.end.y as f32, segment.end.z as f32],
                        color,
                    });
                }
                SegmentType::ArcCW | SegmentType::ArcCCW => {
                    if !self.show_work_moves {
                        continue;
                    }
                    let color = if is_current { 
                        self.current_color 
                    } else { 
                        self.arc_color 
                    };
                    
                    // Tessellate arc into line segments
                    if let Some(center) = segment.center {
                        let segments_per_arc = 32;
                        let radius = segment.start.distance_to(&center);
                        
                        // Calculate angles
                        let start_angle = (segment.start.y - center.y).atan2(segment.start.x - center.x);
                        let end_angle = (segment.end.y - center.y).atan2(segment.end.x - center.x);
                        
                        let mut angle_diff = end_angle - start_angle;
                        
                        // Adjust for arc direction
                        match segment.segment_type {
                            SegmentType::ArcCW => {
                                if angle_diff > 0.0 {
                                    angle_diff -= 2.0 * std::f64::consts::PI;
                                }
                            }
                            SegmentType::ArcCCW => {
                                if angle_diff < 0.0 {
                                    angle_diff += 2.0 * std::f64::consts::PI;
                                }
                            }
                            _ => {}
                        }
                        
                        let angle_step = angle_diff / segments_per_arc as f64;
                        
                        for i in 0..segments_per_arc {
                            let angle1 = start_angle + angle_step * i as f64;
                            let angle2 = start_angle + angle_step * (i + 1) as f64;
                            
                            let x1 = center.x + radius * angle1.cos();
                            let y1 = center.y + radius * angle1.sin();
                            let x2 = center.x + radius * angle2.cos();
                            let y2 = center.y + radius * angle2.sin();
                            
                            let z_ratio = (i + 1) as f64 / segments_per_arc as f64;
                            let z = segment.start.z + (segment.end.z - segment.start.z) * z_ratio;
                            
                            vertices.push(ToolpathVertex {
                                position: [x1 as f32, y1 as f32, z as f32],
                                color,
                            });
                            vertices.push(ToolpathVertex {
                                position: [x2 as f32, y2 as f32, z as f32],
                                color,
                            });
                        }
                    } else {
                        // Fallback to linear if no center point
                        vertices.push(ToolpathVertex {
                            position: [segment.start.x as f32, segment.start.y as f32, segment.start.z as f32],
                            color,
                        });
                        vertices.push(ToolpathVertex {
                            position: [segment.end.x as f32, segment.end.y as f32, segment.end.z as f32],
                            color,
                        });
                    }
                }
            }
        }

        vertices
    }

    /// Calculate bounding box of all segments
    pub fn bounding_box(&self) -> Option<BoundingBox> {
        if self.segments.is_empty() {
            return None;
        }

        let mut min_x = f64::MAX;
        let mut max_x = f64::MIN;
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;
        let mut min_z = f64::MAX;
        let mut max_z = f64::MIN;

        for segment in &self.segments {
            min_x = min_x.min(segment.start.x).min(segment.end.x);
            max_x = max_x.max(segment.start.x).max(segment.end.x);
            min_y = min_y.min(segment.start.y).min(segment.end.y);
            max_y = max_y.max(segment.start.y).max(segment.end.y);
            min_z = min_z.min(segment.start.z).min(segment.end.z);
            max_z = max_z.max(segment.start.z).max(segment.end.z);
        }

        Some(BoundingBox {
            min: na::Point3::new(min_x as f32, min_y as f32, min_z as f32),
            max: na::Point3::new(max_x as f32, max_y as f32, max_z as f32),
        })
    }

    /// Get total toolpath length
    pub fn total_length(&self) -> f64 {
        self.segments.iter().map(|s| s.length()).sum()
    }

    /// Get number of segments
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }
}

/// Vertex structure for toolpath rendering
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ToolpathVertex {
    /// Position in 3D space
    pub position: [f32; 3],
    /// Color (RGBA)
    pub color: [f32; 4],
}

unsafe impl bytemuck::Pod for ToolpathVertex {}
unsafe impl bytemuck::Zeroable for ToolpathVertex {}

impl ToolpathVertex {
    /// Get vertex buffer layout for WGPU
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ToolpathVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

/// Bounding box for toolpath
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    /// Minimum corner
    pub min: na::Point3<f32>,
    /// Maximum corner
    pub max: na::Point3<f32>,
}

impl BoundingBox {
    /// Get center of bounding box
    pub fn center(&self) -> na::Point3<f32> {
        na::Point3::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
            (self.min.z + self.max.z) / 2.0,
        )
    }

    /// Get size of bounding box
    pub fn size(&self) -> na::Vector3<f32> {
        na::Vector3::new(
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }

    /// Get diagonal distance
    pub fn diagonal(&self) -> f32 {
        self.size().norm()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Point3D;

    #[test]
    fn test_toolpath_renderer_default() {
        let renderer = ToolpathRenderer::default();
        assert_eq!(renderer.segments.len(), 0);
        assert!(renderer.show_rapids);
        assert!(renderer.show_work_moves);
    }

    #[test]
    fn test_set_segments() {
        let mut renderer = ToolpathRenderer::new();
        let segments = vec![
            Segment::rapid(
                Point3D { x: 0.0, y: 0.0, z: 0.0 },
                Point3D { x: 10.0, y: 10.0, z: 0.0 },
            ),
        ];
        
        renderer.set_segments(segments);
        assert_eq!(renderer.segments.len(), 1);
    }

    #[test]
    fn test_generate_vertices() {
        let mut renderer = ToolpathRenderer::new();
        let segments = vec![
            Segment::linear(
                Point3D { x: 0.0, y: 0.0, z: 0.0 },
                Point3D { x: 10.0, y: 10.0, z: 0.0 },
                1000.0,
            ),
        ];
        
        renderer.set_segments(segments);
        let vertices = renderer.generate_vertices();
        
        // Each line segment produces 2 vertices
        assert_eq!(vertices.len(), 2);
    }

    #[test]
    fn test_bounding_box() {
        let mut renderer = ToolpathRenderer::new();
        let segments = vec![
            Segment::linear(
                Point3D { x: 0.0, y: 0.0, z: 0.0 },
                Point3D { x: 10.0, y: 10.0, z: 5.0 },
                1000.0,
            ),
        ];
        
        renderer.set_segments(segments);
        let bbox = renderer.bounding_box().unwrap();
        
        assert_eq!(bbox.min, na::Point3::new(0.0, 0.0, 0.0));
        assert_eq!(bbox.max, na::Point3::new(10.0, 10.0, 5.0));
    }

    #[test]
    fn test_total_length() {
        let mut renderer = ToolpathRenderer::new();
        let segments = vec![
            Segment::linear(
                Point3D { x: 0.0, y: 0.0, z: 0.0 },
                Point3D { x: 10.0, y: 0.0, z: 0.0 },
                1000.0,
            ),
            Segment::linear(
                Point3D { x: 10.0, y: 0.0, z: 0.0 },
                Point3D { x: 10.0, y: 10.0, z: 0.0 },
                1000.0,
            ),
        ];
        
        renderer.set_segments(segments);
        assert!((renderer.total_length() - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_visibility_filters() {
        let mut renderer = ToolpathRenderer::new();
        let segments = vec![
            Segment::rapid(
                Point3D { x: 0.0, y: 0.0, z: 0.0 },
                Point3D { x: 10.0, y: 0.0, z: 0.0 },
            ),
        ];
        
        renderer.set_segments(segments);
        
        // With rapids visible
        let vertices = renderer.generate_vertices();
        assert_eq!(vertices.len(), 2);
        
        // Without rapids visible
        renderer.show_rapids = false;
        let vertices = renderer.generate_vertices();
        assert_eq!(vertices.len(), 0);
    }
}
