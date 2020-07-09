use crate::graphics::pipeline::PipelineHandler;
use std::collections::HashMap;
use crate::graphics::texture::Texture;
use crate::graphics::texture;
use crate::graphics::types::VertexC;
use crate::tools::uniforms::Uniforms;
use crate::tools::uniforms;

pub struct RenderResources {
    pub vertex_buffers: HashMap<i32, wgpu::Buffer>,
    pub index_buffers: Vec<wgpu::Buffer>,
    pub indices_lengths: Vec<u32>,
    pub uniform_buffers: HashMap<i32, wgpu::Buffer>,
    pub uniform_bind_groups: Vec<wgpu::BindGroup>,
    pub uniform_bind_group_layouts: Vec<wgpu::BindGroupLayout>,
    pub textures: HashMap<String, Texture>,
    pub pipeline_handler: PipelineHandler,
}

impl RenderResources {
    pub fn new(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor) -> Self {
        let pipeline_handler = PipelineHandler::new(device, sc_desc);
        let mut textures = HashMap::new();
        let depth_texture = texture::Texture::new_depth(device, sc_desc, "depth_texture");
        textures.insert("depth_texture".to_owned(), depth_texture);
        Self {
            vertex_buffers: HashMap::new(),
            index_buffers: Vec::new(),
            indices_lengths: Vec::new(),
            uniform_buffers: HashMap::new(),
            uniform_bind_groups: Vec::new(),
            uniform_bind_group_layouts: Vec::new(),
            textures,
            pipeline_handler,
        }
    }

    // creates a new
    pub fn new_vertex_buffer(&mut self, device: &wgpu::Device, data: &Vec<VertexC>, id: i32) {
        let vertex_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(data),
            wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST
        );
        self.vertex_buffers.insert(id, vertex_buffer);
    }

    pub fn update_vertex_buffer(&mut self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder, data: &Vec<VertexC>, id: i32) {
        let staging_vertex_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(data),
            wgpu::BufferUsage::COPY_SRC,
        );
        encoder.copy_buffer_to_buffer(
            &staging_vertex_buffer,
            0,
            self.vertex_buffers.get(&id).unwrap(),
            0,
            (std::mem::size_of::<VertexC>() * data.len()) as wgpu::BufferAddress
        );
    }

    pub fn new_uniform_buffer(&mut self, device: &wgpu::Device, data: &[Uniforms], id: i32) {
        let uniform_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(data),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );
        self.uniform_buffers.insert(id, uniform_buffer);
    }

    pub fn update_uniform_buffer(&mut self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder, data: &[Uniforms], id: i32) {
        let staging_uniform_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(data),
            wgpu::BufferUsage::COPY_SRC,
        );
        encoder.copy_buffer_to_buffer(
            &staging_uniform_buffer,
            0,
            &self.uniform_buffers.get(&id).unwrap(),
            0,
            std::mem::size_of::<uniforms::Uniforms>() as wgpu::BufferAddress,
        )
    }
}
