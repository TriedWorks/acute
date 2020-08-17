use crate::graphics::buffer::VertexBuffer;
use crate::graphics::pipeline::PipelineHandler;
use crate::graphics::texture;
use crate::graphics::texture::Texture;
use crate::graphics::types::VertexC;
use crate::tools::uniforms;
use crate::tools::uniforms::Uniforms;
use std::collections::HashMap;

// 16 MB Buffer
// TODO: check graphics card max size and use that as a default value
const VERTEX_BUFFER_DEFAULT_SIZE: usize = 16_777_216;

pub struct RenderResources {
    pub vertex_buffers: Vec<VertexBuffer>,
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
            vertex_buffers: Vec::new(),
            uniform_buffers: HashMap::new(),
            uniform_bind_groups: Vec::new(),
            uniform_bind_group_layouts: Vec::new(),
            textures,
            pipeline_handler,
        }
    }

    pub fn update_vertex_data(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        vertex_data: Vec<VertexC>,
    ) {
        let data_size = std::mem::size_of::<VertexC>() * vertex_data.len();
        let mut data_sliced = Vec::new();
        let mut tmp_slice = Vec::new();
        let mut tmp_len: usize = 0;
        if data_size > VERTEX_BUFFER_DEFAULT_SIZE {
            for vertex in vertex_data {
                tmp_len += std::mem::size_of::<VertexC>();
                tmp_slice.push(vertex);
                // vbds - vbds / 100 is to make sure the default size is not overshot
                if tmp_len >= VERTEX_BUFFER_DEFAULT_SIZE - VERTEX_BUFFER_DEFAULT_SIZE / 100 {
                    data_sliced.push(tmp_slice);
                    tmp_len = 0;
                    tmp_slice = Vec::new();
                }
            }
        } else {
            data_sliced = vec![vertex_data];
        }

        let missing_vertex_buffer_amount: i32 =
            self.vertex_buffers.len() as i32 - data_sliced.len() as i32;
        if missing_vertex_buffer_amount != 0 {
            for _ in 0..-missing_vertex_buffer_amount {
                if missing_vertex_buffer_amount < 0 {
                    self.new_vertex_buffer(device, None);
                } else {
                    self.vertex_buffers.pop();
                }
            }
        }

        data_sliced
            .iter()
            .zip(self.vertex_buffers.iter_mut())
            .for_each(|(data, buffer)| {
                let staging_vertex_buffer = device.create_buffer_with_data(
                    bytemuck::cast_slice(data),
                    wgpu::BufferUsage::COPY_SRC,
                );
                encoder.copy_buffer_to_buffer(
                    &staging_vertex_buffer,
                    0,
                    &buffer.buffer,
                    0,
                    (std::mem::size_of::<VertexC>() * data.len()) as wgpu::BufferAddress,
                );
                buffer.current_size = data_size;
                buffer.available_size = buffer.max_size - buffer.current_size;
                buffer.vertex_amount = data.len() as u32;
            });
    }

    pub fn new_vertex_buffer(&mut self, device: &wgpu::Device, size: Option<usize>) {
        let size = match size {
            Some(size) => size,
            None => VERTEX_BUFFER_DEFAULT_SIZE,
        };

        let label = format!("vertex_buffer_{}", self.vertex_buffers.len());

        let vertex_buffer_desc = wgpu::BufferDescriptor {
            label: Some(&label),
            size: size as wgpu::BufferAddress,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
        };

        let vertex_buffer = device.create_buffer(&vertex_buffer_desc);
        self.vertex_buffers.push(VertexBuffer {
            buffer: vertex_buffer,
            max_size: size,
            current_size: 0,
            available_size: size,
            vertex_amount: 0,
        })
    }

    pub fn new_uniform_buffer(&mut self, device: &wgpu::Device, data: &[Uniforms], id: i32) {
        let uniform_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(data),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );
        self.uniform_buffers.insert(id, uniform_buffer);
    }

    pub fn update_uniform_buffer(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        data: &[Uniforms],
        id: i32,
    ) {
        let staging_uniform_buffer =
            device.create_buffer_with_data(bytemuck::cast_slice(data), wgpu::BufferUsage::COPY_SRC);
        encoder.copy_buffer_to_buffer(
            &staging_uniform_buffer,
            0,
            &self.uniform_buffers.get(&id).unwrap(),
            0,
            std::mem::size_of::<uniforms::Uniforms>() as wgpu::BufferAddress,
        )
    }
}
