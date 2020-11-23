use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
pub use winit::{self, *};

pub struct WinitState {
    pub event_loop: EventLoop<()>,
    pub window: Window,
}

impl WinitState {
    pub fn new<T: Into<String>>(
        title: T,
        size: LogicalSize<u32>,
    ) -> (WindowBuilder, EventLoop<()>) {
        let event_loop = EventLoop::new();
        (
            WindowBuilder::new().with_title(title).with_inner_size(size),
            event_loop,
        )
    }
}
