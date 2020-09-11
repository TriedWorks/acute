use acute::prelude::*;
use legion::system;

fn main() {
    let (window, event_loop) = WinitWindow::new(WindowDescriptor::default());
    // let event_loop = WinitWindow::new_headless();
    let mut app = App::builder()
        .with_defaults(window)
        // .with_defaults_headless()
        .add_system(test_input_system())
        // .add_system(test_timer())
        .add_render_system(test_render_system())
        .build();

    event_loop.run(move |event, _event_loop, mut control_flow| {
        app.run(&event, &mut control_flow);
    })
}

#[system]
fn test_render(#[resource] renderer: &mut WgpuRenderer) {
    let frame = renderer.resources.swap_chain.get_current_frame().expect("Failed").output;
    let mut encoder = renderer
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

        render_pass.set_pipeline(&renderer.resources.pipelines.get("none").unwrap());
        render_pass.draw(0..3, 0..1);
    }

    renderer.queue.submit(Some(encoder.finish()));
}

#[system]
fn test_input(#[resource] input: &Input) {
    if input.keyboard.just_pressed(VirtualKeyCode::Space) {
        println!("Pressed Space")
    }
    if input.mouse.just_pressed(MouseButton::Left) {
        println!(
            "click at {} | {}!",
            input.mouse.position.0, input.mouse.position.1
        )
    }
}

#[system]
fn test_print() {
    println!("Test");
}

#[system]
fn test_timer(#[resource] timer: &Timer) {
    println!("{:?}", timer.delta_time());
}
