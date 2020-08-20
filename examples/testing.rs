use acute::prelude::*;

fn main() {
    let (window, event_loop) = WinitWindow::new(WindowDescriptor::default());
    // let event_loop = WinitWindow::new_headless();
    let mut app = App::builder()
        .with_defaults(window)
        // .with_defaults_headless()
        .add_system(test_input())
        // .add_system(test_timer())
        .add_render_system(test_render())
        .build();

    event_loop.run(move |event, _event_loop, mut control_flow| {
        app.run(&event, &mut control_flow);
    })
}

fn test_render() -> Box<dyn Schedulable> {
    SystemBuilder::new("TestRenderSystem")
        .write_resource::<Renderer>()
        .build(move |_, world, renderer, _| {
            let frame = renderer.sc.get_current_frame().expect("Failed").output;
            let mut encoder =
                renderer
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("TestRenderEncoder"),
                    });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });
            }

            renderer.queue.submit(Some(encoder.finish()));
        })
}

fn test_input() -> Box<dyn Schedulable> {
    SystemBuilder::new("TestPrintSystem")
        .read_resource::<Input>()
        .build(|_, _, input, _| {
            let input: &Input = input;
            if input.keyboard.just_pressed(VirtualKeyCode::Space) {
                println!("Pressed Space")
            }
            if input.mouse.just_pressed(MouseButton::Left) {
                println!(
                    "click at {} | {}!",
                    input.mouse.position.0, input.mouse.position.1
                )
            }
        })
}

fn test_print() -> Box<dyn Schedulable> {
    SystemBuilder::new("TestPrintSystem").build(|_, _, _, _| println!("Test"))
}

fn test_timer() -> Box<dyn Schedulable> {
    SystemBuilder::new("PrintTimeSystem")
        .read_resource::<Timer>()
        .build(move |_, _, timer, _| println!("{:?}", timer.delta_time()))
}
