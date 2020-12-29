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
use acute_app::AppBuilder;


pub struct WGPURenderPlugin { }

impl Default for WGPURenderPlugin {
    fn default() -> Self {
        Self {}
    }
}

// impl Plugin for WGPURenderPlugin {
//     fn add(&self, app: &mut AppBuilder) {
//         let renderer = futures::executor::block_on(
//             // WgpuRenderer::new(&mut app.app.resources)
//         );
//         app.add_resource(renderer);
//     }
// }