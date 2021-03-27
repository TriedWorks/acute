use crate::encoder::Encoder;
use crate::resource_context::WgpuResourceContext;
use std::sync::Arc;

pub struct WgpuRenderContext {
    pub device: Arc<wgpu::Device>,
    pub resources: WgpuResourceContext,
    pub encoder: Encoder,
}

impl WgpuRenderContext {
    pub fn new(device: Arc<wgpu::Device>, resources: WgpuResourceContext) -> Self {
        Self {
            device,
            resources,
            encoder: Encoder::default(),
        }
    }

    pub fn finish(&mut self) -> Option<wgpu::CommandBuffer> {
        self.encoder.take().map(|encoder| encoder.finish())
    }
}
