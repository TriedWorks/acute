use crate::resources::WgpuResources;
use acute_window::WindowId;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct WgpuResourceContext {
    pub device: Arc<wgpu::Device>,
    pub resources: WgpuResources,
}

impl WgpuResourceContext {
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

    pub fn set_swap_chain(&self, window_id: WindowId, sc_desc: wgpu::SwapChainDescriptor) {
        let mut swap_chains = self.resources.swap_chains.write();
        let mut sc_descs = self.resources.sc_descs.write();
        let surfaces = self.resources.surfaces.read();
        let surface = surfaces.get(&window_id).unwrap();
        let swap_chain = self.device.create_swap_chain(surface, &sc_desc);
        swap_chains.insert(window_id, swap_chain);
        sc_descs.insert(window_id, sc_desc);
    }
}
