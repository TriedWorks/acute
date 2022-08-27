mod app;
mod plugin;

pub use app::{App, AppEventExit};
pub use plugin::*;

pub mod prelude {
    pub use super::app::App;
    pub use super::plugin::*;
    pub use super::{CoreStage, StartupSchedule, StartupStage};
}

use bevy_ecs::schedule::StageLabel;
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum CoreStage {
    First,
    PreUpdate,
    Update,
    PostUpdate,
    Last,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub struct StartupSchedule;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum StartupStage {
    PreStartup,
    Startup,
    PostStartup,
}
