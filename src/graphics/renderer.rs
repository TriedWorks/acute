use super::texture;

pub struct Renderer {
    pub surface: wgpu::Surface,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub depth_texture: texture::Texture,
    pub window: winit::window::Window,
}

impl Renderer {
    pub async fn new(
        window: winit::window::Window,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> Self {
        let surface = wgpu::Surface::create(&window);
        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        )
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: wgpu::Limits::default(),
            })
            .await;

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let depth_texture = texture::Texture::new_depth(&device, &sc_desc, "depth_texture");

        Self {
            surface,
            size,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            depth_texture,
            window,
        }
    }
}