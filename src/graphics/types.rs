use bytemuck::{Zeroable, Pod};
use ultraviolet::{
    Vec3,
};
use crate::components::default::{Transform, Color};

pub trait Renderable {
    fn vertices_of(&self, transform: &Transform, color: Option<&Color>) -> Vec<VertexC>;

    fn to_indexed(&self) {}
}




#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VertexC {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

impl VertexC {
    pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3
                },
                wgpu::VertexAttributeDescriptor {
                    offset: std::mem::size_of::<Vec3>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float4,
                },
            ],
        }
    }
}

unsafe impl Zeroable for VertexC { }
unsafe impl Pod for VertexC {}