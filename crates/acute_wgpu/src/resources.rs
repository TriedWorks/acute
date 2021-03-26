use acute_window::WindowId;
use parking_lot::{RwLock, RwLockReadGuard};
use std::collections::HashMap;
use std::sync::Arc;

pub struct WgpuResources {
    pub surfaces: Arc<RwLock<HashMap<WindowId, wgpu::Surface>>>,
    pub swap_chains: Arc<RwLock<HashMap<WindowId, wgpu::SwapChain>>>,
    pub sc_descs: Arc<RwLock<HashMap<WindowId, wgpu::SwapChainDescriptor>>>,
}

pub struct WgpuResourcesReadGuard<'a> {
    pub surfaces: RwLockReadGuard<'a, HashMap<WindowId, wgpu::Surface>>,
    pub swap_chains: RwLockReadGuard<'a, HashMap<WindowId, wgpu::SwapChain>>,
    pub sc_descs: RwLockReadGuard<'a, HashMap<WindowId, wgpu::SwapChainDescriptor>>,
}

pub struct WgpuResourcesRef<'a> {
    pub surfaces: &'a HashMap<WindowId, wgpu::Surface>,
    pub swap_chains: &'a HashMap<WindowId, wgpu::SwapChain>,
    pub sc_descs: &'a HashMap<WindowId, wgpu::SwapChainDescriptor>,
}

impl WgpuResources {
    pub fn new() -> Self {
        Self {
            surfaces: Arc::new(Default::default()),
            swap_chains: Default::default(),
            sc_descs: Default::default(),
        }
    }

    pub fn read(&self) -> WgpuResourcesReadGuard {
        WgpuResourcesReadGuard {
            surfaces: self.surfaces.read(),
            swap_chains: self.swap_chains.read(),
            sc_descs: self.sc_descs.read(),
        }
    }
}

impl<'a> WgpuResourcesReadGuard<'a> {
    pub fn refs(&'a self) -> WgpuResourcesRef<'a> {
        WgpuResourcesRef {
            surfaces: &self.surfaces,
            swap_chains: &self.swap_chains,
            sc_descs: &self.sc_descs,
        }
    }
}
