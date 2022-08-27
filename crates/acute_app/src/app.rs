use bevy_ecs::event::Events;
use bevy_ecs::prelude::StageLabel;
use bevy_ecs::schedule::{
    IntoSystemDescriptor, Schedule, ShouldRun, Stage, State, StateData, SystemSet, SystemStage,
};
use bevy_ecs::system::{IntoExclusiveSystem, Resource};

use crate::{CoreStage, Plugin, PluginBundle, PluginBundleBuilder, StartupSchedule, StartupStage};
use bevy_ecs::world::{FromWorld, Mut, World};

pub struct App {
    pub world: World,
    pub schedule: Schedule,
    pub runner: Box<dyn Fn(App)>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn empty() -> Self {
        Self {
            world: Default::default(),
            schedule: Default::default(),
            runner: Box::new(run_once),
        }
    }

    pub fn update(&mut self) {
        self.schedule.run(&mut self.world);
    }

    pub fn run(&mut self) {
        let mut app = std::mem::replace(self, App::empty());
        let runner = std::mem::replace(&mut app.runner, Box::new(run_once));
        (runner)(app)
    }

    pub fn add_stage<S: Stage>(&mut self, label: impl StageLabel, stage: S) -> &mut Self {
        self.schedule.add_stage(label, stage);
        self
    }

    pub fn add_stage_after<S: Stage>(
        &mut self,
        target: impl StageLabel,
        label: impl StageLabel,
        stage: S,
    ) -> &mut Self {
        self.schedule.add_stage_after(target, label, stage);
        self
    }

    pub fn add_stage_before<S: Stage>(
        &mut self,
        target: impl StageLabel,
        label: impl StageLabel,
        stage: S,
    ) -> &mut Self {
        self.schedule.add_stage_before(target, label, stage);
        self
    }

    pub fn add_startup_stage<S: Stage>(&mut self, label: impl StageLabel, stage: S) -> &mut Self {
        self.schedule
            .stage(StartupSchedule, |schedule: &mut Schedule| {
                schedule.add_stage(label, stage)
            });
        self
    }

    pub fn add_startup_stage_after<S: Stage>(
        &mut self,
        target: impl StageLabel,
        label: impl StageLabel,
        stage: S,
    ) -> &mut Self {
        self.schedule
            .stage(StartupSchedule, |schedule: &mut Schedule| {
                schedule.add_stage_after(target, label, stage)
            });
        self
    }

    pub fn add_startup_stage_before<S: Stage>(
        &mut self,
        target: impl StageLabel,
        label: impl StageLabel,
        stage: S,
    ) -> &mut Self {
        self.schedule
            .stage(StartupSchedule, |schedule: &mut Schedule| {
                schedule.add_stage_before(target, label, stage)
            });
        self
    }

    pub fn stage<T: Stage, F: FnOnce(&mut T) -> &mut T>(
        &mut self,
        label: impl StageLabel,
        func: F,
    ) -> &mut Self {
        self.schedule.stage(label, func);
        self
    }

    pub fn add_system<Params>(&mut self, system: impl IntoSystemDescriptor<Params>) -> &mut Self {
        self.add_system_to_stage(CoreStage::Update, system)
    }

    pub fn add_system_set(&mut self, system_set: SystemSet) -> &mut Self {
        self.add_system_set_to_stage(CoreStage::Update, system_set)
    }

    pub fn add_system_to_stage<Params>(
        &mut self,
        stage_label: impl StageLabel,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut Self {
        use std::any::TypeId;
        assert_ne!(
            stage_label.type_id(),
            TypeId::of::<StartupStage>(),
            "add systems to a startup stage using App::add_startup_system_to_stage"
        );
        self.schedule.add_system_to_stage(stage_label, system);
        self
    }

    pub fn add_system_set_to_stage(
        &mut self,
        stage_label: impl StageLabel,
        system_set: SystemSet,
    ) -> &mut Self {
        use std::any::TypeId;
        assert_ne!(
            stage_label.type_id(),
            TypeId::of::<StartupStage>(),
            "add system sets to a startup stage using App::add_startup_system_set_to_stage"
        );
        self.schedule
            .add_system_set_to_stage(stage_label, system_set);
        self
    }

    pub fn add_startup_system<Params>(
        &mut self,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut Self {
        self.add_startup_system_to_stage(StartupStage::Startup, system)
    }

    pub fn add_startup_system_to_stage<Params>(
        &mut self,
        stage_label: impl StageLabel,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut Self {
        self.schedule
            .stage(StartupSchedule, |schedule: &mut Schedule| {
                schedule.add_system_to_stage(stage_label, system)
            });
        self
    }

    pub fn add_startup_system_set_to_stage(
        &mut self,
        stage_label: impl StageLabel,
        system_set: SystemSet,
    ) -> &mut Self {
        self.schedule
            .stage(StartupSchedule, |schedule: &mut Schedule| {
                schedule.add_system_set_to_stage(stage_label, system_set)
            });
        self
    }

    pub fn add_state<T>(&mut self, initial: T) -> &mut Self
    where
        T: StateData,
    {
        self.add_state_to_stage(CoreStage::Update, initial)
    }

    pub fn add_state_to_stage<T>(&mut self, stage: impl StageLabel, initial: T) -> &mut Self
    where
        T: StateData,
    {
        self.insert_resource(State::new(initial))
            .add_system_set_to_stage(stage, State::<T>::get_driver())
    }

    pub fn add_default_stages(&mut self) -> &mut Self {
        self.add_stage(CoreStage::First, SystemStage::parallel())
            .add_stage(
                StartupSchedule,
                Schedule::default()
                    .with_run_criteria(ShouldRun::once)
                    .with_stage(StartupStage::PreStartup, SystemStage::parallel())
                    .with_stage(StartupStage::Startup, SystemStage::parallel())
                    .with_stage(StartupStage::PostStartup, SystemStage::parallel()),
            )
            .add_stage(CoreStage::PreUpdate, SystemStage::parallel())
            .add_stage(CoreStage::Update, SystemStage::parallel())
            .add_stage(CoreStage::PostUpdate, SystemStage::parallel())
            .add_stage(CoreStage::Last, SystemStage::parallel())
    }

    pub fn add_event<T>(&mut self) -> &mut Self
    where
        T: Resource,
    {
        if !self.world.contains_resource::<Events<T>>() {
            self.init_resource::<Events<T>>()
                .add_system_to_stage(CoreStage::First, Events::<T>::update_system);
        }
        self
    }

    pub fn insert_resource<R: Resource>(&mut self, resource: R) -> &mut Self {
        self.world.insert_resource(resource);
        self
    }

    pub fn insert_non_send_resource<R: 'static>(&mut self, resource: R) -> &mut Self {
        self.world.insert_non_send_resource(resource);
        self
    }

    pub fn init_resource<R: Resource + FromWorld>(&mut self) -> &mut Self {
        self.world.init_resource::<R>();
        self
    }

    pub fn add_resource<R: Resource>(&mut self, resource: R) -> &mut Self {
        self.world.insert_resource(resource);
        self
    }

    pub fn get_resource<R: Resource>(&mut self) -> Option<&R> {
        self.world.get_resource::<R>()
    }

    pub fn get_resource_mut<R: Resource>(&mut self) -> Option<Mut<'_, R>> {
        self.world.get_resource_mut::<R>()
    }

    pub fn resource<R: Resource>(&mut self) -> &R {
        self.world.resource::<R>()
    }

    pub fn resource_mut<R: Resource>(&mut self) -> Mut<'_, R> {
        self.world.resource_mut::<R>()
    }

    pub fn init_non_send_resource<R: 'static + FromWorld>(&mut self) -> &mut Self {
        self.world.init_non_send_resource::<R>();
        self
    }

    pub fn set_runner(&mut self, run_fn: impl Fn(App) + 'static) -> &mut Self {
        self.runner = Box::new(run_fn);
        self
    }

    pub fn add_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        plugin.build(self);
        self
    }

    pub fn init_plugin<T: Plugin + Default>(&mut self) -> &mut Self {
        let plugin = T::default();
        plugin.build(self);
        self
    }

    pub fn add_plugins<T: PluginBundle>(&mut self, mut bundle: T) -> &mut Self {
        let mut builder = PluginBundleBuilder::default();
        bundle.build(&mut builder);
        builder.finish(self);
        self
    }

    pub fn init_plugins<T: PluginBundle + Default>(&mut self) -> &mut Self {
        let mut builder = PluginBundleBuilder::default();
        let mut bundle = T::default();
        bundle.build(&mut builder);
        builder.finish(self);
        self
    }
}

impl Default for App {
    fn default() -> Self {
        let mut app = App::empty();
        app.add_default_stages()
            .add_event::<AppEventExit>()
            .add_system_to_stage(CoreStage::Last, World::clear_trackers.exclusive_system());
        app
    }
}

fn run_once(mut app: App) {
    app.update();
}

#[derive(Debug, Clone, Default)]
pub struct AppEventExit;
