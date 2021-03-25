mod renderer;
mod render_context;
mod resources;
mod command_encoder;
mod render_pass;
mod texture;
mod buffer;
mod pipelines;
mod mesh;

pub use renderer::WgpuRenderer;
pub use wgpu;
use acute_app::{AppBuilder, Plugin};
use crate::render_context::RenderContext;
use acute_ecs::legion::{Resources, World};


pub struct WGPURenderPlugin { }

impl Default for WGPURenderPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for WGPURenderPlugin {
    fn add(&self, app: &mut AppBuilder) {
        let render_system = render_system(app.resources_mut());
        app.add_system(render_system);
    }
}

pub fn render_system(resources: &mut Resources) -> impl FnMut(&mut World, &mut Resources){
    let mut renderer = futures::executor::block_on(
        WgpuRenderer::new()
    );
    let render_context = RenderContext::new(renderer.device.clone());
    resources.insert(render_context);

    move |world, resources | {
        renderer.update(world, resources);
    }
}