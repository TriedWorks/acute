use acute_window::{
    Window as AcuteWindow, WindowDescriptor as AcuteWindowDescriptor, WindowId as AcuteWindowId,
};
use std::collections::HashMap;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::WindowId;
use winit::window::{Window, WindowBuilder};

#[derive(Debug, Default)]
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
            winit_to_window_id: Default::default(),
        }
    }

    pub fn create_window(
        &mut self,
        event_loop: &EventLoopWindowTarget<()>,
        id: AcuteWindowId,
        descriptor: &AcuteWindowDescriptor,
    ) -> AcuteWindow {
        let winit_window_builder = WindowBuilder::new()
            .with_title(descriptor.title.clone())
            .with_inner_size(PhysicalSize::new(descriptor.width, descriptor.height))
            .with_resizable(descriptor.resizable);

        let winit_window = winit_window_builder.build(event_loop).unwrap();

        self.window_id_to_winit.insert(id, winit_window.id());
        self.winit_to_window_id.insert(winit_window.id(), id);
        self.windows.insert(winit_window.id(), winit_window);

        AcuteWindow::new(id, descriptor)
    }

    pub fn get_window(&self, id: AcuteWindowId) -> Option<&Window> {
        self.window_id_to_winit
            .get(&id)
            .and_then(|id| self.windows.get(id))
    }
}
