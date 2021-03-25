use acute::prelude::*;
use acute_assets::{AssetKind, Assets, Image};
use acute_ecs::system;
use acute_input::{Key, MouseButton};

fn main() {
    App::builder()
        .with_defaults()
        .add_system(test_input_system())
        .build()
        .run();
}

#[system]
fn test_assets(#[resource] assets: &mut Assets, #[resource] timer: &Timer) {
    let img_id = assets.load("cat.png", AssetKind::Image);
    let img = assets.get::<Image>(&img_id).unwrap();

    if timer.time_since_start().as_secs() % 3 == 0 {
        println!("id_1: {:?}", img_id);
        println!("img: {:?}", img.dimensions());
    }
}

#[system]
fn test_input(#[resource] input: &Input) {
    if input.keyboard.just_pressed(Key::Space) {
        println!("Pressed Space")
    }
    if input.mouse.just_pressed(MouseButton::Left) {
        println!(
            "click at {} | {}!",
            input.mouse.position.0, input.mouse.position.1
        )
    }
}
