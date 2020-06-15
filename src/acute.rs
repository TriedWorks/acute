use legion::prelude::*;
use winit::{
    window::WindowBuilder,
    event_loop::{EventLoop, ControlFlow},
    event::Event,
};

use rusty_timer::Timer;
use crate::graphics::renderer::Renderer;
use crate::scenes::{SceneHandler, Scene};
use std::time::Duration;

pub struct Acute {
    universe: Universe,
    worlds: Vec<World>,
    scene_handler: SceneHandler,
    renderer: Renderer,
    timer: Timer
}

impl Acute {
    pub fn new(
        window_builder: WindowBuilder,
        init_scene: Option<Box<dyn Scene>>,
        event_loop: &EventLoop<()>
    ) -> Self {
        let window = window_builder.build(&event_loop).unwrap();
        let size = window.inner_size();
        let universe = Universe::new();
        let mut worlds: Vec<World> = Vec::new();
        worlds.push(universe.create_world());
        let mut scene_handler: SceneHandler = SceneHandler::new(init_scene);
        scene_handler.init_world_data(&mut worlds[0]);

        let renderer = futures::executor::block_on(
            Renderer::new(window, size)
        );


        let mut timer: Timer = Timer::new();
        timer.set_fixed_interval(Duration::from_secs_f32(1.0 / 60.0));

        Self {
            universe,
            worlds,
            scene_handler,
            renderer,
            timer,
        }
    }

    pub fn run(&mut self) {
        self.timer.update_delta_time();
        self.timer.update_time_since_last_fixed_update();

        self.scene_handler.update(&mut self.worlds[0], &self.timer.delta_time());

        if self.timer.should_fixed_update() {
            self.scene_handler.fixed_update(&mut self.worlds[0], &self.timer.delta_time());
        }

        // self.renderer.
    }

    pub fn add_scene(&mut self, scene: Box<dyn Scene>) {
        self.scene_handler.add_scene(scene);
    }

    pub fn new_world(&mut self) {
        self.worlds.push(self.universe.create_world());
    }
}