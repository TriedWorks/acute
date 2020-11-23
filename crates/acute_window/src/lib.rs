mod winit_window;

pub use winit;
pub use winit_window::{WindowDescriptor, WinitWindow};
use acute_app::{Plugin, AppBuilder, RenderEvent, App};
use winit::event_loop::EventLoop;
use legion::*;

pub struct WinitWindowPlugin { }

impl Default for WinitWindowPlugin {
    fn default() -> Self {
        Self { }
    }
}

impl Plugin for WinitWindowPlugin {
    fn add(&self, app: &mut AppBuilder) {
        app.set_runner(winit_runner);
    }
}

pub fn winit_runner(mut app: App) {
    let mut event_loop = EventLoop::new();

    event_loop.run(move |event, _, control_flow| {
        app.schedule.execute(&mut app.scene.world, &mut app.resources)
    });
}