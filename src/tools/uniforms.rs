use crate::tools::camera::Camera;
use ultraviolet::Mat4;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Uniforms {
    view_proj: Mat4,
}

impl Uniforms {
    pub fn new() -> Self {
        let mut matrix: Mat4 = Mat4::identity();
        Self {
            view_proj: matrix,
        }
    }

    pub fn update_view_proj(&mut self, view_proj: Mat4) {
        self.view_proj = view_proj;
    }
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}