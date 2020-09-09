use acute_window::winit::window::Window;
use wgpu::ShaderModule;

pub struct Renderer {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc: wgpu::SwapChain,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub window: Window,
    pub surface: wgpu::Surface,
    // pub pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub async fn new(window: Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let (size, surface) = unsafe {
            let size = window.inner_size();
            let surface = instance.create_surface(&window);
            (size, surface)
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: adapter.features(),
                    // use default limits, might change later, if it causes issues.
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            // set this to Fifo to enable "vsync"
            present_mode: wgpu::PresentMode::Fifo,
        };

        let sc = device.create_swap_chain(&surface, &sc_desc);

        // let test_pipeline = create_test_pipeline(&device);

        Self {
            instance,
            adapter,
            device,
            queue,
            sc,
            sc_desc,
            window,
            surface,
            // pipeline: test_pipeline
        }
    }
}

fn create_test_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
    use inline_spirv::include_spirv;
    unimplemented!()
    // let spv_1: &'static [u32] = inline_spirv!(r#"
    //     #version 450
    //
    //     const vec2 positions[3] = vec2[3](
    //         vec2(0.0, 0.5),
    //         vec2(-0.5, -0.5),
    //         vec2(0.5, -0.5)
    //     );
    //
    //     void main() {
    //         gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
    //     }
    //
    //     "#, vert);
    //
    // let spv_2: &'static [u32] = inline_spirv!(r#"
    //     #version 450
    //
    //     layout(location=0) out vec4 f_color;
    //
    //     void main() {
    //         f_color = vec4(0.6, 0.2, 0.9, 1.0);
    //     }
    //
    //     "#, frag);
    //
    // let vs_module = device.create_shader_module(wgpu::ShaderModule());
    // let fs_module = device.create_shader_module(wgpu::include_spirv!("../../../assets/shaders/none.fs.spv"));
    //
    // let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    //     label: None,
    //     bind_group_layouts: &[],
    //     push_constant_ranges: &[]
    // });
    //
    // device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    //     label: None,
    //     layout: Some(&layout),
    //     vertex_stage: wgpu::ProgrammableStageDescriptor {
    //         module: &vs_module,
    //         entry_point: "",
    //     },
    //     fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
    //         module: &fs_module,
    //         entry_point: "",
    //     }),
    //     rasterization_state: None,
    //     primitive_topology: wgpu::PrimitiveTopology::TriangleList,
    //     color_states: &[],
    //     depth_stencil_state: None,
    //     vertex_state: wgpu::VertexStateDescriptor {
    //         index_format: Default::default(),
    //         vertex_buffers: &[]
    //     },
    //     sample_count: 1,
    //     sample_mask: !0,
    //     alpha_to_coverage_enabled: false
}

