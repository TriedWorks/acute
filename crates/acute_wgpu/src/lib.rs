use crate::context::WgpuContext;
use crate::renderer::WgpuRenderer;
use acute_app::{AppBuilder, Plugin};
use futures::executor::block_on;
use renderer::surface_creation_system;

mod context;
mod encoder;
mod renderer;
mod resources;

#[derive(Default)]
pub struct WgpuPlugin {}

impl Plugin for WgpuPlugin {
    fn add(&self, app: &mut AppBuilder) {
        let renderer = block_on(WgpuRenderer::new());
        let context = WgpuContext::new(renderer.device.clone());

        app.add_resource(renderer)
            .add_resource(context)
            .add_system(surface_creation_system());
    }
}
