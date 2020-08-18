pub mod prelude;

pub use acute_app;
pub use acute_core;
use acute_core::{update_timer, Timer};
pub use acute_ecs;
pub use acute_input;
use acute_input::Input;
use acute_window::window::Window;

pub trait DefaultAddons {
    fn with_defaults(self, window: Window) -> Self;
    fn with_defaults_headless(self) -> Self;
}

impl DefaultAddons for acute_app::AppBuilder {
    fn with_defaults(self, window: Window) -> Self {
        self.add_resource(Timer::new())
            .add_resource(Input::new())
            .with_window(window)
            .add_system(update_timer())
    }

    fn with_defaults_headless(self) -> Self {
        self.add_resource(Timer::new()).add_system(update_timer())
    }
}
