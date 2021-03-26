use crate::context::WgpuContext;
use acute_app::{EventReader, Events};
use acute_window::{WindowCreatedEvent, WindowResizedEvent, Windows};
use acute_winit::window::WinitWindows;
use legion::system;
use std::ops::Deref;
use std::sync::Arc;
use wgpu::BackendBit;

pub struct WgpuRenderer {
    pub instance: wgpu::Instance,
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,
    pub window_created_event_reader: EventReader<WindowCreatedEvent>,
    pub window_resized_event_reader: EventReader<WindowResizedEvent>,
}

impl WgpuRenderer {
    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(BackendBit::PRIMARY);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                compatible_surface: None,
            })
            .await
            .expect("Unable to find a GPU");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: Default::default(),
                    limits: Default::default(),
                },
                None,
            )
            .await
            .unwrap();

        let device = Arc::new(device);

        Self {
            instance,
            device,
            queue,
            window_created_event_reader: Default::default(),
            window_resized_event_reader: Default::default(),
        }
    }
}

#[system]
pub fn surface_creation(
    #[resource] renderer: &mut WgpuRenderer,
    #[resource] context: &mut WgpuContext,
    #[resource] windows: &Windows,
    #[resource] winit_windows: &WinitWindows,
    #[resource] events: &Events<WindowCreatedEvent>,
) {
    let mut reader = events.get_reader();

    for event in reader.iter(events) {
        let window = windows
            .get(event.id)
            .expect("Received window created event for non-existent window.");

        let winit_window = winit_windows.get_window(window.id()).unwrap();
        let surface = unsafe { renderer.instance.create_surface(winit_window.deref()) };
        context.set_surface(window.id(), surface);
    }
}

#[system]
pub fn render(#[resource] context: &mut WgpuContext) {
    let resources = context.resources.read().refs();
    for (window, surface) in context.resources.surfaces.read().iter() {}
}
