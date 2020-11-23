use acute_ecs::legion::systems::Resource;
use acute_ecs::legion::*;

use acute_scenes::Scene;

use crate::builder::AppBuilder;
use crate::State;
use rusty_timer::Timer;
use std::time::Duration;

pub struct App {
    pub resources: Resources,
    pub schedule: Schedule,
    pub render_schedule: Schedule,
    pub scene: Scene,
    pub timer: Timer,
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
        loop {
            self.timer.update_delta_time();
            self.timer.update_fixed_time();
            if self.timer.should_fixed_update() {
                self.schedule.execute(&mut self.scene.world, &mut self.resources);
                self.render_schedule.execute(&mut self.scene.world, &mut self.resources);
            }
        }
        // if let Some(mut input) = self.resources.get_mut::<Input>() {
        //     // input.update(event);
        //     if input.keyboard.pressed(VirtualKeyCode::LAlt)
        //         && input.keyboard.pressed(VirtualKeyCode::F4)
        //     {
        //         // *control_flow = ControlFlow::Exit;
        //     }
        // }
        // match event {
        //     WinitEvent::WindowEvent { event, .. } => match event {
        //         WindowEvent::CloseRequested => {
        //             *control_flow = ControlFlow::Exit;
        //         }
        //         WindowEvent::Resized(size) => {
        //             if let Some(mut renderer) = self.resources.get_mut::<WgpuRenderer>() {
        //                 renderer.resize(size)
        //             }
        //         }
        //         _ => {}
        //     },
        //     WinitEvent::RedrawRequested(_) => {
        //         self.render_schedule
        //             .execute(&mut self.scene.world, &mut self.resources);
        //     }
        //     WinitEvent::MainEventsCleared => {
        //         if let Some(window) = self.resources.get::<Window>() {
        //             window.request_redraw();
        //         }
        //     }
        //     _ => {}
        // }

        self.schedule
            .execute(&mut self.scene.world, &mut self.resources);
        self.scene.update(&mut self.resources);
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
            timer: Default::default()
        }
    }
}
