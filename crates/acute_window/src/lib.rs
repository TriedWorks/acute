mod winit_window;

pub use winit;
pub use winit_window::{WindowDescriptor, WinitWindow};
use acute_app::{Plugin, AppBuilder, RenderEvent};
use winit::event_loop::EventLoop;
use std::ops::Deref;
use legion::*;

pub struct WinitWindowPlugin { }

impl Default for WinitWindowPlugin {
    fn default() -> Self {
        Self { }
    }
}

impl Plugin for WinitWindowPlugin {
    fn add(&self, app: &mut AppBuilder) {
        let window = WinitWindow::new(WindowDescriptor::default(), &app.app.event_loop.as_ref().unwrap());
        app.add_resource(window);
        app.add_resource(RenderEvent::Nothing);
    }
}