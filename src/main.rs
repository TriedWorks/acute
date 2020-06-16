extern crate acute;

use acute::acute::Acute;
use acute::window::WinitState;
use acute::scenes::Scene;

use winit::{event::Event, event::WindowEvent, dpi::LogicalSize};
use winit::event_loop::ControlFlow;
use winit::window::WindowBuilder;
use std::time::Duration;
use legion::world::World;

use legion::entity::Entity;
use ultraviolet::{
    Vec3,
    Rotor3,
};
use acute::components::simple::Transform;
use acute::components::geometry::Triangle2D;


const WINDOW_SIZE: LogicalSize<u32> = LogicalSize { width: 1280, height: 720 };
const TITLE: &str = "Acute Test";

fn main() {
    let (window_builder, event_loop) = WinitState::new(TITLE, WINDOW_SIZE);
    let test_scene = TestScene::new();

    let mut acute: Acute = Acute::new(window_builder, Some(Box::new(test_scene)), &event_loop);

    event_loop.run(move |event, _, mut control_flow| {
        acute.run(&event, &mut control_flow);
    });
}

struct TestScene {}

impl TestScene {
    fn new() -> Self {
        Self {}
    }
}

impl Scene for TestScene {
    fn update(&mut self, world: &mut World, delta_time: &Duration) {}

    fn fixed_update(&mut self, world: &mut World, delta_time: &Duration) {}

    fn on_start(&mut self, world: &mut World) {
        println!("init_world");
        world.insert(
            (),
            (0..1).map(|_| (
                Transform {
                    pos: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::default(),
                    rotation: Rotor3 { s: 0.0, bv: Default::default() },
                },
                Triangle2D::default(),
            )),
        );
    }
}


// OLD MAIN FILE (WILL STAY HERE UNTIL ECS AND RENDERING WORKS)


// extern crate ultraviolet as uv;
//
// use std::mem;
//
// use winit::{
//     event::*,
//     event_loop::{ControlFlow, EventLoop},
//     window::{Window, WindowBuilder},
// };
//
// use futures::executor::block_on;
// use glsl_to_spirv::ShaderType;
// use wgpu::{RenderPassDescriptor, ShaderModule};
// use winit_input_helper::WinitInputHelper;
//
// mod config;
// mod graphics;
// mod state;
// mod types;
// mod utils;
// mod tools;
// mod window;
//
// use state::traits::Stateful;
//
// use crate::state::state_handler::StateHandler;
//
// fn main() {
//     let event_loop = EventLoop::new();
//     let window = WindowBuilder::new()
//         .with_title(config::APP_NAME)
//         .build(&event_loop)
//         .unwrap();
//
//     window.set_cursor_grab(true).unwrap();
//
//     let mut state = block_on(State::new(&window));
//
//     event_loop.run(move |event, _, control_flow| {
//         state.input.update(&event);
//         match event {
//             Event::WindowEvent {
//                 ref event,
//                 window_id,
//             } if window_id == window.id() => {
//                 if !state.input(event, control_flow) {
//                     match event {
//                         WindowEvent::Resized(physical_size) => {
//                             state.resize(*physical_size);
//                         }
//                         WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
//                             state.resize(**new_inner_size);
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//             Event::RedrawRequested(_) => {
//                 state.update();
//                 state.render();
//             }
//             Event::MainEventsCleared => {
//                 window.request_redraw();
//             }
//             _ => {}
//         }
//     })
// }
//
// struct State {
//     surface: wgpu::Surface,
//     device: wgpu::Device,
//     queue: wgpu::Queue,
//     sc_desc: wgpu::SwapChainDescriptor,
//     swap_chain: wgpu::SwapChain,
//     input: WinitInputHelper,
//
//     state_handler: state::state_handler::StateHandler,
//
//     size: winit::dpi::PhysicalSize<u32>,
// }
//
// impl State {
//     async fn new(window: &Window) -> Self {
//         let mut input = WinitInputHelper::new();
//
//         let size = window.inner_size();
//
//         let surface = wgpu::Surface::create(window);
//
//         let adapter = wgpu::Adapter::request(
//             &wgpu::RequestAdapterOptions {
//                 power_preference: wgpu::PowerPreference::Default,
//                 compatible_surface: Some(&surface),
//             },
//             wgpu::BackendBit::PRIMARY,
//         )
//             .await
//             .unwrap();
//
//         let (device, queue) = adapter
//             .request_device(&wgpu::DeviceDescriptor {
//                 extensions: wgpu::Extensions {
//                     anisotropic_filtering: false,
//                 },
//                 limits: Default::default(),
//             })
//             .await;
//
//         let sc_desc = wgpu::SwapChainDescriptor {
//             usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
//             format: wgpu::TextureFormat::Bgra8UnormSrgb,
//             width: size.width,
//             height: size.height,
//             present_mode: wgpu::PresentMode::Fifo,
//         };
//
//         let swap_chain = device.create_swap_chain(&surface, &sc_desc);
//
//         let mut state_handler = StateHandler::new(&device, &sc_desc);
//
//         state_handler.add_state(Box::new(state::states::test_state::TestState::new(
//             &device, &queue, &sc_desc, &size,
//         )));
//
//         state_handler.set_state(state::states::state_ids::CHAOTIC);
//
//         Self {
//             surface,
//             device,
//             queue,
//             sc_desc,
//             swap_chain,
//             input,
//
//             state_handler,
//
//             size,
//         }
//     }
//
//     fn update(&mut self) {
//         self.state_handler.states[self.state_handler.current_state_in_vec].update(&mut self.device, &mut self.queue);
//     }
//
//     fn render(&mut self) {
//         let frame = self
//             .swap_chain
//             .get_next_texture()
//             .expect("Timeout getting texture");
//         let mut encoder = self
//             .device
//             .create_command_encoder(&wgpu::CommandEncoderDescriptor {
//                 label: Some("Render Encoder"),
//             });
//
//         {
//             self.state_handler.states[self.state_handler.current_state_in_vec].render(&frame, &mut encoder);
//         }
//
//         self.queue.submit(&[encoder.finish()]);
//     }
//
//     fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
//         self.size = new_size;
//         self.sc_desc.width = new_size.width;
//         self.sc_desc.height = new_size.height;
//         self.state_handler.states[self.state_handler.current_state_in_vec].resize(&mut self.device, &mut self.sc_desc, &self.size);
//         self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
//     }
//
//     // RESIZE STATE AFTER STATE CHANGE TO FULFILL ASSERTION
//     fn input(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) -> bool {
//         use crate::state::states::state_ids;
//
//         if self.input.key_released(VirtualKeyCode::Escape) || self.input.quit() {
//             *control_flow = ControlFlow::Exit;
//             return false;
//         }
//         if self.input.key_pressed(VirtualKeyCode::F1) {
//             self.state_handler.set_state(state_ids::NONE);
//             self.state_handler.states[self.state_handler.current_state_in_vec].resize(&mut self.device, &mut self.sc_desc, &self.size);
//         }
//         if self.input.key_pressed(VirtualKeyCode::F2) {
//             self.state_handler.set_state(state_ids::CHAOTIC);
//             self.state_handler.states[self.state_handler.current_state_in_vec].resize(&mut self.device, &mut self.sc_desc, &self.size);
//         }
//         self.state_handler.states[self.state_handler.current_state_in_vec].input(&self.input)
//     }
// }
