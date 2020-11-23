use acute_ecs::legion::*;

use acute_scenes::Scene;

use crate::builder::AppBuilder;
use crate::State;
use std::time::Duration;

pub struct App {
    pub resources: Resources,
    pub schedule: Schedule,
    pub scene: Scene,
    pub runner: Box<dyn Fn(App)>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> AppBuilder {
        AppBuilder::default()
    }

    pub fn update(&mut self) {
        self.schedule.execute(&mut self.scene.world, &mut self.resources);
    }

    pub fn run(mut self) {
        let runner = std::mem::replace(&mut self.runner, Box::new(App::run_once));
        (runner)(self)
    }

    fn run_once(mut self) {
        self.schedule.execute(&mut self.scene.world, &mut self.resources);
    }
}

impl Default for App {
    fn default() -> Self {
        let scene = Scene::new(None);
        Self {
            resources: Default::default(),
            schedule: Schedule::builder().build(),
            scene,
            runner: Box::new(App::run_once)
        }
    }
}
