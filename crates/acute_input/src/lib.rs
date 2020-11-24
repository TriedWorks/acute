mod input;
mod keyboard;
mod mouse;
mod codes;

pub use input::Input;
pub use codes::*;
use acute_app::{Plugin, AppBuilder};

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