use crate::app::App;
use acute_ecs::legion::prelude::*;
use acute_ecs::legion::systems::{resource::Resource, schedule::Builder};
use acute_render_backend::Renderer;
use acute_window::winit::window::Window;

pub struct AppBuilder {
    pub app: App,
    pub system_builder: Builder,
    pub render_system_builder: Builder,
    pub window: Option<Window>,
}

impl AppBuilder {
    pub fn build(mut self) -> App {
        let renderer = match self.window {
            Some(window) => Some(futures::executor::block_on(Renderer::new(window))),
            None => None,
        };
        if let Some(renderer) = renderer {
            self.app.resources.insert(renderer);
        }

        self.app.schedule = self.system_builder.build();
        self.app.render_schedule = self.render_system_builder.build();

        self.app
    }

    pub fn with_window(mut self, window: Window) -> Self {
        self.window = Some(window);
        self
    }

    pub fn add_resource<T: Resource>(mut self, resource: T) -> Self {
        self.app.resources.insert(resource);
        self
    }

    pub fn add_system(mut self, system: Box<dyn Schedulable>) -> Self {
        self.system_builder = self.system_builder.add_system(system);
        self
    }

    pub fn add_render_system(mut self, render_system: Box<dyn Schedulable>) -> Self {
        self.system_builder = self.system_builder.add_system(render_system);
        self
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            app: Default::default(),
            system_builder: Default::default(),
            render_system_builder: Default::default(),
            window: None,
        }
    }
}
