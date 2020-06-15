use super::texture;
use legion::world::World;
use crate::graphics::{shader, pipeline};
use glsl_to_spirv::ShaderType;


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
    pub depth_texture: texture::Texture,
    pub window: winit::window::Window,

    //following should be moved into their own
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    render_pipeline: wgpu::RenderPipeline,
    uniforms: uniforms::Uniforms,
    uniform_bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
}

impl Renderer {
    pub async fn new(
        window: winit::window::Window,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> Self {
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

        let depth_texture = texture::Texture::new_depth(&device, &sc_desc, "depth_texture");

        
        // following should be moved into their own
        let vs_module = shader::create_shader_module(
            include_str!("../../../assets/shaders/default_vertex.glsl"),
            ShaderType::Vertex,
            &device,
        );

        let fs_module = shader::create_shader_module(
            include_str!("../../../assets/shaders/default_fragment.glsl"),
            ShaderType::Fragment,
            &device,
        );

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&uniform_bind_group_layout],
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

        let vertex_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(VERTICES),
            wgpu::BufferUsage::VERTEX,
        );

        let index_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(INDICES),
            wgpu::BufferUsage::INDEX,
        );

        Self {
            surface,
            size,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            depth_texture,
            window,
            vertex_buffer,
            index_buffer,
            num_indices: 0,
            render_pipeline,
            uniforms: (),
            uniform_bind_group: (),
            uniform_buffer: ()
        }
    }

    pub fn render(&mut self, world: &World) {

    }
}