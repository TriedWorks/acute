use crate::app::App;
use crate::events::event_update_system;
use crate::{Events, Plugin, PluginBundle, PluginBundleBuilder};
use acute_ecs::systems::{Builder, ParallelRunnable, Resource};
use acute_ecs::Resources;

pub struct AppBuilder {
    pub app: App,
    pub startup_system_builder: Builder,
    pub system_builder: Builder,
}

impl AppBuilder {
    pub fn build(&mut self) -> &mut Self {
        self.app.schedule = self.system_builder.build();
        self.startup_system_builder
            .build()
            .execute(&mut self.app.scene.world, &mut self.app.resources);

        self
    }

    pub fn run(&mut self) {
        let app = std::mem::take(&mut self.app);
        app.run();
    }

    pub fn set_runner(&mut self, runner: impl Fn(App) + 'static) -> &mut Self {
        self.app.runner = Box::new(runner);
        self
    }

    pub fn add_resource<T: Resource>(&mut self, resource: T) -> &mut Self {
        self.app.resources.insert(resource);
        self
    }

    pub fn add_event<T: Resource + Send + Sync>(&mut self) -> &mut Self {
        self.add_resource(Events::<T>::default())
            .add_system(event_update_system::<T>())
    }

    pub fn add_startup_system<T: ParallelRunnable + 'static>(&mut self, system: T) -> &mut Self {
        self.startup_system_builder.add_system(system);
        self
    }

    pub fn add_system<T: ParallelRunnable + 'static>(&mut self, system: T) -> &mut Self {
        self.system_builder.add_system(system);
        self
    }

    pub fn add_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        plugin.build(self);
        self
    }

    pub fn add_bundle<T: PluginBundle + Default>(&mut self) -> &mut Self {
        let mut builder = PluginBundleBuilder::default();
        let mut bundle = T::default();
        bundle.build(&mut builder);
        builder.finish(self);
        self
    }

    pub fn add_bundle_custom<T: PluginBundle>(&mut self, mut bundle: T) -> &mut Self {
        let mut builder = PluginBundleBuilder::default();
        bundle.build(&mut builder);
        builder.finish(self);
        self
    }

    pub fn resources(&self) -> &Resources {
        &self.app.resources
    }

    pub fn resources_mut(&mut self) -> &mut Resources {
        &mut self.app.resources
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            app: Default::default(),
            startup_system_builder: Default::default(),
            system_builder: Default::default(),
        }
    }
}
