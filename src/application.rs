use legion::prelude::*;
use winit::{
    window::WindowBuilder,
    event_loop::{EventLoop, ControlFlow},
    event::Event,
};

use crate::graphics::renderer::Renderer;
use crate::scenes::{SceneHandler, Scene};

pub struct Acute {
    pub universe: Universe,
    pub worlds: Vec<World>,
    pub scene_handler: SceneHandler,
    pub renderer: Renderer,
}

impl Acute {
    pub fn new(
        window_builder: WindowBuilder,
        event_loop: &EventLoop<()>,
        init_scene: Option<Box<dyn Scene>>,
    ) -> Self {
        let window = window_builder.build(event_loop).unwrap();
        let size = window.inner_size();
        let universe = Universe::new();
        let worlds: Vec<World> = Vec::new();

        let renderer = futures::executor::block_on(
            Renderer::new(window, size)
        );

        let scene_handler: SceneHandler = SceneHandler::new(init_scene);

        Self {
            universe,
            worlds,
            scene_handler,
            renderer,
        }
    }

    pub fn run(
        &mut self,
        event: &Event<()>,
        control_flow: &mut ControlFlow,
    ) {

    }
}