use std::collections::HashMap;
use acute_window::winit::window::Window;

pub struct WgpuResources {
    pub swap_chain: wgpu::SwapChain,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub window: Window,
    pub surface: wgpu::Surface,
    pub pipelines: HashMap<String, wgpu::RenderPipeline>
}

impl WgpuResources {
    pub fn new(window: Window, surface: wgpu::Surface, device: &wgpu::Device) -> Self {
        let size = window.inner_size();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            // set this to Fifo to enable "vsync"
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Self {
            swap_chain,
            sc_desc,
            window,
            surface,
            pipelines: Default::default()
        }
    }

    pub fn with_testing(&mut self, device: &wgpu::Device) {
        self.pipelines.insert("none".to_string(), create_test_pipeline(device, &self.sc_desc));
    }
}

fn create_test_pipeline(
    device: &wgpu::Device,
    sc_desc: &wgpu::SwapChainDescriptor,
) -> wgpu::RenderPipeline {
    let vs_module = device.create_shader_module(wgpu::include_spirv!(
        "../../../assets/shaders/compiled/none.vert.spv"
    ));
    let fs_module = device.create_shader_module(wgpu::include_spirv!(
        "../../../assets/shaders/compiled/none.frag.spv"
    ));

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("test pipeline"),
        layout: Some(&layout),
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fs_module,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::Back,
            clamp_depth: false,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states: &[wgpu::ColorStateDescriptor {
            format: sc_desc.format,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint32,
            vertex_buffers: &[],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    render_pipeline
}
