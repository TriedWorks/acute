mod winit_window;

pub use winit;
pub use winit_window::{WindowDescriptor, WinitWindow};
use acute_app::{Plugin, AppBuilder};
use winit::event_loop::EventLoop;
use std::ops::Deref;

pub struct WinitWindowPlugin { }

impl Default for WinitWindowPlugin {
    fn default() -> Self {
        Self { }
    }
}

impl Plugin for WinitWindowPlugin {
    fn add(&self, app: &mut AppBuilder) {
        let event_loop = EventLoop::new();
        let window = WinitWindow::new(WindowDescriptor::default(), &event_loop);

        app.app.resources.insert(event_loop);
        app.app.resources.insert(window);
    }
}