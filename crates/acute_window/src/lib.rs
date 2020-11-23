mod winit_window;

pub use winit;
pub use winit_window::{WindowDescriptor, WinitWindow};
use acute_app::{Plugin, AppBuilder, RenderEvent};
use winit::event_loop::EventLoop;
use std::ops::Deref;
use legion::*;
use std::sync::Mutex;

pub struct WinitWindowPlugin { }

impl Default for WinitWindowPlugin {
    fn default() -> Self {
        Self { }
    }
}

impl Plugin for WinitWindowPlugin {
    fn add(&self, app: &mut AppBuilder) {
    }
}
