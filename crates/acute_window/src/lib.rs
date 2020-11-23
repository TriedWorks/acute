mod window;
pub use window::*;
use acute_app::{Plugin, AppBuilder};

pub struct WindowPlugin;

impl Default for WindowPlugin {
    fn default() -> Self {
        Self { }
    }
}

impl Plugin for WindowPlugin {
    fn add(&self, app: &mut AppBuilder) {
        let mut windows = Windows::new();
        windows.add(Window::default());
        app.add_resource(windows);
    }
}

