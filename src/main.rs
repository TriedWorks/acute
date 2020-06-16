extern crate acute;

use acute::acute::Acute;
use acute::window::WinitState;
use acute::scenes::Scene;

use winit::{event::Event, event::WindowEvent, dpi::LogicalSize};
use winit::event_loop::ControlFlow;
use winit::window::WindowBuilder;
use std::time::Duration;
use legion::world::World;

use legion::entity::Entity;
use ultraviolet::{
    Vec3,
    Rotor3,
};
use acute::components::simple::{Transform, Color};
use acute::components::geometry::Triangle2D;


const WINDOW_SIZE: LogicalSize<u32> = LogicalSize { width: 1280, height: 720 };
const TITLE: &str = "Acute Test";

fn main() {
    let (window_builder, event_loop) = WinitState::new(TITLE, WINDOW_SIZE);
    let test_scene = TestScene::new();

    let mut acute: Acute = Acute::new(window_builder, Some(Box::new(test_scene)), &event_loop);

    event_loop.run(move |event, _, mut control_flow| {
        acute.run(&event, &mut control_flow);
    });
}

struct TestScene {}

impl TestScene {
    fn new() -> Self {
        Self {}
    }
}

impl Scene for TestScene {
    fn update(&mut self, world: &mut World, delta_time: &Duration) {}

    fn fixed_update(&mut self, world: &mut World, delta_time: &Duration) {}

    fn on_start(&mut self, world: &mut World) {
        println!("init_world");
        for i in 0..5 {
            world.insert(
                (),
                (0..1).map(|_| (
                    Transform {
                        pos: Vec3::new((2 * i) as f32, 0.0, i as f32),
                        scale: Vec3::default(),
                        rotation: Rotor3 { s: 0.0, bv: Default::default() },
                    },
                    Triangle2D {
                        a: Vec3::new((-i * 2) as f32, (-i * 2) as f32, i as f32),
                        b: Vec3::new((i * 2) as f32, (-i * 2) as f32, i as f32),
                        c: Vec3::new(0.0, (i * 2) as f32, 0.0),
                    },
                    Color { data: [1.0 / i as f32, 1.0 / (2 * i) as f32, 1.0 / ( 2 * i) as f32, 1.0]}
                )),
            );
        }
    }
}