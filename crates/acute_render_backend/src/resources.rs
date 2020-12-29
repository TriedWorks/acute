use std::collections::HashMap;
use crate::buffer::BufferId;
use parking_lot::{RwLock, RwLockReadGuard};
use std::sync::Arc;
use acute_window::WindowId;
use crate::texture::TextureId;

#[derive(Debug)]
pub struct ResourceRefs<'a> {
    pub swap_chain_frames: &'a HashMap<TextureId, wgpu::SwapChainFrame>,
    pub buffers: &'a HashMap<BufferId, Arc<wgpu::Buffer>>,
    pub pipelines: &'a HashMap<String, wgpu::RenderPipeline>,
}

#[derive(Debug)]
pub struct ResourcesReadLock<'a> {
    pub swap_chain_frames: RwLockReadGuard<'a, HashMap<TextureId, wgpu::SwapChainFrame>>,
    pub buffers: RwLockReadGuard<'a, HashMap<BufferId, Arc<wgpu::Buffer>>>,
    pub pipelines: RwLockReadGuard<'a, HashMap<String, wgpu::RenderPipeline>>,
}

#[derive(Default, Clone, Debug)]
pub struct Resources {
    pub surfaces: Arc<RwLock<HashMap<WindowId, wgpu::Surface>>>,
    pub swap_chains: Arc<RwLock<HashMap<WindowId, wgpu::SwapChain>>>,
    pub swap_chain_frames: Arc<RwLock<HashMap<TextureId, wgpu::SwapChainFrame>>>,
    pub buffers: Arc<RwLock<HashMap<BufferId, Arc<wgpu::Buffer>>>>,
    pub pipelines: Arc<RwLock<HashMap<String, wgpu::RenderPipeline>>>,
}

impl Resources {
    pub fn read(&self) -> ResourcesReadLock {
        ResourcesReadLock {
            swap_chain_frames: self.swap_chain_frames.read(),
            buffers: self.buffers.read(),
            pipelines: self.pipelines.read(),
        }
    }
}

impl<'a> ResourcesReadLock<'a> {
    pub fn refs(&'a self) -> ResourceRefs<'a> {
        ResourceRefs {
            swap_chain_frames: &self.swap_chain_frames,
            buffers: &self.buffers,
            pipelines: &self.pipelines,
        }
    }
}

// fn create_test_pipeline(
//     device: &wgpu::Device,
//     sc_desc: &wgpu::SwapChainDescriptor,
// ) -> wgpu::RenderPipeline {
//     let vs_module = device.create_shader_module(wgpu::include_spirv!(
//         "../../../assets/shaders/compiled/simple_color.vert.spv"
//     ));
//     let fs_module = device.create_shader_module(wgpu::include_spirv!(
//         "../../../assets/shaders/compiled/simple_color.frag.spv"
//     ));
//
//     let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//         label: None,
//         bind_group_layouts: &[],
//         push_constant_ranges: &[],
//     });
//
//     let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//         label: Some("test pipeline"),
//         layout: Some(&layout),
//         vertex_stage: wgpu::ProgrammableStageDescriptor {
//             module: &vs_module,
//             entry_point: "main",
//         },
//         fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
//             module: &fs_module,
//             entry_point: "main",
//         }),
//         rasterization_state: Some(wgpu::RasterizationStateDescriptor {
//             front_face: wgpu::FrontFace::Ccw,
//             cull_mode: wgpu::CullMode::Back,
//             clamp_depth: false,
//             depth_bias: 0,
//             depth_bias_slope_scale: 0.0,
//             depth_bias_clamp: 0.0,
//         }),
//         primitive_topology: wgpu::PrimitiveTopology::TriangleList,
//         color_states: &[wgpu::ColorStateDescriptor {
//             format: sc_desc.format,
//             color_blend: wgpu::BlendDescriptor::REPLACE,
//             alpha_blend: wgpu::BlendDescriptor::REPLACE,
//             write_mask: wgpu::ColorWrite::ALL,
//         }],
//         depth_stencil_state: None,
//         vertex_state: wgpu::VertexStateDescriptor {
//             index_format: wgpu::IndexFormat::Uint32,
//             vertex_buffers: &[Vertex::desc()],
//         },
//         sample_count: 1,
//         sample_mask: !0,
//         alpha_to_coverage_enabled: false,
//     });
//
//     render_pipeline
// }
