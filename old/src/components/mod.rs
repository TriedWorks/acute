pub mod default;
pub mod geometry;
pub mod material;
pub mod mesh;

pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a>;
}
