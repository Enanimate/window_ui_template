use glam::{Mat4, Vec2, Vec3};
use winit::dpi::PhysicalSize;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Camera2DUniform {
    pub(crate) view_proj: [[f32; 4]; 4],
}

pub(crate) struct Camera2D {
    position: Vec2,
    screen_size: PhysicalSize<u32>,
}

impl Camera2D {
    pub(crate) fn new(screen_width: u32, screen_height: u32) -> Self {
        Self { 
            position: Vec2::new(0.0, 0.0), 
            screen_size: PhysicalSize::new(screen_width, screen_height),
        }
    }

    fn build_projection_matrix(&self) -> Mat4 {
        let width = self.screen_size.width as f32;
        let height = self.screen_size.height as f32;

        Mat4::orthographic_rh_gl(
            0.0,
            width,
            height,
            0.0,
            -1.0,
            1.0,
        )
    }

    fn build_view_matrix(&self) -> Mat4 {
        Mat4::from_translation(Vec3::new(-self.position.x, -self.position.y, 0.0))
    }

    pub(crate) fn build_view_projection_matrix(&self) -> Mat4 {
        let view_proj = self.build_projection_matrix() * self.build_view_matrix();
        view_proj
    }

    pub(crate) fn update_screen_size(&mut self, new_size: PhysicalSize<u32>) {
        self.screen_size = new_size;
    }
}