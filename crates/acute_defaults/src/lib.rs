pub mod imports;

use acute_app::{PluginBundle, PluginBundleBuilder};
use acute_input::InputPlugin;
use acute_wgpu::WgpuPlugin;
use acute_window::WindowPlugin;
use acute_winit::WinitPlugin;

#[derive(Default)]
pub struct DefaultBundle {}

impl PluginBundle for DefaultBundle {
    fn build(&mut self, group: &mut PluginBundleBuilder) {
        group.add::<WindowPlugin>();
        group.add::<WinitPlugin>();
        group.add::<InputPlugin>();
        group.add::<WgpuPlugin>();
    }
}
