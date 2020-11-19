use crate::app::App;
use acute_ecs::legion::systems::{Builder, ParallelRunnable, Resource};
use acute_render_backend::WgpuRenderer;
use acute_window::winit::window::Window;

pub struct AppBuilder {
    pub app: App,
    pub startup_system_builder: Builder,
    pub system_builder: Builder,
    pub render_system_builder: Builder,
}

impl AppBuilder {
    pub fn build(mut self) -> App {
        self.app.schedule = self.system_builder.build();
        let mut startup = self.startup_system_builder.build();
        startup.execute(&mut self.app.scene.world, &mut self.app.resources);
        if self.app.resources.contains::<Window>() {
            let mut renderer = futures::executor::block_on(WgpuRenderer::new(&mut self.app.resources));
            renderer.resources.with_testing(&renderer.device);
            self.app.render_schedule = self.render_system_builder.build();
            self.app.resources.insert(renderer);
        }

        self.app
    }

    pub fn add_resource<T: Resource>(mut self, resource: T) -> Self {
        self.app.resources.insert(resource);
        self
    }

    pub fn add_startup_system<T: ParallelRunnable + 'static>(mut self, system: T) -> Self {
        self.startup_system_builder.add_system(system);
        self
    }

    pub fn add_system<T: ParallelRunnable + 'static>(mut self, system: T) -> Self {
        self.system_builder.add_system(system);
        self
    }

    pub fn add_render_system<T: ParallelRunnable + 'static>(mut self, system: T) -> Self {
        self.render_system_builder.add_system(system);
        self
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            app: Default::default(),
            startup_system_builder: Default::default(),
            system_builder: Default::default(),
            render_system_builder: Default::default(),
        }
    }
}
