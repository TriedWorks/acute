pub mod default;
pub mod geometry;
pub mod mesh;
pub mod material;

pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a>;
}