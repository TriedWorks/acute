mod app;
mod builder;
mod state;
mod events;

pub use app::App;
pub use builder::AppBuilder;
pub use state::State;
pub use rusty_timer::Timer;
pub use events::*;

pub trait Plugin {
    fn add(&self, app: &mut AppBuilder);
}