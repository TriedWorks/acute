use acute::prelude::*;
use acute_assets::{AssetKind, Assets};

fn main() {
    App::builder()
        .with_defaults()
        // .add_system(test_print_system())
        .build()
        .run();
}

#[system]
fn test_render(#[resource] renderer: &mut WgpuRenderer) {
    let frame = renderer
        .resources
        .swap_chain
        .get_current_frame()
        .expect("Failed")
        .output;
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

        render_pass.set_pipeline(&renderer.resources.pipelines.get("simple_color").unwrap());
        render_pass.set_vertex_buffer(0, renderer.resources.buffers.get(0).unwrap().slice(..));
        render_pass.set_index_buffer(renderer.resources.buffers.get(1).unwrap().slice(..));
        render_pass.draw_indexed(0..6, 0, 0..1);
    }

    renderer.queue.submit(Some(encoder.finish()));
}

// #[system]
// fn test_assets(#[resource] assets: &mut Assets) {
//     let img = assets.add("cat.png", AssetKind::Image);
// }

// #[system]
// fn test_input(#[resource] input: &Input) {
//     if input.keyboard.just_pressed(VirtualKeyCode::Space) {
//         println!("Pressed Space")
//     }
//     if input.mouse.just_pressed(MouseButton::Left) {
//         println!(
//             "click at {} | {}!",
//             input.mouse.position.0, input.mouse.position.1
//         )
//     }
// }

#[system]
fn test_print() {
    println!("Test");
}

#[system]
fn test_timer(#[resource] timer: &Option<Timer>) {
    if let Some(timer) = timer {
        println!("{:?}", timer.delta_time());
    } else {
        println!("no timer")
    }

}
