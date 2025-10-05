//! Grid rendering for the 3D viewport
//!
//! Renders a reference grid and coordinate axes.

/// Grid configuration
#[derive(Debug, Clone)]
pub struct Grid {
    /// Size of the grid (distance from center to edge)
    pub size: f32,
    /// Spacing between grid lines
    pub spacing: f32,
    /// Grid color
    pub color: [f32; 4],
    /// Whether to show the grid
    pub visible: bool,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            size: 100.0,
            spacing: 10.0,
            color: [0.3, 0.3, 0.3, 1.0],
            visible: true,
        }
    }
}

impl Grid {
    /// Create a new grid with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate grid line vertices
    pub fn generate_vertices(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        if !self.visible {
            return vertices;
        }

        let steps = (self.size / self.spacing) as i32;

        // Horizontal lines (along X axis)
        for i in -steps..=steps {
            let z = i as f32 * self.spacing;
            vertices.push(Vertex {
                position: [-self.size, 0.0, z],
                color: self.color,
            });
            vertices.push(Vertex {
                position: [self.size, 0.0, z],
                color: self.color,
            });
        }

        // Vertical lines (along Z axis)
        for i in -steps..=steps {
            let x = i as f32 * self.spacing;
            vertices.push(Vertex {
                position: [x, 0.0, -self.size],
                color: self.color,
            });
            vertices.push(Vertex {
                position: [x, 0.0, self.size],
                color: self.color,
            });
        }

        vertices
    }

    /// Set grid visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Set grid size
    pub fn set_size(&mut self, size: f32) {
        self.size = size.max(10.0);
    }

    /// Set grid spacing
    pub fn set_spacing(&mut self, spacing: f32) {
        self.spacing = spacing.max(1.0);
    }
}

/// Coordinate axes configuration
#[derive(Debug, Clone)]
pub struct Axes {
    /// Length of each axis
    pub length: f32,
    /// Whether to show axes
    pub visible: bool,
}

impl Default for Axes {
    fn default() -> Self {
        Self {
            length: 50.0,
            visible: true,
        }
    }
}

impl Axes {
    /// Create new axes with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate axis line vertices
    pub fn generate_vertices(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        if !self.visible {
            return vertices;
        }

        // X axis - Red
        vertices.push(Vertex {
            position: [0.0, 0.0, 0.0],
            color: [1.0, 0.0, 0.0, 1.0],
        });
        vertices.push(Vertex {
            position: [self.length, 0.0, 0.0],
            color: [1.0, 0.0, 0.0, 1.0],
        });

        // Y axis - Green
        vertices.push(Vertex {
            position: [0.0, 0.0, 0.0],
            color: [0.0, 1.0, 0.0, 1.0],
        });
        vertices.push(Vertex {
            position: [0.0, self.length, 0.0],
            color: [0.0, 1.0, 0.0, 1.0],
        });

        // Z axis - Blue
        vertices.push(Vertex {
            position: [0.0, 0.0, 0.0],
            color: [0.0, 0.0, 1.0, 1.0],
        });
        vertices.push(Vertex {
            position: [0.0, 0.0, self.length],
            color: [0.0, 0.0, 1.0, 1.0],
        });

        vertices
    }

    /// Set axes visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Set axes length
    pub fn set_length(&mut self, length: f32) {
        self.length = length.max(1.0);
    }
}

/// Vertex structure for grid and axes
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    /// Position in 3D space
    pub position: [f32; 3],
    /// Color (RGBA)
    pub color: [f32; 4],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    /// Get vertex buffer layout for WGPU
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_default() {
        let grid = Grid::default();
        assert_eq!(grid.size, 100.0);
        assert_eq!(grid.spacing, 10.0);
        assert!(grid.visible);
    }

    #[test]
    fn test_grid_vertices() {
        let grid = Grid::new();
        let vertices = grid.generate_vertices();
        assert!(!vertices.is_empty());
        
        // Each grid line has 2 vertices
        // Should have (steps*2+1)*2 horizontal + (steps*2+1)*2 vertical lines
        let steps = (grid.size / grid.spacing) as usize;
        let expected_lines = (steps * 2 + 1) * 2;
        assert_eq!(vertices.len(), expected_lines * 2);
    }

    #[test]
    fn test_grid_visibility() {
        let mut grid = Grid::new();
        grid.set_visible(false);
        let vertices = grid.generate_vertices();
        assert_eq!(vertices.len(), 0);
    }

    #[test]
    fn test_axes_default() {
        let axes = Axes::default();
        assert_eq!(axes.length, 50.0);
        assert!(axes.visible);
    }

    #[test]
    fn test_axes_vertices() {
        let axes = Axes::new();
        let vertices = axes.generate_vertices();
        
        // Should have 3 axes * 2 vertices each = 6 vertices
        assert_eq!(vertices.len(), 6);
        
        // Check X axis is red
        assert_eq!(vertices[1].color, [1.0, 0.0, 0.0, 1.0]);
        // Check Y axis is green
        assert_eq!(vertices[3].color, [0.0, 1.0, 0.0, 1.0]);
        // Check Z axis is blue
        assert_eq!(vertices[5].color, [0.0, 0.0, 1.0, 1.0]);
    }

    #[test]
    fn test_axes_visibility() {
        let mut axes = Axes::new();
        axes.set_visible(false);
        let vertices = axes.generate_vertices();
        assert_eq!(vertices.len(), 0);
    }

    #[test]
    fn test_vertex_size() {
        // Verify Vertex is correctly sized for GPU
        assert_eq!(std::mem::size_of::<Vertex>(), 28);
    }
}
