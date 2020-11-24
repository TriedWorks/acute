//
use acute_app::{AppBuilder, Plugin, Timer};
use acute_assets::AssetPlugin;
use acute_input::InputPlugin;
use acute_window::WindowPlugin;
use acute_winit::WinitPlugin;
use legion::*;
use std::time::Duration;

#[system]
pub fn update_timer(#[resource] timer: &mut Timer) {
    timer.update_delta_time();
    timer.update_fixed_time();
}

pub trait DefaultPlugins {
    fn with_defaults(&mut self) -> &mut Self;
    fn with_defaults_headless(&mut self) -> &mut Self;
}

pub trait DefaultPluginAdditions {
    fn set_timer(&mut self, interval: f32) -> &mut Self;
}

impl DefaultPlugins for AppBuilder {
    fn with_defaults(&mut self) -> &mut Self {
        let mut timer = Timer::default();
        timer.set_fixed_interval(Duration::from_secs_f32(1.0 / 60.0));
        self.add_resource(Timer::default());

        WindowPlugin::default().add(self);
        WinitPlugin::default().add(self);
        InputPlugin::default().add(self);
        AssetPlugin::default().add(self);
        self
    }

    fn with_defaults_headless(&mut self) -> &mut Self {
        unimplemented!()
    }
}

impl DefaultPluginAdditions for AppBuilder {
    fn set_timer(&mut self, interval: f32) -> &mut Self {
        self.app
            .resources
            .get_mut::<Timer>()
            .unwrap()
            .set_fixed_interval(Duration::from_secs_f32(interval));

        self
    }
}
