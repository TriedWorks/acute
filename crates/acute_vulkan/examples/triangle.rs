use acute_vulkan::internal::ash;
use acute_vulkan::internal::driver::*;
use acute_winit::internal::winit::dpi::PhysicalSize;
use acute_winit::internal::winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use acute_winit::internal::winit::event_loop::{ControlFlow, EventLoop};
use acute_winit::internal::winit::window::WindowBuilder;
use ash::vk;
use std::sync::atomic::fence;

fn main() {
    acute_tracing::init_tracing_extern();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Triangle Test")
        .with_inner_size(PhysicalSize::new(1080, 720))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    let instance = Instance::new(InstanceCreateInfo {
        application_name: Some("Testing".to_string()),
        engine_name: Some("Acute".to_string()),
        vulkan_version: Version::V1_3,
        render: true,
        ..InstanceCreateInfo::default()
    })
    .unwrap();

    let adapter = instance.adapters().next().unwrap();
    let mut surface = instance.create_surface(&window).unwrap();

    let graphics_family = adapter
        .queue_families()
        .find(|queue| queue.supports_graphics())
        .unwrap();

    let transfer_family = adapter
        .queue_families()
        .find(|queue| queue.supports_transfer() && !queue.supports_graphics())
        .unwrap();

    let (device, mut queues) = adapter
        .request_device(DeviceCreateInfo {
            queue_families: vec![
                QueueCreateInfo::new(graphics_family, vec![1.0]),
                QueueCreateInfo::new(transfer_family, vec![0.5]),
            ],
            ..DeviceCreateInfo::default()
        })
        .unwrap();

    let format = surface.formats(&adapter).unwrap().next().unwrap();
    let mut queue = queues.next().unwrap();

    surface
        .configure(
            &device,
            &SurfaceConfig {
                usage: TextureUsages::COLOR_ATTACHMENT,
                format,
                width: window.inner_size().width,
                height: window.inner_size().height,
                mode: PresentMode::Mailbox,
            },
        )
        .unwrap();

    let present_semaphore = device.create_binary_semaphore();
    let render_semaphore = device.create_binary_semaphore();
    let render_fence = device.create_fence();

    event_loop.run(move |event, event_loop, control_flow| match event {
        Event::WindowEvent { event, window_id } => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput { ref input, .. } => {
                if let Some(key) = input.virtual_keycode {
                    if key == VirtualKeyCode::Escape && input.state == ElementState::Pressed {
                        *control_flow = ControlFlow::Exit
                    }
                }
            }
            _ => {}
        },
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            render_fence.wait_reset(1000).unwrap();

            let frame = surface
                .acquire_frame(1000, Some(&present_semaphore), None)
                .unwrap()
                .unwrap();

            let mut encoder = device.command_encoder(CommandEncoderInfo { queue: &queue });

            encoder.begin_encoding();

            encoder.frame_transition(
                ImageTransitionLayout::Undefined,
                ImageTransitionLayout::ColorAttachment,
                &frame,
            );
            encoder.begin_rendering(RenderInfo {
                color_attachments: &[RenderAttachmentInfo {
                    load_op: LoadOp::Clear,
                    store_op: StoreOp::Store,
                    clear: ClearOp::Color(Color::RED),
                }],
                frame: &frame,
                offset: (0, 0),
                area: (window.inner_size().width, window.inner_size().height),
            });

            encoder.end_rendering();

            encoder.frame_transition(
                ImageTransitionLayout::ColorAttachment,
                ImageTransitionLayout::Present,
                &frame,
            );

            queue
                .submit(
                    [encoder.finish()],
                    &[&render_semaphore],
                    &[&present_semaphore],
                    Some(&render_fence),
                )
                .unwrap();

            frame
                .present(&queue, &surface, &[&render_semaphore])
                .unwrap();
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
