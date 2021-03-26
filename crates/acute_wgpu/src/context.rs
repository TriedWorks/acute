use crate::encoder::Encoder;
use crate::resources::WgpuResources;
use acute_window::WindowId;
use legion::system;
use std::sync::Arc;

pub struct WgpuContext {
    pub device: Arc<wgpu::Device>,
    pub resources: WgpuResources,
}

impl WgpuContext {
    pub fn new(device: Arc<wgpu::Device>) -> Self {
        Self {
            device,
            resources: WgpuResources::new(),
        }
    }

    pub fn set_surface(&self, window_id: WindowId, surface: wgpu::Surface) {
        let mut surfaces = self.resources.surfaces.write();
        surfaces.insert(window_id, surface);
    }
}
