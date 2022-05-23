use crate::renderer::WgpuRenderer;
use crate::resource_context::WgpuResourceContext;
use acute_app::{App, Plugin};
use futures::executor::block_on;
use renderer::graph_render_system;
use renderer::surface_creation_system;

mod encoder;
mod render_context;
mod render_graph;
mod renderer;
mod resource_context;
mod resources;

#[derive(Default)]
pub struct WgpuPlugin {}

impl Plugin for WgpuPlugin {
    fn build(&self, app: &mut App) {
        let renderer = block_on(WgpuRenderer::new());
        let context = WgpuResourceContext::new(renderer.device.clone());

        app.add_resource(renderer)
            .add_resource(context)
            .add_system(surface_creation_system())
            .add_system(graph_render_system());
    }
}
