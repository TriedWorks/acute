mod codes;
pub mod events;
mod keyboard;
mod mouse;

pub use codes::*;
pub use keyboard::Keyboard;
pub use mouse::Mouse;

use crate::events::{KeyboardEvent, MouseButtonEvent, MouseMoveEvent, MouseScrollEvent};
use crate::keyboard::keyboard_update_system;
use crate::mouse::{
    mouse_button_update_system, mouse_move_update_system, mouse_scroll_update_system,
};
use acute_app::{AppBuilder, Plugin};

pub struct InputPlugin;

impl Default for InputPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<KeyboardEvent>()
            .add_event::<MouseButtonEvent>()
            .add_event::<MouseScrollEvent>()
            .add_event::<MouseMoveEvent>()
            .add_resource(Keyboard::new())
            .add_resource(Mouse::new())
            .add_system(keyboard_update_system())
            .add_system(mouse_button_update_system())
            .add_system(mouse_scroll_update_system())
            .add_system(mouse_move_update_system());
    }
}
