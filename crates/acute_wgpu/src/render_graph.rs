use crate::render_context::WgpuRenderContext;
use crate::resource_context::WgpuResourceContext;
use acute_window::Windows;
use std::ops::Deref;

pub struct WgpuRenderGraph {}

impl WgpuRenderGraph {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(
        &self,
        queue: &mut wgpu::Queue,
        resource_context: &WgpuResourceContext,
        windows: &Windows,
    ) {
        let resources = resource_context.resources.read();
        let resource_ctx = resource_context.clone();
        let device = resource_ctx.device.clone();
        let mut context = WgpuRenderContext::new(device, resource_ctx);

        let mut command_buffers = Vec::new();

        for (id, _) in windows.windows.iter() {
            let encoder = context.encoder.get_or_create(context.device.deref());
            let swap_chain = resources.swap_chains.get(id).unwrap();
            let frame = swap_chain.get_current_frame().unwrap().output;

            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });
            }

            if let Some(command_buffer) = context.finish() {
                command_buffers.push(command_buffer);
            }
            queue.submit(command_buffers.drain(..))
        }
    }
}
