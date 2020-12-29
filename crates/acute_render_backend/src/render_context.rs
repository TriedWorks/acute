use std::sync::Arc;
use crate::resources::Resources;
use crate::command_encoder::CommandEncoder;
use acute_window::{Window, WindowId};
use crate::texture::TextureId;

#[derive(Debug)]
pub struct RenderContext {
    pub device: Arc<wgpu::Device>,
    pub encoder: CommandEncoder,
    pub resources: Resources,
}

impl RenderContext {
    pub fn new(device: Arc<wgpu::Device>) -> Self {
        Self {
            device,
            encoder: Default::default(),
            resources: Default::default()
        }
    }

    pub fn finish(&mut self) -> Option<wgpu::CommandBuffer> {
        self.encoder.take().map(|encoder| encoder.finish())
    }

    fn create_swap_chain(&self, window: &Window) {
        let surfaces = self.resources.surfaces.read();
        let mut swap_chains = self.resources.swap_chains.write();

        let surface = surfaces.get(&window.id()).expect("No surface found for window.");

        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: window.width(),
            height: window.height(),
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swap_chain = self.device.create_swap_chain(surface, &swap_chain_descriptor);
        swap_chains.insert(window.id(), swap_chain);
    }

    fn next_swap_chain_texture(&self, window: &Window) -> TextureId {
        if let Some(texture_id) = self.try_next_swap_chain_texture(window.id()) {
            texture_id
        } else {
            self.resources.swap_chains.write().remove(&window.id());
            self.create_swap_chain(window);
            self.try_next_swap_chain_texture(window.id())
                .expect("Failed to create next swap chain!")
        }
    }

    fn drop_swap_chain_texture(&self, texture: TextureId) {
        let mut swap_chain_outputs = self.resources.swap_chain_frames.write();
        swap_chain_outputs.remove(&texture);
    }

    fn drop_all_swap_chain_textures(&self) {
        let mut swap_chain_outputs = self.resources.swap_chain_frames.write();
        swap_chain_outputs.clear();
    }


    fn try_next_swap_chain_texture(&self, window_id: WindowId) -> Option<TextureId> {
        let mut swap_chains = self.resources.swap_chains.write();
        let mut swap_chain_outputs = self.resources.swap_chain_frames.write();

        let swap_chain = swap_chains.get_mut(&window_id).unwrap();
        let next_texture = swap_chain.get_current_frame().ok()?;
        let id = TextureId::new();
        swap_chain_outputs.insert(id, next_texture);
        Some(id)
    }
}