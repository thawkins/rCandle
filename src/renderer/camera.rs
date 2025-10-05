//! Camera control for 3D viewport
//!
//! Provides camera positioning, rotation, and projection.

use nalgebra as na;

/// 3D Camera for viewing the toolpath
#[derive(Debug, Clone)]
pub struct Camera {
    /// Camera position in world space
    pub position: na::Point3<f32>,
    /// Camera target (look-at point)
    pub target: na::Point3<f32>,
    /// Up vector
    pub up: na::Vector3<f32>,
    /// Field of view in degrees
    pub fov: f32,
    /// Aspect ratio (width / height)
    pub aspect: f32,
    /// Near clipping plane
    pub near: f32,
    /// Far clipping plane
    pub far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: na::Point3::new(0.0, 0.0, 10.0),
            target: na::Point3::new(0.0, 0.0, 0.0),
            up: na::Vector3::new(0.0, 1.0, 0.0),
            fov: 45.0,
            aspect: 16.0 / 9.0,
            near: 0.1,
            far: 1000.0,
        }
    }
}

impl Camera {
    /// Create a new camera with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Build view matrix
    pub fn view_matrix(&self) -> na::Matrix4<f32> {
        na::Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }

    /// Build projection matrix
    pub fn projection_matrix(&self) -> na::Matrix4<f32> {
        na::Matrix4::new_perspective(self.aspect, self.fov.to_radians(), self.near, self.far)
    }

    /// Get combined view-projection matrix
    pub fn view_projection_matrix(&self) -> na::Matrix4<f32> {
        self.projection_matrix() * self.view_matrix()
    }

    /// Update aspect ratio (called when viewport resizes)
    pub fn set_aspect(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }

    /// Reset camera to default position
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Zoom camera in/out
    pub fn zoom(&mut self, delta: f32) {
        let direction = (self.position - self.target).normalize();
        let distance = (self.position - self.target).norm();
        let new_distance = (distance * (1.0 - delta * 0.1)).max(0.5);
        self.position = self.target + direction * new_distance;
    }

    /// Pan camera (move left/right/up/down)
    pub fn pan(&mut self, dx: f32, dy: f32) {
        let view = self.view_matrix();
        let right = na::Vector3::new(view[(0, 0)], view[(1, 0)], view[(2, 0)]);
        let up = na::Vector3::new(view[(0, 1)], view[(1, 1)], view[(2, 1)]);
        
        let distance = (self.position - self.target).norm();
        let pan_scale = distance * 0.001;
        
        let offset = right * dx * pan_scale + up * dy * pan_scale;
        self.position += offset;
        self.target += offset;
    }

    /// Rotate camera around target
    pub fn rotate(&mut self, yaw: f32, pitch: f32) {
        let distance = (self.position - self.target).norm();
        let direction = (self.position - self.target).normalize();
        
        // Convert to spherical coordinates
        let mut theta = direction.z.atan2(direction.x); // Yaw
        let mut phi = (direction.y / distance).acos(); // Pitch
        
        // Apply rotation
        theta += yaw;
        phi = (phi + pitch).clamp(0.01, std::f32::consts::PI - 0.01);
        
        // Convert back to Cartesian
        let x = distance * phi.sin() * theta.cos();
        let y = distance * phi.cos();
        let z = distance * phi.sin() * theta.sin();
        
        self.position = self.target + na::Vector3::new(x, y, z);
    }
}

/// Camera controller for handling user input
#[derive(Debug, Clone)]
pub struct CameraController {
    /// Rotation speed (radians per pixel)
    pub rotate_speed: f32,
    /// Pan speed
    pub pan_speed: f32,
    /// Zoom speed
    pub zoom_speed: f32,
    /// Whether left mouse button is pressed
    pub rotating: bool,
    /// Whether middle mouse button is pressed
    pub panning: bool,
    /// Last mouse position
    pub last_mouse_pos: Option<(f32, f32)>,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            rotate_speed: 0.005,
            pan_speed: 1.0,
            zoom_speed: 0.1,
            rotating: false,
            panning: false,
            last_mouse_pos: None,
        }
    }
}

impl CameraController {
    /// Create a new camera controller
    pub fn new() -> Self {
        Self::default()
    }

    /// Handle mouse press
    pub fn mouse_pressed(&mut self, button: MouseButton, x: f32, y: f32) {
        self.last_mouse_pos = Some((x, y));
        match button {
            MouseButton::Left => self.rotating = true,
            MouseButton::Middle => self.panning = true,
            _ => {}
        }
    }

    /// Handle mouse release
    pub fn mouse_released(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.rotating = false,
            MouseButton::Middle => self.panning = false,
            _ => {}
        }
        self.last_mouse_pos = None;
    }

    /// Handle mouse movement
    pub fn mouse_moved(&mut self, camera: &mut Camera, x: f32, y: f32) {
        if let Some((last_x, last_y)) = self.last_mouse_pos {
            let dx = x - last_x;
            let dy = y - last_y;

            if self.rotating {
                camera.rotate(-dx * self.rotate_speed, -dy * self.rotate_speed);
            } else if self.panning {
                camera.pan(-dx * self.pan_speed, dy * self.pan_speed);
            }
        }
        self.last_mouse_pos = Some((x, y));
    }

    /// Handle mouse wheel
    pub fn mouse_wheel(&mut self, camera: &mut Camera, delta: f32) {
        camera.zoom(delta * self.zoom_speed);
    }

    /// Reset controller state
    pub fn reset(&mut self) {
        self.rotating = false;
        self.panning = false;
        self.last_mouse_pos = None;
    }
}

/// Mouse button enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button
    Middle,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_default() {
        let camera = Camera::default();
        assert_eq!(camera.position.z, 10.0);
        assert_eq!(camera.target, na::Point3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_camera_matrices() {
        let camera = Camera::default();
        let view = camera.view_matrix();
        let proj = camera.projection_matrix();
        let vp = camera.view_projection_matrix();
        
        // Just verify they produce valid matrices
        assert!(!view[(0, 0)].is_nan());
        assert!(!proj[(0, 0)].is_nan());
        assert!(!vp[(0, 0)].is_nan());
    }

    #[test]
    fn test_camera_zoom() {
        let mut camera = Camera::default();
        let initial_distance = (camera.position - camera.target).norm();
        
        camera.zoom(1.0);
        let new_distance = (camera.position - camera.target).norm();
        
        assert!(new_distance < initial_distance);
    }

    #[test]
    fn test_camera_pan() {
        let mut camera = Camera::default();
        let initial_pos = camera.position;
        
        camera.pan(10.0, 10.0);
        
        assert_ne!(camera.position, initial_pos);
    }

    #[test]
    fn test_camera_rotate() {
        let mut camera = Camera::default();
        let initial_pos = camera.position;
        
        camera.rotate(0.1, 0.1);
        
        assert_ne!(camera.position, initial_pos);
        // Distance should remain constant
        let initial_dist = (initial_pos - camera.target).norm();
        let new_dist = (camera.position - camera.target).norm();
        assert!((initial_dist - new_dist).abs() < 0.01);
    }

    #[test]
    fn test_controller_state() {
        let mut controller = CameraController::new();
        
        controller.mouse_pressed(MouseButton::Left, 0.0, 0.0);
        assert!(controller.rotating);
        
        controller.mouse_released(MouseButton::Left);
        assert!(!controller.rotating);
    }
}
