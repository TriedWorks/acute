use crate::app::App;
use acute_ecs::legion::systems::{Builder, ParallelRunnable, Resource};
use crate::Plugin;

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

    pub fn add_startup_system<T: ParallelRunnable + 'static>(&mut self, system: T) -> &mut Self {
        self.startup_system_builder.add_system(system);
        self
    }

    pub fn add_system<T: ParallelRunnable + 'static>(&mut self, system: T) -> &mut Self {
        self.system_builder.add_system(system);
        self
    }

    pub fn add_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        plugin.add(self);
        self
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
