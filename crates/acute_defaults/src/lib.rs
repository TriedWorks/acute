pub mod imports;

use acute_app::{PluginBundle, PluginBundleBuilder};
use acute_input::InputPlugin;
use acute_vulkan::VulkanPlugin;
use acute_window::WindowPlugin;
use acute_winit::WinitPlugin;

#[derive(Default)]
pub struct DefaultBundle {}

impl PluginBundle for DefaultBundle {
    fn build(&mut self, group: &mut PluginBundleBuilder) {
        group.add_init::<InputPlugin>();
        group.add_init::<WindowPlugin>();
        group.add_init::<WinitPlugin>();
        group.add_init::<VulkanPlugin>();
        // group.add_init::<WgpuPlugin>();
    }
}
