use acute_ecs::legion::systems::Resource;
use acute_ecs::legion::*;

use acute_scenes::Scene;

use crate::builder::AppBuilder;
use crate::State;
use rusty_timer::Timer;
use std::time::Duration;
use winit::event_loop::EventLoop;

pub struct App {
    pub resources: Resources,
    pub schedule: Schedule,
    pub render_schedule: Schedule,
    pub scene: Scene,
    pub timer: Timer,
    pub event_loop: Option<EventLoop<()>>
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> AppBuilder {
        AppBuilder::default()
    }

    pub fn run(&mut self) {
        self.timer.set_fixed_interval(Duration::from_secs_f32(0.01666666666));
        let event_loop = self.event_loop.take().unwrap();
        event_loop.run(move |event, window, control_flow| {

        })
    }
}

impl Default for App {
    fn default() -> Self {
        let scene = Scene::new(None);
        Self {
            resources: Default::default(),
            schedule: Schedule::builder().build(),
            render_schedule: Schedule::builder().build(),
            scene,
            timer: Default::default(),
            event_loop: Some(EventLoop::new())
        }
    }
}
