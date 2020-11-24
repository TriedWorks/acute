use acute::prelude::*;
use acute_assets::{AssetKind, Assets};
use acute_input::{MouseButton, Key};

fn main() {
    App::builder()
        .with_defaults()
        .add_system(test_input_system())
        .build()
        .run();
}


// #[system]
// fn test_assets(#[resource] assets: &mut Assets) {
//     let img = assets.add("cat.png", AssetKind::Image);
// }

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

#[system]
fn test_print() {
    println!("Test");
}
