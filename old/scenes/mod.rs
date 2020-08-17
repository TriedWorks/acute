use legion::prelude::*;
use std::any::Any;
use std::time::Duration;

pub struct SceneHandler {
    scenes: Vec<Box<dyn Scene>>,
}

impl SceneHandler {
    pub fn new(init_scene: Option<Box<dyn Scene>>) -> Self {
        let mut scenes = Vec::new();
        match init_scene {
            Some(scene) => scenes.push(scene),
            None => scenes.push(Box::new(NoneScene::new())),
        }
        Self { scenes }
    }

    pub fn update(&mut self, world: &mut World, delta_time: &Duration) {
        match self.scenes.last_mut() {
            Some(scene) => scene.update(world, delta_time),
            None => {}
        }
    }

    pub fn fixed_update(&mut self, world: &mut World, delta_time: &Duration) {
        match self.scenes.last_mut() {
            Some(scene) => scene.fixed_update(world, delta_time),
            None => {}
        }
    }

    pub fn add_scene(&mut self, scene: Box<dyn Scene>) {
        self.scenes.push(scene);
    }

    pub fn init_world_data(&mut self, world: &mut World) {
        match self.scenes.last_mut() {
            Some(scene) => scene.on_start(world),
            None => {}
        }
    }

    pub fn push_front(&mut self, scene_id: u32) {
        // let scenes = self.scenes
    }
}

pub trait Scene: Any {
    fn update(&mut self, world: &mut World, delta_time: &Duration) {}

    fn fixed_update(&mut self, world: &mut World, delta_time: &Duration) {}

    fn on_start(&mut self, world: &mut World) {}

    fn on_stop(&mut self, world: &mut World) {}
}

pub struct NoneScene {}

impl NoneScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for NoneScene {
    fn update(&mut self, world: &mut World, delta_time: &Duration) {
        println!("update");
    }

    fn fixed_update(&mut self, world: &mut World, delta_time: &Duration) {
        println!("fixed_update");
    }
}
