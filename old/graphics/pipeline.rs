use crate::graphics::types::VertexC;
use crate::graphics::{shader, texture};
use glsl_to_spirv::ShaderType;
use std::collections::HashMap;
use wgpu::ShaderModule;

pub struct PipelineHandler {
    pub pipelines: HashMap<String, wgpu::RenderPipeline>,
    pub shaders: HashMap<String, ShaderModule>,
}

impl PipelineHandler {
    pub fn new(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor) -> Self {
        let mut pipelines = HashMap::new();
        let mut shaders = HashMap::new();

        let vs_module = shader::create_shader_module(
            include_str!("../../assets/shaders/none.vert"),
            ShaderType::Vertex,
            &device,
        );

        let fs_module = shader::create_shader_module(
            include_str!("../../assets/shaders/none.frag"),
            ShaderType::Fragment,
            &device,
        );
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[],
            });

        let render_pipeline = create_render_pipeline(
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
        pipelines.insert("none".to_string(), render_pipeline);
        shaders.insert("none_vs".to_string(), vs_module);
        shaders.insert("none_fs".to_string(), fs_module);

        Self { pipelines, shaders }
    }

    pub fn insert_pipeline(&mut self, label: &str, pipeline: wgpu::RenderPipeline) {
        self.pipelines.insert(label.to_string(), pipeline);
    }
}

pub fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    primitive: wgpu::PrimitiveTopology,
    vs_module: &wgpu::ShaderModule,
    fs_module: &wgpu::ShaderModule,
    color_format: wgpu::TextureFormat,
    depth_format: wgpu::TextureFormat,
    vertex_buffers: &[wgpu::VertexBufferDescriptor],
    alpha: bool,
    entry_point: &str,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: vs_module,
            entry_point,
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: fs_module,
            entry_point,
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::Back,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        color_states: &[wgpu::ColorStateDescriptor {
            format: color_format,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::all(),
        }],
        primitive_topology: primitive,
        depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
            format: depth_format,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
            stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
            stencil_read_mask: 0,
            stencil_write_mask: 0,
        }),
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers,
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: alpha,
    })
}

pub fn create_render_pipeline_no_depth(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    primitive: wgpu::PrimitiveTopology,
    vs_module: &wgpu::ShaderModule,
    fs_module: &wgpu::ShaderModule,
    color_format: wgpu::TextureFormat,
    vertex_buffers: &[wgpu::VertexBufferDescriptor],
    alpha: bool,
    entry_point: &str,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: vs_module,
            entry_point,
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: fs_module,
            entry_point,
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::Back,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        color_states: &[wgpu::ColorStateDescriptor {
            format: color_format,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::all(),
        }],
        primitive_topology: primitive,
        depth_stencil_state: None,
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers,
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: alpha,
    })
}

pub fn default_texture_bind_group_layout(
    device: &wgpu::Device,
    label: &str,
) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        bindings: &[
            create_bind_group_layout_entry(
                0,
                wgpu::ShaderStage::FRAGMENT,
                wgpu::BindingType::SampledTexture {
                    multisampled: false,
                    dimension: wgpu::TextureViewDimension::D2,
                    component_type: wgpu::TextureComponentType::Uint,
                },
            ),
            create_bind_group_layout_entry(
                1,
                wgpu::ShaderStage::FRAGMENT,
                wgpu::BindingType::Sampler { comparison: false },
            ),
        ],
        label: Some(label),
    })
}

fn create_bind_group_layout_entry(
    binding: u32,
    visibility: wgpu::ShaderStage,
    ty: wgpu::BindingType,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty,
    }
}
