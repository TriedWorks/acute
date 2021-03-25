use crate::render_context::RenderContext;
use crate::resources::ResourceRefs;
use std::ops::Range;

pub struct RenderPass<'a> {
    pub render_pass: wgpu::RenderPass<'a>,
    pub render_context: &'a RenderContext,
    pub resources: ResourceRefs<'a>,
}

impl<'a> RenderPass<'a> {
    pub fn set_viewport(&mut self, x: f32, y: f32, w: f32, h: f32, min_depth: f32, max_depth: f32) {
        self.render_pass
            .set_viewport(x, y, w, h, min_depth, max_depth);


    }

    pub fn set_scissor_rect(&mut self, x: u32, y: u32, w: u32, h: u32) {
        self.render_pass.set_scissor_rect(x, y, w, h);
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.render_pass.draw(vertices, instances);
    }
}