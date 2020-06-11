use super::texture;

pub struct Renderer {
    pub surface: wgpu::Surface,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub adapter: wgpu::Adapter,
    pub swap_chain: wgpu::SwapChain,
    pub window: winit::window::Window,
}

impl Renderer {
    pub async fn new(
        window: winit::window::Window,
        size: winit::dpi::PhysicalSize<u32>,
        resources: &mut legion::resource::Resources,
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

        resources.insert(sc_desc);
        resources.insert(queue);
        resources.insert(device);
        resources.insert(depth_texture);

        Self {
            surface,
            size,
            adapter,
            swap_chain,
            window,
        }
    }
}