use acute_app::{App, Plugin};
use acute_ecs::event::Events;
pub use events::*;
pub use window::*;

mod events;
mod window;

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
        app.add_event::<WindowCreateEvent>()
            .add_event::<WindowCreatedEvent>()
            .add_event::<WindowCloseRequestedEvent>()
            .init_resource::<Windows>();

        if self.add_primary {
            let descriptor = app
                .get_resource_mut::<WindowDescriptor>()
                .map(|desc| (*desc).clone())
                .unwrap_or_default();

            let mut create_window_events = app.resource_mut::<Events<WindowCreateEvent>>();

            create_window_events.send(WindowCreateEvent {
                id: WindowId::new(),
                descriptor,
            })
        }
    }
}
