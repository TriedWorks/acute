use ultraviolet::{
    Vec4,
    Vec3,
    Vec2
};
use wgpu::VertexBufferDescriptor;

pub struct MeshVertex {
    position: Vec3,
    normal: Vec3,
    uv: Vec3,
    tangent: Vec4,
}

impl super::Vertex for MeshVertex {
    fn desc<'a>() -> VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<MeshVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<Vec3>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: (mem::size_of::<Vec3>() + mem::size_of::<Vec2>()) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float3
                }
            ]
        }
    }
}

pub struct Mesh {
    pub vertices: Vec<MeshVertex>,
    pub triangles: Vec<u32>,
}

pub struct MeshRenderer {

}

pub struct MeshFilter {

}