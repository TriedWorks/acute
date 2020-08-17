use acute_ecs::prelude::*;
use acute_scenes::Scene;
use acute_window::winit::event::Event as WindowEvent;
use acute_window::winit::event_loop::{ControlFlow};
use acute_ecs::systems::resource::Resource;
use super::builder::AppBuilder;
use crate::State;
use acute_render_backend::Renderer;

pub struct App {
    pub universe: Universe,
    pub resources: Resources,
    pub schedule: Schedule,
    pub render_schedule: Schedule,
    pub scene: Scene,
    pub renderer: Option<Renderer>
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn builder() -> AppBuilder {
        AppBuilder::default()
    }

    pub fn run(&mut self, _event: WindowEvent<()>, _control_flow: &mut ControlFlow) {
        self.schedule.execute(&mut self.scene.world, &mut self.resources);
        self.scene.update(&mut self.resources);
        self.render_schedule.execute(&mut self.scene.world, &mut self.resources);
    }

    pub fn run_with_state<T: State>(&mut self, state: &mut T, _event: WindowEvent<()>, _control_flow: &mut ControlFlow) {
        state.update( self);
        state.update_fixed(self);
        self.schedule.execute(&mut self.scene.world, &mut self.resources);
        self.scene.update(&mut self.resources);
        self.render_schedule.execute(&mut self.scene.world, &mut self.resources);
    }

    pub fn add_resource<T: Resource>(&mut self, resource: T) {
        self.resources.insert(resource);
    }
}

impl Default for App {
    fn default() -> Self {
        let universe = Universe::new();
        let scene = Scene::new(&universe, None);
        Self {
            universe,
            resources: Default::default(),
            schedule: Schedule::builder().build(),
            render_schedule: Schedule::builder().build(),
            scene,
            renderer: None
        }
    }
}