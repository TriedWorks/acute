extern crate acute;

use winit::{dpi::LogicalSize};
use std::time::{Duration, Instant};
use legion::prelude::*;
use acute::{
    acute::Acute,
    window::WinitState,
    scenes::Scene,
    components::{
        geometry::{Mesh, Cuboid, Quad, Triangle, Vertex},
        default::{
            Transform, Color, Static, Group
        }
    },
};
use ultraviolet::{
    Vec3,
    Rotor3,
};

const WINDOW_SIZE: LogicalSize<u32> = LogicalSize { width: 1280, height: 720 };
const TITLE: &str = "Acute Test";

#[derive(Copy, Clone, PartialEq)]
struct UpDown;

fn main() {
    let (window_builder, event_loop) = WinitState::new(TITLE, WINDOW_SIZE);
    let test_scene = TestScene::new();

    let mut acute: Acute = Acute::new(window_builder, Some(Box::new(test_scene)), &event_loop);

    event_loop.run(move |event, _, mut control_flow| {
        acute.run(&event, &mut control_flow);
    });
}

struct TestScene {
    time_since_start: Instant,
}

impl TestScene {
    fn new() -> Self {
        Self {
            time_since_start: Instant::now()
        }
    }
}

impl Scene for TestScene {
    fn update(&mut self, world: &mut World, delta_time: &Duration) {

    }

    fn fixed_update(&mut self, world: &mut World, delta_time: &Duration) {
        let move_query = <Write<Transform>>::query().filter(tag::<UpDown>());

        for (mut transform) in move_query.iter_mut(world) {
            transform.position[1] += self.time_since_start.elapsed().as_secs_f32().sin() * 0.01;
        }

        let rotate_xy_query = <Write<Transform>>::query().filter(tag_value(&Group("rotate_xy")));
        for (mut transform) in rotate_xy_query.iter_mut(world) {
            transform.rotation += Rotor3::from_rotation_xy(12.0*delta_time.as_secs_f32());
            transform.rotation.normalize();
        }

        let rotate_xz_query = <Write<Transform>>::query().filter(tag_value(&Group("rotate_xyxz")));
        for (mut transform) in rotate_xz_query.iter_mut(world) {
            transform.rotation += Rotor3::from_rotation_xz(12.0*delta_time.as_secs_f32());
            transform.rotation += Rotor3::from_rotation_xy(12.0*delta_time.as_secs_f32());
            transform.rotation.normalize();
        }

        let mesh_query = <(Write<Mesh>, Read<Transform>)>::query().filter(changed::<Transform>());

        for (mut mesh, transform) in mesh_query.iter_mut(world) {
            mesh.update_rotation(transform.rotation);
        }
    }

    fn on_start(&mut self, world: &mut World) {
        // create ground, and mark as static
        // TODO: add scaling so one can be used
        for i in -10..10 {
            for j in -10..20 {
                let entity = world.insert(
                    (),
                    std::iter::once((
                        Transform {
                            position: Vec3::new((i as f32), 0.0, (j as f32)),
                            rotation: Rotor3::identity(),
                        },
                        Mesh::new_mesh(&Mesh::default_horizontal_quad().to_vec()),
                        Color { data: [0.3, 0.5, 0.7, 1.0]}
                    )))[0];

                world.add_tag(entity, Static).unwrap();
            }
        }

        // insert a cube with the up_down movement tag
        {
            let entity = world.insert(
                (),
                std::iter::once((
                    Transform {
                        position: Vec3::new(0.0, -2.0, 0.0),
                        rotation: Rotor3::identity(),
                    },
                    Mesh::new_mesh(&Mesh::default_cuboid().to_vec()),
                    Color { data: [0.2, 0.8, 0.3, 1.0] }
                )))[0];

            world.add_tag(entity, UpDown).unwrap();
            world.add_tag(entity, Group("rotate_xyxz")).unwrap();
        }

        // insert a quad with the rotate_xy tag
        {
            let entity = world.insert(
                (),
                std::iter::once((
                    Transform {
                        position: Vec3::new(1.0, -1.5, -4.0),
                        rotation: Rotor3::identity(),
                    },
                    Mesh::new_mesh(&Mesh::default_quad().to_vec()),
                    Color { data: [0.5, 0.4, 0.3, 1.0]}
                )))[0];

            world.add_tag(entity, Group("rotate_xy")).unwrap();
        }

        // rotate the vertices of each entity in the beginning for not identity cases
        let rotation_query = <(Write<Mesh>, Read<Transform>)>::query();
        for (mut mesh, transform) in rotation_query.iter_mut(world) {
            mesh.update_rotation(transform.rotation);
        }
    }
}