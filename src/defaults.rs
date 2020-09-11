//
use acute_core::{update_timer_system, Timer};
use acute_input::Input;
use acute_window::winit::window::Window;

pub trait DefaultAddons {
    fn with_defaults(self, window: Window) -> Self;
    fn with_defaults_headless(self) -> Self;
}

impl DefaultAddons for acute_app::AppBuilder {
    fn with_defaults(self, window: Window) -> Self {
        self.add_resource(Timer::new())
            .add_resource(Input::new())
            .add_resource(window)
            .add_system(update_timer_system())
    }

    fn with_defaults_headless(self) -> Self {
        self.add_resource(Timer::new())
            .add_system(update_timer_system())
    }
}
