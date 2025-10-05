//! Main renderer implementation
//!
//! Manages WGPU rendering context and coordinates rendering of grid, axes, and toolpath.

use super::{Camera, CameraController, grid::{Grid, Axes}, toolpath::ToolpathRenderer};
use crate::parser::Segment;
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// Main renderer for 3D visualization
pub struct Renderer {
    /// WGPU device
    device: Arc<wgpu::Device>,
    /// WGPU queue
    queue: Arc<wgpu::Queue>,
    /// Render pipeline
    pipeline: wgpu::RenderPipeline,
    /// Camera
    camera: Camera,
    /// Camera controller
    camera_controller: CameraController,
    /// Grid
    grid: Grid,
    /// Axes
    axes: Axes,
    /// Toolpath renderer
    toolpath: ToolpathRenderer,
    /// Uniform buffer for view-projection matrix
    uniform_buffer: wgpu::Buffer,
    /// Bind group
    bind_group: wgpu::BindGroup,
}

impl Renderer {
    /// Create a new renderer
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>, format: wgpu::TextureFormat) -> Self {
        // Create shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Renderer Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/line.wgsl").into()),
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Renderer Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Create uniform buffer
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: std::mem::size_of::<[[f32; 4]; 4]>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Renderer Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Renderer Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Renderer Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[super::grid::Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            device,
            queue,
            pipeline,
            camera: Camera::new(),
            camera_controller: CameraController::new(),
            grid: Grid::new(),
            axes: Axes::new(),
            toolpath: ToolpathRenderer::new(),
            uniform_buffer,
            bind_group,
        }
    }

    /// Get mutable reference to camera
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    /// Get reference to camera
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// Get mutable reference to camera controller
    pub fn camera_controller_mut(&mut self) -> &mut CameraController {
        &mut self.camera_controller
    }

    /// Get reference to camera controller
    pub fn camera_controller(&self) -> &CameraController {
        &self.camera_controller
    }

    /// Get mutable reference to grid
    pub fn grid_mut(&mut self) -> &mut Grid {
        &mut self.grid
    }

    /// Get reference to grid
    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    /// Get mutable reference to axes
    pub fn axes_mut(&mut self) -> &mut Axes {
        &mut self.axes
    }

    /// Get reference to axes
    pub fn axes(&self) -> &Axes {
        &self.axes
    }

    /// Get mutable reference to toolpath
    pub fn toolpath_mut(&mut self) -> &mut ToolpathRenderer {
        &mut self.toolpath
    }

    /// Get reference to toolpath
    pub fn toolpath(&self) -> &ToolpathRenderer {
        &self.toolpath
    }

    /// Set toolpath segments
    pub fn set_segments(&mut self, segments: Vec<Segment>) {
        self.toolpath.set_segments(segments);
    }

    /// Update uniform buffer with current camera matrices
    fn update_uniforms(&self) {
        let vp_matrix = self.camera.view_projection_matrix();
        let matrix_ref: &[[f32; 4]; 4] = vp_matrix.as_ref();
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(matrix_ref));
    }

    /// Render the scene
    pub fn render(&self, view: &wgpu::TextureView, depth_view: &wgpu::TextureView) {
        // Update uniform buffer
        self.update_uniforms();

        // Generate all vertices upfront
        let grid_vertices = if self.grid.visible {
            self.grid.generate_vertices()
        } else {
            Vec::new()
        };

        let axes_vertices = if self.axes.visible {
            self.axes.generate_vertices()
        } else {
            Vec::new()
        };

        let toolpath_vertices = self.toolpath.generate_vertices();

        // Create vertex buffers
        let grid_buffer = if !grid_vertices.is_empty() {
            Some(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Grid Vertex Buffer"),
                contents: bytemuck::cast_slice(&grid_vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }))
        } else {
            None
        };

        let axes_buffer = if !axes_vertices.is_empty() {
            Some(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Axes Vertex Buffer"),
                contents: bytemuck::cast_slice(&axes_vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }))
        } else {
            None
        };

        let toolpath_buffer = if !toolpath_vertices.is_empty() {
            Some(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Toolpath Vertex Buffer"),
                contents: bytemuck::cast_slice(&toolpath_vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }))
        } else {
            None
        };

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Renderer Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Renderer Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.15,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);

            // Render grid
            if let Some(ref buffer) = grid_buffer {
                render_pass.set_vertex_buffer(0, buffer.slice(..));
                render_pass.draw(0..grid_vertices.len() as u32, 0..1);
            }

            // Render axes
            if let Some(ref buffer) = axes_buffer {
                render_pass.set_vertex_buffer(0, buffer.slice(..));
                render_pass.draw(0..axes_vertices.len() as u32, 0..1);
            }

            // Render toolpath
            if let Some(ref buffer) = toolpath_buffer {
                render_pass.set_vertex_buffer(0, buffer.slice(..));
                render_pass.draw(0..toolpath_vertices.len() as u32, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
    }

    /// Reset camera to default view
    pub fn reset_camera(&mut self) {
        self.camera.reset();
    }

    /// Zoom camera to fit all content
    pub fn zoom_to_fit(&mut self) {
        if let Some(bbox) = self.toolpath.bounding_box() {
            let center = bbox.center();
            let size = bbox.size();
            let distance = size.norm() * 1.5;

            self.camera.target = center;
            self.camera.position = center + nalgebra::Vector3::new(distance, distance, distance);
        }
    }
    
    /// Calculate bounds of the toolpath
    /// Returns (min, max) as (Vec3, Vec3)
    pub fn calculate_bounds(&self) -> (glam::Vec3, glam::Vec3) {
        if let Some(bbox) = self.toolpath.bounding_box() {
            let center = bbox.center();
            let size = bbox.size();
            
            let min = glam::Vec3::new(
                center.x - size.x / 2.0,
                center.y - size.y / 2.0,
                center.z - size.z / 2.0,
            );
            
            let max = glam::Vec3::new(
                center.x + size.x / 2.0,
                center.y + size.y / 2.0,
                center.z + size.z / 2.0,
            );
            
            (min, max)
        } else {
            // Default bounds if no toolpath
            (glam::Vec3::new(-50.0, -50.0, -50.0), glam::Vec3::new(50.0, 50.0, 50.0))
        }
    }
    
    /// Apply a view preset to the camera
    pub fn apply_view_preset(&mut self, preset: super::ViewPreset, center: glam::Vec3, distance: f32) {
        use super::view_presets::ViewPreset;
        
        // Convert glam::Vec3 to nalgebra::Point3
        let center_pt = nalgebra::Point3::new(center.x, center.y, center.z);
        
        match preset {
            ViewPreset::Top => {
                self.camera.position = center_pt + nalgebra::Vector3::new(0.0, 0.0, distance);
                self.camera.target = center_pt;
            }
            ViewPreset::Bottom => {
                self.camera.position = center_pt + nalgebra::Vector3::new(0.0, 0.0, -distance);
                self.camera.target = center_pt;
            }
            ViewPreset::Front => {
                self.camera.position = center_pt + nalgebra::Vector3::new(0.0, distance, 0.0);
                self.camera.target = center_pt;
            }
            ViewPreset::Back => {
                self.camera.position = center_pt + nalgebra::Vector3::new(0.0, -distance, 0.0);
                self.camera.target = center_pt;
            }
            ViewPreset::Right => {
                self.camera.position = center_pt + nalgebra::Vector3::new(distance, 0.0, 0.0);
                self.camera.target = center_pt;
            }
            ViewPreset::Left => {
                self.camera.position = center_pt + nalgebra::Vector3::new(-distance, 0.0, 0.0);
                self.camera.target = center_pt;
            }
            ViewPreset::Isometric => {
                let iso_distance = distance / 1.732; // sqrt(3)
                self.camera.position = center_pt + nalgebra::Vector3::new(
                    iso_distance,
                    iso_distance,
                    iso_distance,
                );
                self.camera.target = center_pt;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests are limited because they require a WGPU device
    // which is not always available in test environments

    #[test]
    fn test_camera_operations() {
        let mut camera = Camera::new();
        let initial_pos = camera.position;

        camera.zoom(0.5);
        assert_ne!(camera.position, initial_pos);

        camera.reset();
        assert_eq!(camera.position.z, 10.0);
    }

    #[test]
    fn test_grid_generation() {
        let grid = Grid::new();
        let vertices = grid.generate_vertices();
        assert!(!vertices.is_empty());
    }

    #[test]
    fn test_axes_generation() {
        let axes = Axes::new();
        let vertices = axes.generate_vertices();
        assert_eq!(vertices.len(), 6); // 3 axes * 2 vertices
    }
}
