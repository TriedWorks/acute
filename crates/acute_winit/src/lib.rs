use crate::convert::{convert_key, convert_mouse, convert_state};
use crate::window::WinitWindows;
use acute_app::{App, AppBuilder, EventReader, Events, Plugin};
use acute_ecs::system;
use acute_ecs::Resources;
use acute_input::events::{KeyboardEvent, MouseButtonEvent, MouseMoveEvent, MouseScrollEvent};
use acute_window::{WindowCreate, WindowCreated, Windows as AcuteWindows, Windows};
use winit::event::WindowEvent;
use winit::event::{Event, MouseScrollDelta};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
mod convert;
pub mod window;

pub struct WinitPlugin;

impl Default for WinitPlugin {
    fn default() -> Self {
        Self {}
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
    #[resource] _acute_windows: &AcuteWindows,
    #[resource] _winit_windows: &WinitWindows,
) {
}

pub fn winit_runner(mut app: App) {
    let event_loop = EventLoop::new();
    app.resources.insert(event_loop.create_proxy());

    let mut window_create_reader = EventReader::<WindowCreate>::default();

    event_loop.run(move |event, event_loop, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::Resized(_size) => {
                // TODO: implement this
            }
            WindowEvent::KeyboardInput { ref input, .. } => {
                if let Some(key) = input.virtual_keycode {
                    let mut events = app.resources.get_mut::<Events<KeyboardEvent>>().unwrap();
                    events.send(KeyboardEvent {
                        key: Some(convert_key(key)),
                        state: convert_state(input.state),
                    });
                }
            }
            WindowEvent::MouseInput { button, .. } => {
                let mut events = app.resources.get_mut::<Events<MouseButtonEvent>>().unwrap();
                events.send(MouseButtonEvent {
                    button: Some(convert_mouse(button)),
                });
            }

            WindowEvent::CursorMoved { position, .. } => {
                let mut events = app.resources.get_mut::<Events<MouseMoveEvent>>().unwrap();
                events.send(MouseMoveEvent {
                    position: (position.x, position.y),
                });
            }

            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(x, y) => {
                    let mut events = app.resources.get_mut::<Events<MouseScrollEvent>>().unwrap();
                    events.send(MouseScrollEvent { scroll: (x, y) });
                }
                MouseScrollDelta::PixelDelta(_) => {}
            },
            _ => {}
        },
        Event::MainEventsCleared => {
            handle_window_creation(&mut app.resources, event_loop, &mut window_create_reader);
            app.update();
        }
        Event::RedrawRequested(_) => {}
        Event::RedrawEventsCleared => {}
        _ => {}
    });
}

pub fn handle_window_creation(
    resources: &mut Resources,
    event_loop: &EventLoopWindowTarget<()>,
    event_reader: &mut EventReader<WindowCreate>,
) {
    let mut winit_windows = resources.get_mut::<WinitWindows>().unwrap();
    let mut windows = resources.get_mut::<Windows>().unwrap();

    let create_events = resources.get::<Events<WindowCreate>>().unwrap();
    let mut window_created_events = resources.get_mut::<Events<WindowCreated>>().unwrap();

    for window_create_event in event_reader.iter(&create_events) {
        let window = winit_windows.create_window(
            event_loop,
            window_create_event.id,
            &window_create_event.descriptor,
        );
        windows.add(window);
        window_created_events.send(WindowCreated {
            id: window_create_event.id,
        });
    }
}
