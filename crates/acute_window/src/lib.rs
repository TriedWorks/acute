use acute_app::{App, AppEventExit, Plugin};
use acute_tracing::info;
use bevy_ecs::event::{EventWriter, Events};
use bevy_ecs::system::ResMut;
pub use events::*;
pub use window::*;
pub use windows::*;

mod events;
mod window;
mod windows;

pub struct WindowPlugin {
    pub add_primary: bool,
}

impl Default for WindowPlugin {
    fn default() -> Self {
        Self { add_primary: true }
    }
}

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WindowEventCreate>()
            .add_event::<WindowEventCreated>()
            .add_event::<WindowEventCloseRequested>()
            .add_event::<WindowEventClosed>()
            .add_system(exit_on_no_windows_system)
            .init_resource::<Windows>();

        if self.add_primary {
            let descriptor = app
                .get_resource_mut::<WindowDescriptor>()
                .map(|desc| (*desc).clone())
                .unwrap_or_default();

            let mut create_window_events = app.resource_mut::<Events<WindowEventCreate>>();

            create_window_events.send(WindowEventCreate {
                id: WindowId::new(),
                descriptor,
            })
        }

        info!("Loaded Plugin: Window")
    }
}

pub fn exit_on_no_windows_system(
    mut windows: ResMut<Windows>,
    mut events: EventWriter<AppEventExit>,
) {
    if windows.iter().count() == 0 {
        events.send(AppEventExit)
    }
}
