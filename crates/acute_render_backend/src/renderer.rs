use acute_ecs::legion::prelude::*;
use acute_window::winit::window::Window;

pub struct Renderer {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc: wgpu::SwapChain,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub window: Window,
    pub surface: wgpu::Surface,
}

impl Renderer {
    pub async fn new(window: Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let (size, surface) = unsafe {
            let size = window.inner_size();
            let surface = instance.create_surface(&window);
            (size, surface)
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

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
            .unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            // set this to Fifo to enable "vsync"
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let sc = device.create_swap_chain(&surface, &sc_desc);

        Self {
            instance,
            adapter,
            device,
            queue,
            sc,
            sc_desc,
            window,
            surface,
        }
    }
}
