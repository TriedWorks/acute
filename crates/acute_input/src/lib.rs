mod codes;
mod input;
mod keyboard;
mod mouse;

use acute_app::{AppBuilder, Plugin};
pub use codes::*;
pub use input::Input;

pub struct InputPlugin;

impl Default for InputPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for InputPlugin {
    fn add(&self, app: &mut AppBuilder) {
        app.add_resource(Input::new());
    }
}
