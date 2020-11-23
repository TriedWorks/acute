use std::collections::HashMap;
use acute_window::{WindowId as AcuteWindowId, Window as AcuteWindow};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::{WindowBuilder, Window};
use winit::dpi::PhysicalSize;
use winit::window::WindowId;

pub struct WinitWindows {
    pub windows: HashMap<WindowId, Window>,
    pub window_id_to_winit: HashMap<AcuteWindowId, WindowId>,
    pub winit_to_window_id: HashMap<WindowId, AcuteWindowId>,
}

impl WinitWindows {
    pub fn new() -> Self {
        Self {
            windows: Default::default(),
            window_id_to_winit: Default::default(),
            winit_to_window_id: Default::default()
        }
    }

    pub fn create_window(
        &mut self,
        window: &AcuteWindow,
        event_loop: &EventLoopWindowTarget<()>,
    ) {
        let winit_window_builder = WindowBuilder::new()
            .with_title(window.title())
            .with_inner_size(PhysicalSize::new(
                window.width(),
                window.height()
            ))
            .with_resizable(window.resizable());

        let winit_window = winit_window_builder.build(event_loop).unwrap();

        self.window_id_to_winit.insert(window.id(), winit_window.id());
        self.winit_to_window_id.insert(winit_window.id(), window.id());
        self.windows.insert(winit_window.id(), winit_window);
    }

    pub fn get_window(&self, id: AcuteWindowId) -> Option<&Window> {
        self.window_id_to_winit
            .get(&id)
            .and_then(|id| self.windows.get(id))
    }
}