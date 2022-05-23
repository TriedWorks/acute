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
use acute_app::{App, CoreStage, Plugin};
use acute_ecs::{schedule::ParallelSystemDescriptorCoercion, schedule::SystemLabel};

pub mod prelude {
    pub use crate::codes::*;
    pub use crate::keyboard::Keyboard;
    pub use crate::mouse::Mouse;
}

pub struct InputPlugin;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct InputSystem;

impl Default for InputPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<KeyboardEvent>()
            .add_event::<MouseButtonEvent>()
            .add_event::<MouseScrollEvent>()
            .add_event::<MouseMoveEvent>()
            .init_resource::<Keyboard>()
            .init_resource::<Mouse>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                keyboard_update_system.label(InputSystem),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                mouse_button_update_system.label(InputSystem),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                mouse_scroll_update_system.label(InputSystem),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                mouse_move_update_system.label(InputSystem),
            );
    }
}
