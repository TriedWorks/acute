mod app;
mod builder;
mod events;
mod state;

pub use app::App;
pub use builder::AppBuilder;
pub use events::*;
pub use rusty_timer::Timer;
pub use state::State;

pub trait Plugin {
    fn add(&self, app: &mut AppBuilder);
}
