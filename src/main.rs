use acute::prelude::*;

struct AppState {}

impl State for AppState {}

fn main() {
    let (window, event_loop) = WinitWindow::new(WindowDescriptor::default());

    let mut app = App::builder()
        .add_resource(Timer::new())
        .add_system(print_test())
        .add_system(update_timer())
        .add_system(print_time())
        .build();

    event_loop.run(move |event, event_loop, mut control_flow| {
        app.run(event, &mut control_flow);
    })
}

fn print_test() -> Box<dyn Schedulable> {
    SystemBuilder::new("TestPrintSystem").build(|_, _, _, _| println!("Test"))
}

fn print_time() -> Box<dyn Schedulable> {
    SystemBuilder::new("PrintTimeSystem")
        .read_resource::<Timer>()
        .build(move |_,  _, timer, _| {
            println!("{:?}", timer.delta_time())
        })
}

fn update_timer() -> Box<dyn Schedulable> {
    SystemBuilder::new("UpdateTimer")
        .write_resource::<Timer>()
        .build(move |_, _, timer, _ | {
            timer.update_delta_time();
            timer.update_fixed_time();
        })
}