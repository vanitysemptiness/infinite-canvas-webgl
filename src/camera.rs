use cgmath::{Matrix4, SquareMatrix, Vector2, Vector3};
use winit::event::{ElementState, MouseButton, WindowEvent};
pub struct Camera {
    pub position: Vector2<f32>,
    pub zoom: f32,
    is_panning: bool,
    last_mouse_pos: Vector2<f32>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vector2::new(0.0, 0.0),
            zoom: 1.0,
            is_panning: false,
            last_mouse_pos: Vector2::new(0.0, 0.0),
        }
    }

    pub fn process_input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::MouseWheel { delta, .. } => {
                // Zoom logic...
                true
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                self.is_panning = *state == ElementState::Pressed;
                true
            }
            WindowEvent::CursorMoved { position, .. } => {
                let current_pos = Vector2::new(position.x as f32, position.y as f32);
                if self.is_panning {
                    let delta = (current_pos - self.last_mouse_pos) / self.zoom;
                    self.position -= delta;
                }
                self.last_mouse_pos = current_pos;
                self.is_panning
            }
            _ => false,
        }
    }

    pub fn update_uniform(&self, camera_uniform: &mut CameraUniform) {
        let mut camera_matrix = Matrix4::from_scale(self.zoom);
        camera_matrix = camera_matrix * Matrix4::from_translation(Vector3::new(
            -self.position.x,
            -self.position.y,
            0.0,
        ));
        camera_uniform.view_proj = camera_matrix.into();
        println!("Camera position: {:?}, zoom: {}", self.position, self.zoom);
    }
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }
}