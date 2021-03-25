use acute_app::{AppBuilder, Events, Plugin};

pub use crate::events::{WindowCloseRequested, WindowCreate, WindowCreated};
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
    fn add(&self, app: &mut AppBuilder) {
        app.add_event::<WindowCreate>()
            .add_event::<WindowCreated>()
            .add_event::<WindowCloseRequested>()
            .add_resource(Windows::new());

        if self.add_primary {
            let descriptor = WindowDescriptor::default();

            let mut create_window_events =
                app.resources().get_mut::<Events<WindowCreate>>().unwrap();

            create_window_events.send(WindowCreate {
                id: WindowId::new(),
                descriptor,
            })
        }
    }
}
