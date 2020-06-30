use crate::graphics::pipeline::PipelineHandler;
use std::collections::HashMap;
use crate::graphics::texture::Texture;

pub struct RenderResources {
    pub vertex_buffers: Vec<wgpu::Buffer>,
    pub index_buffers: Vec<wgpu::Buffer>,
    pub indices_lengths: Vec<u32>,
    pub uniform_buffers: Vec<wgpu::Buffer>,
    pub uniform_bind_groups: Vec<wgpu::BindGroup>,
    pub textures: HashMap<String, Texture>,
    pub pipeline_handler: PipelineHandler,
}

impl RenderResources {
    pub fn new() -> Self {
        let pipeline_handler = PipelineHandler::new();
        Self {
            vertex_buffers: Vec::new(),
            index_buffers: Vec::new(),
            indices_lengths: Vec::new(),
            uniform_buffers: Vec::new(),
            uniform_bind_groups: Vec::new(),
            textures: HashMap::new(),
            pipeline_handler,
        }
    }
}