//
use legion::*;
use acute_assets::Assets;
use acute_input::Input;
use acute_window::winit::event::VirtualKeyCode::A;
use acute_window::winit::window::Window;
use acute_app::Timer;

#[system]
pub fn update_timer(#[resource] timer: &mut Timer) {
    timer.update_delta_time();
    timer.update_fixed_time();
}

pub trait DefaultAddons {
    fn with_defaults(self, window: Window) -> Self;
    fn with_defaults_headless(self) -> Self;
}

impl DefaultAddons for acute_app::AppBuilder {
    fn with_defaults(self, window: Window) -> Self {
        self.add_resource(Timer::new())
            .add_resource(Input::new())
            .add_resource(window)
            .add_resource(Assets::default())
            .add_system(update_timer_system())
    }

    fn with_defaults_headless(self) -> Self {
        self.add_resource(Timer::new())
            .add_system(update_timer_system())
    }
}
