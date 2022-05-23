pub mod archetype;
pub mod bundle;
pub mod change_detection;
pub mod component;
pub mod entity;
pub mod event;
pub mod query;
pub mod schedule;
pub mod storage;
pub mod system;
pub mod world;
pub use acute_ptr as ptr;

/// Most commonly used re-exported types.
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        bundle::Bundle,
        change_detection::DetectChanges,
        component::Component,
        entity::Entity,
        event::{EventReader, EventWriter},
        query::{Added, AnyOf, ChangeTrackers, Changed, Or, QueryState, With, Without},
        schedule::{
            AmbiguitySetLabel, ExclusiveSystemDescriptorCoercion, ParallelSystemDescriptorCoercion,
            RunCriteria, RunCriteriaDescriptorCoercion, RunCriteriaLabel, Schedule, Stage,
            StageLabel, State, SystemLabel, SystemSet, SystemStage,
        },
        system::{
            Commands, In, IntoChainSystem, IntoExclusiveSystem, IntoSystem, Local, NonSend,
            NonSendMut, ParamSet, Query, RemovedComponents, Res, ResMut, System,
            SystemParamFunction,
        },
        world::{FromWorld, Mut, World},
    };
}

pub use acute_ecs_macros::all_tuples;
