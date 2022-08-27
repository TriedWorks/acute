pub mod imports;

use acute_app::{PluginBundle, PluginBundleBuilder};
use acute_input::InputPlugin;
use acute_tracing::TracingPlugin;
use acute_vulkan::{VulkanPlugin, VulkanRenderPlugin};
use acute_window::WindowPlugin;
use acute_winit::WinitPlugin;

#[derive(Default)]
pub struct DefaultPlugins {}

#[derive(Default)]
pub struct ComputePlugins {}

// #[derive(Default)]
// pub struct Minimum {}

impl PluginBundle for DefaultPlugins {
    fn build(&mut self, group: &mut PluginBundleBuilder) {
        group.add_init::<TracingPlugin>();
        group.add_init::<InputPlugin>();
        group.add_init::<WindowPlugin>();
        group.add_init::<WinitPlugin>();
        group.add_init::<VulkanPlugin>();
        group.add_init::<VulkanRenderPlugin>();
        // group.add_init::<WgpuPlugin>();
    }
}

impl PluginBundle for ComputePlugins {
    fn build(&mut self, group: &mut PluginBundleBuilder) {
        group.add_init::<TracingPlugin>();
        group.add_init::<VulkanPlugin>();
    }
}
