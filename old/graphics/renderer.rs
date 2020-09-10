use glsl_to_spirv::ShaderType;
use legion::prelude::*;

use crate::components::default::{Color, ID};
use crate::graphics::buffer;
use crate::graphics::resources::RenderResources;
use crate::tools::uniforms::Uniforms;
use crate::{
    components::{default::Transform, geometry::Mesh},
    graphics::{
        pipeline, shader, texture,
        types::{Renderable, VertexC},
    },
    tools::{camera::Camera, uniforms},
};

// Vertex Buffer is fixed sized right now, this is not good and should be changed!
// Split functions up and make it dynamic

pub struct Renderer {
    pub surface: wgpu::Surface,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub window: winit::window::Window,
    pub uniforms: Uniforms,
    pub resources: RenderResources,
}

impl Renderer {
    pub async fn new(window: winit::window::Window, size: winit::dpi::PhysicalSize<u32>) -> Self {
        let surface = wgpu::Surface::create(&window);
        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        )
        .await
        .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: wgpu::Limits::default(),
            })
            .await;

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let mut resources = RenderResources::new(&device, &sc_desc);

        let mut uniforms = uniforms::Uniforms::new();

        resources.new_uniform_buffer(&device, &[uniforms], 0);

        let (uniform_bind_group, uniform_bind_group_layout) = buffer::create_uniform_bind_group(
            &device,
            wgpu::ShaderStage::VERTEX,
            0,
            "uniform_bind_group_layout",
            0,
            "uniform_bind_group",
            &resources.uniform_buffers.get(&0).unwrap(),
            std::mem::size_of_val(&uniforms),
        );

        resources.uniform_bind_groups.push(uniform_bind_group);
        resources
            .uniform_bind_group_layouts
            .push(uniform_bind_group_layout);

        // following should be moved into their own
        let vs_module = shader::create_shader_module(
            include_str!("../../assets/shaders/default.vert"),
            ShaderType::Vertex,
            &device,
        );

        let fs_module = shader::create_shader_module(
            include_str!("../../assets/shaders/default.frag"),
            ShaderType::Fragment,
            &device,
        );

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&resources.uniform_bind_group_layouts[0]],
            });

        let render_pipeline = pipeline::create_render_pipeline(
            &device,
            &render_pipeline_layout,
            wgpu::PrimitiveTopology::TriangleList,
            &vs_module,
            &fs_module,
            sc_desc.format.clone(),
            texture::DEPTH_FORMAT,
            &[VertexC::desc()],
            true,
            "main",
        );

        resources
            .pipeline_handler
            .insert_pipeline("testing", render_pipeline);

        Self {
            surface,
            size,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            window,
            uniforms,
            resources,
        }
    }

    // Physics change within a fixed interval but render is as often as possible
    pub fn update_render_data(&mut self, world: &World, camera: &Camera) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Update Encoder"),
            });

        let query = <(Read<Transform>, Read<Mesh>, Read<Color>)>::query();
        let mut all_vertices: Vec<VertexC> = Vec::new();
        for (transform, mesh, color) in query.iter(world) {
            all_vertices.extend(Mesh::vertices_of(&mesh, &transform, Some(&color)))
        }
        self.resources
            .update_vertex_data(&self.device, &mut encoder, all_vertices);
        self.uniforms.update_view_proj(camera.to_matrix());

        self.resources
            .update_uniform_buffer(&self.device, &mut encoder, &[self.uniforms], 0);

        self.queue.submit(&[encoder.finish()]);
    }

    pub fn render(&mut self, world: &World) {
        let frame = self
            .swap_chain
            .get_next_texture()
            .expect("Timeout getting texture");

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.2,
                        g: 0.0,
                        b: 0.5,
                        a: 1.0,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.resources.textures.get("depth_texture").unwrap().view,
                    depth_load_op: wgpu::LoadOp::Clear,
                    depth_store_op: wgpu::StoreOp::Store,
                    clear_depth: 1.0,
                    stencil_load_op: wgpu::LoadOp::Clear,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_stencil: 0,
                }),
            });

            render_pass.set_pipeline(
                &self
                    .resources
                    .pipeline_handler
                    .pipelines
                    .get("testing")
                    .unwrap(),
            );
            render_pass.set_bind_group(0, &self.resources.uniform_bind_groups[0], &[]);

            for buffer in &self.resources.vertex_buffers {
                render_pass.set_vertex_buffer(0, &buffer.buffer, 0, 0);
                render_pass.draw(0..buffer.vertex_amount, 0..1);
            }
        }
        self.queue.submit(&[encoder.finish()]);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        self.resources.textures.insert(
            "depth_texture".to_owned(),
            texture::Texture::new_depth(&self.device, &self.sc_desc, "depth_texture"),
        );
    }
}
