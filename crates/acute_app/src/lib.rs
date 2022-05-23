mod app;
mod plugin;

use acute_ecs::schedule::StageLabel;
pub use app::App;
pub use plugin::*;

pub mod prelude {
    pub use super::app::App;
    pub use super::plugin::*;
    pub use super::{CoreStage, StartupSchedule, StartupStage};
}

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
