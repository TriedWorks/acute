//
use legion::*;
use acute_app::{Timer, Plugin, AppBuilder};
use acute_window::WindowPlugin;
use acute_winit::WinitPlugin;

#[system]
pub fn update_timer(#[resource] timer: &mut Timer) {
    timer.update_delta_time();
    timer.update_fixed_time();
}

pub trait DefaultPlugins {
    fn with_defaults(&mut self) -> &mut Self;
    fn with_defaults_headless(&mut self) -> &mut Self;
}

impl DefaultPlugins for AppBuilder {
    fn with_defaults(&mut self) -> &mut Self {
        WindowPlugin::default().add(self);
        WinitPlugin::default().add(self);

        self
    }

    fn with_defaults_headless(&mut self) -> &mut Self {
        unimplemented!()
    }
}
