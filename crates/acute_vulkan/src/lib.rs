extern crate core;

pub mod driver;

use crate::driver::instance::{Instance, InstanceCreateInfo};
use acute_app::{App, CoreStage, Plugin};
use acute_tracing::info;
use acute_window::{WindowEventClosed, WindowEventCreated};
use acute_winit::WinitWindows;
use bevy_ecs::event::EventReader;
use bevy_ecs::schedule::ShouldRun::No;
use bevy_ecs::system::{Commands, Res, ResMut};
use bevy_ecs::world::World;

pub mod prelude {
    // pub use crate::VkSettings;
    // pub use crate::VkApiVersion;
}

pub mod internal {
    pub use crate::driver;
    pub use ash;
    pub use vk_shader_macros as shader_macros;
    // pub use crate::{
    //     base::VkBase,
    //     renderer::{VkRenderer, Renderer, find_memory_type_index, record_submit_commandbuffer},
    // };
}

#[derive(Default)]
pub struct VulkanPlugin {}

#[derive(Default)]
pub struct VulkanRenderPlugin {}

impl Plugin for VulkanPlugin {
    fn build(&self, app: &mut App) {
        let info = app
            .get_resource::<InstanceCreateInfo>()
            .cloned()
            .unwrap_or_default();
        let windows = app.resource::<WinitWindows>();
        {
            let window = windows.windows.values().next().unwrap();

            let instance = Instance::new(info.clone()).unwrap();
            let adapters = instance.adapters().next().unwrap();
            let surface = instance.create_surface(window).unwrap();
            println!("{:?}", adapters);
        }
        // app
        //     .init_resource::<VkSettings>();
        //
        // let settings = app.resource::<VkSettings>();
        // let base = VkBase::init(settings).unwrap();
        // app.add_resource(base);
    }
}

impl Plugin for VulkanRenderPlugin {
    fn build(&self, app: &mut App) {
        // let mut renderer = Renderer::default();
        //
        // init_renderers(&app.world, &mut renderer);
        //
        // app.add_resource(renderer)
        //     .add_system(remove_renderer_on_window_close_system);
    }
}

// fn remove_renderer_on_window_close_system(
//     mut renderer: ResMut<Renderer>,
//     mut events: EventReader<WindowEventClosed>,
// ) {
//     for event in events.iter() {
//         let _ = renderer.remove(&event.id);
//     }
// }

// pub fn init_renderers(
//     world: &World,
//     renderer: &mut Renderer,
// ) {
//     let base = world.get_resource::<VkBase>().unwrap();
//     let windows = world.get_resource::<WinitWindows>().unwrap();
//
//     windows.windows.iter().for_each(|(id, window)| {
//         let id = windows.get_window_id(*id).unwrap();
//         if !renderer.contains(&id) {
//             renderer.add(id, VkRenderer::init(base, window).unwrap());
//             info!("Init: renderer for window {:?}", id)
//         }
//
//     })
// }

// fn render_creation_system(
//     mut created: EventReader<WindowEventCreated>,
//     mut renderer: ResMut<Renderer>,
//     windows: Res<WinitWindows>,
//     settings: Res<VkSettings>,
// ) {
//     let settings = settings.as_ref();
//     for event in created.iter() {
//         println!("Window CREATED!!! {:?}", event.id);
//     }
// }
