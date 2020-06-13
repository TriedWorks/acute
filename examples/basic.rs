extern crate acute;

use acute::application::Acute;
use acute::window::WinitState;

use winit::dpi::LogicalSize;


const WINDOW_SIZE: LogicalSize<u32> = LogicalSize { width: 1280, height: 720 };

fn main() {
    let (window_builder, event_loop) = WinitState::new("Acute Test", WINDOW_SIZE);
    let acute: Acute = Acute::new(window_builder, &event_loop, None);
    event_loop.run(move |event, _, control_flow| {})
}