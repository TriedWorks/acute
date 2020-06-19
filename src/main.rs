extern crate acute;

use acute::acute::Acute;
use acute::window::WinitState;
use acute::scenes::Scene;

use winit::{dpi::LogicalSize};
use std::time::Duration;
use legion::prelude::*;

use ultraviolet::{
    Vec3,
    Rotor3,
};
use acute::components::simple::{Transform, Color};
use acute::components::geometry::{Mesh, Triangle, Vertex};


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

fn create_test_mesh() -> Mesh {
    let triangle00: Triangle = Triangle::new(&[
        Vertex {position: [-10.0, -10.0, 0.0]},
        Vertex {position: [10.0, -10.0, 0.0]},
        Vertex {position: [0.0, 10.0, 0.0]},
    ]);

    let triangle01: Triangle = Triangle::new(&[
        Vertex {position: [0.0, 10.0, 0.0]},
        Vertex {position: [10.0, -10.0, 0.0]},
        Vertex {position: [10.0, 10.0, -10.0]},
    ]);

    let triangle1: Triangle = Triangle::new(&[
        Vertex {position: [10.0, 10.0, 0.0]},
        Vertex {position: [30.0, 10.0, 0.0]},
        Vertex {position: [20.0, 30.0, 0.0]},
    ]);

    let triangles = vec![triangle00, triangle01, triangle1];
    Triangle::new_mesh(&triangles)
}

struct TestScene {}

impl TestScene {
    fn new() -> Self {
        Self {}
    }
}

impl Scene for TestScene {
    fn update(&mut self, world: &mut World, delta_time: &Duration) {

    }

    fn fixed_update(&mut self, world: &mut World, delta_time: &Duration) {
        let transform_query = <Write<Transform>>::query();

        for mut transform in transform_query.iter(world) {
            transform.position[0] += 1.0 * delta_time.as_secs_f32();
            transform.rotation += Rotor3::from_rotation_xy(2.0 * delta_time.as_secs_f32());
            transform.rotation.normalize();
        }
        let mesh_query = <(Write<Mesh>, Read<Transform>)>::query();

        for (mut mesh, transform) in mesh_query.iter(world) {
            mesh.update_rotation(transform.rotation);
        }
    }

    fn on_start(&mut self, world: &mut World) {
        world.insert(
            (),
            (0..1).map(|_| (
                Transform {
                    position: Vec3::new(0.0, 0.0, 0.0),
                    rotation: Rotor3 { s: 0.0, bv: Default::default() },
                },
                create_test_mesh(),
                Color { data: [0.1, 0.4, 0.3, 1.0]}
            )),
        );
    }
}