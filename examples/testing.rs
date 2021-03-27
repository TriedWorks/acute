use acute::prelude::*;
use acute::ecs::system;
use acute::input::{Key, Mouse, MouseButton, Keyboard};

fn main() {
    App::builder()
        .add_bundle::<DefaultBundle>()
        .add_system(test_input_system())
        .build()
        .run();
}

#[system]
fn test_input(#[resource] keyboard: &Keyboard, #[resource] mouse: &Mouse) {
    if keyboard.just_pressed(Key::Space) {
        println!("Pressed Space")
    }
    if mouse.just_pressed(MouseButton::Left) {
        println!("click at {} | {}!", mouse.position.0, mouse.position.1)
    }
}
