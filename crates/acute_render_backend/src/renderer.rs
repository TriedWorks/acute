use acute_ecs::legion::{Resources, World};
use std::sync::Arc;
use acute_window::{Windows, WindowCreated};
use acute_winit::window::WinitWindows;
use acute_app::{Events, EventReader};
use crate::render_context::RenderContext;

pub struct WgpuRenderer {
    pub instance: wgpu::Instance,
    pub device: Arc<wgpu::Device>,
    pub queue: wgpu::Queue,
    pub window_created_event_reader: EventReader<WindowCreated>,
}

impl WgpuRenderer {
    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: None,
            })
            .await
            .expect("failed to find an appropriate device");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // use default limits, might change later, if it causes issues.
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .expect("failed to create device");

        let device = Arc::new(device);

        Self {
            instance,
            device,
            queue,
            window_created_event_reader: Default::default()
        }
    }

    pub fn handle_window_created_events(&mut self, resources: &Resources) {
        let mut render_context = resources.get::<RenderContext>().unwrap();
        let windows = resources.get::<Windows>().unwrap();
        let window_created_events = resources.get::<Events<WindowCreated>>().unwrap();
        for window_created_event in self
            .window_created_event_reader.iter(&window_created_events) {
            let window = windows
                .get(window_created_event.id)
                .expect("Received window created event");
            {
                let winit_windows = resources.get::<WinitWindows>().unwrap();
                let winit_window = winit_windows.get_window(window.id()).unwrap();
                let surface = unsafe { self.instance.create_surface(winit_window)};
                render_context.add_window_surface(window.id(), surface);
            }
        }
    }

    pub fn render(&mut self, world: &mut World, resources: &mut Resources) {
        let mut render_context = resources.get_mut::<RenderContext>().unwrap();
        render_context.begin_render_pass();
        render_context.encoder.finish(&mut self.queue);
    }

    pub fn update(&mut self, world: &mut World, resources: &mut Resources) {
        self.handle_window_created_events(resources);
        self.render(world, resources);

        let render_context = resources.get_mut::<RenderContext>().unwrap();
        render_context.drop_all_swap_chain_textures();
    }
}