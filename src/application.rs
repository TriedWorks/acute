use crate::graphics::renderer::Renderer;
use legion::prelude::*;
use winit::window::WindowBuilder;
use winit::event_loop::EventLoop;

// TODO (future): create separate worlds within a universe instead of just using one resource ?
pub struct Application {
    pub renderer: Renderer,
    pub resources: Resources,
}

impl Application {
    pub fn new(
        window_builder: WindowBuilder,
        event_loop: &EventLoop<()>,
    ) -> Self {
        let window = window_builder.build(event_loop).unwrap();
        let size = window.inner_size();
        let mut resources = Resources::default();

        let renderer = futures::executor::block_on(
            Renderer::new(window, size, &mut resources)
        );

        unimplemented!()
    }
}