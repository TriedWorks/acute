use acute::prelude::*;
use acute_input::Input;
use acute_window::event::{VirtualKeyCode, MouseButton};

fn main() {
    let (window, event_loop) = WinitWindow::new(WindowDescriptor::default());
    // let event_loop = WinitWindow::new_headless();
    let mut app = App::builder()
        .with_defaults(window)
        // .with_defaults_headless()
        .add_system(test_input())
        .build();

    event_loop.run(move |event, _event_loop, mut control_flow| {
        app.run(&event, &mut control_flow);
    })
}

fn test_input() -> Box<dyn Schedulable> {
    SystemBuilder::new("TestPrintSystem")
        .read_resource::<Input>()
        .build(|_, _, input, _| {
            if input.keyboard.just_pressed(VirtualKeyCode::Space) {
                println!("Pressed Space")
            }
            if input.mouse.just_pressed(MouseButton::Left) {
                println!("click!")
            }
        })
}

fn test_print() -> Box<dyn Schedulable> {
    SystemBuilder::new("TestPrintSystem").build(|_, _, _, _| println!("Test"))
}

fn test_timer() -> Box<dyn Schedulable> {
    SystemBuilder::new("PrintTimeSystem")
        .read_resource::<Timer>()
        .build(move |_, _, timer, _| println!("{:?}", timer.delta_time()))
}
