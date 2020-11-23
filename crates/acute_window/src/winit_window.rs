use winit::dpi::LogicalSize;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::{Fullscreen, Window, WindowBuilder};
use legion::*;
use winit::event::{WindowEvent, Event};
use acute_app::{RenderEvent, Events};

pub struct WindowDescriptor {
    title: String,
    size: LogicalSize<u32>,
    fullscreen: Option<Fullscreen>,
    resizable: bool,
}

impl Default for WindowDescriptor {
    fn default() -> Self {
        Self {
            title: "Acute".to_string(),
            size: LogicalSize {
                width: 1280,
                height: 720,
            },
            fullscreen: None,
            resizable: false,
        }
    }
}

pub struct WinitWindow;

impl WinitWindow {
    pub fn new(window_desc: WindowDescriptor, event_loop: &EventLoop<()>) -> Window {
        let window = WindowBuilder::new()
            .with_title(window_desc.title)
            .with_inner_size(window_desc.size)
            .with_fullscreen(window_desc.fullscreen)
            .with_resizable(window_desc.resizable)
            .build(&event_loop)
            .unwrap();
        window
    }
}