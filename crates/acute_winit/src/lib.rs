use acute_app::{Plugin, AppBuilder, App};
use acute_window::Windows as AcuteWindows;
use winit::event_loop::{EventLoop, EventLoopWindowTarget, ControlFlow};
use legion::*;
use crate::window::WinitWindows;
use winit::event::{Event, MouseScrollDelta};
use winit::event::WindowEvent;
use acute_input::Input;
use crate::convert::{convert_key, convert_state, convert_mouse};

mod window;
mod convert;

pub struct WinitPlugin;

impl Default for WinitPlugin {
    fn default() -> Self {
        Self { }
    }
}

impl Plugin for WinitPlugin {
    fn add(&self, app: &mut AppBuilder) {
        app.set_runner(winit_runner);
        app.add_resource(WinitWindows::new());
        app.add_system(update_windows_system());
    }
}

#[system]
pub fn update_windows(
    #[resource] acute_windows: &AcuteWindows,
    #[resource] winit_windows: &WinitWindows,
) {
}

pub fn winit_runner(mut app: App) {
    let event_loop = EventLoop::new();
    app.resources.insert(event_loop.create_proxy());

    event_loop.run(move |event, event_loop, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(size) => {
                }
                WindowEvent::KeyboardInput {ref input, ..} => {
                    if let Some(key) = input.virtual_keycode {
                        if let Some(mut acute_input) = app.resources.get_mut::<Input>() {
                            acute_input.update_keyboard(convert_key(key), convert_state(input.state));
                        }
                    }

                }
                WindowEvent::MouseInput {button, ..} => {
                    if let Some(mut acute_input) = app.resources.get_mut::<Input>() {
                        acute_input.update_mouse(convert_mouse(button));
                    }
                }

                WindowEvent::MouseWheel {delta, ..} => {
                    match delta {
                        MouseScrollDelta::LineDelta(x, y) => {
                            if let Some(mut acute_input) = app.resources.get_mut::<Input>() {
                                acute_input.update_mouse_scroll((x, y));
                            }
                        }
                        MouseScrollDelta::PixelDelta(_) => {}
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                handle_window_creation(&mut app.resources, event_loop);
                app.update();
            }
            Event::RedrawRequested(_) => {}
            Event::RedrawEventsCleared => {}
        _ => {}
        }
    });
}

pub fn handle_window_creation(
    resources: &mut Resources,
    event_loop: &EventLoopWindowTarget<()>,
) {
    let mut winit_windows = resources.get_mut::<WinitWindows>().unwrap();
    let acute_windows = resources.get::<AcuteWindows>().unwrap();
    for acute_window in acute_windows.iter() {
        if !winit_windows.window_id_to_winit.contains_key(&acute_window.id()) {
            winit_windows.create_window(acute_window, event_loop);
        }
    }
}