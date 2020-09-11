use acute_window::winit::dpi::PhysicalSize;
use acute_window::winit::window::Window;
use crate::resources::WgpuResources;
use acute_ecs::legion::Resources;
use std::ops::Deref;

pub struct WgpuRenderer {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub resources: WgpuResources
}

impl WgpuRenderer {
    pub async fn new(resources: &mut Resources) -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let window = resources.get::<Window>().unwrap();

        let surface = unsafe {
            let surface = instance.create_surface(window.deref());
            surface
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("failed to find an appropriate device");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: adapter.features(),
                    // use default limits, might change later, if it causes issues.
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .expect("failed to create device");
        {}
        let resources= WgpuResources::new(resources, surface, &device);

        Self {
            instance,
            adapter,
            device,
            queue,
            resources,
        }
    }

    pub fn resize(&mut self, size: &PhysicalSize<u32>) {
        self.resources.sc_desc.width = size.width;
        self.resources.sc_desc.height = size.height;
        self.resources.swap_chain = self.device.create_swap_chain(&self.resources.surface, &self.resources.sc_desc);
    }
}