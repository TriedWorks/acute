use crate::convert::{convert_key, convert_mouse, convert_state};
use acute_app::{App, AppEventExit, CoreStage, Plugin};
use acute_input::events::{KeyboardEvent, MouseButtonEvent, MouseMoveEvent, MouseScrollEvent};
use acute_window::{
    WindowEventCloseRequested, WindowEventClosed, WindowEventCreate, WindowEventCreated,
    Windows as AcuteWindows, Windows,
};
use bevy_ecs::event::{EventReader, EventWriter, Events, ManualEventReader};
use bevy_ecs::system::ResMut;
use bevy_ecs::world::World;
use winit::event::WindowEvent;
use winit::event::{Event, MouseScrollDelta};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};

mod convert;
mod window;

use acute_tracing::info;
pub use window::WinitWindows;
pub use winit::window::Window as WinitWindow;

pub mod internal {
    pub use winit;
}

#[derive(Default)]
pub struct WinitPlugin;

impl Plugin for WinitPlugin {
    fn build(&self, app: &mut App) {
        app.init_non_send_resource::<WinitWindows>()
            .set_runner(winit_runner)
            .add_system(close_system)
            .add_system_to_stage(CoreStage::PostUpdate, update_windows_system);

        let event_loop = EventLoop::new();
        let mut create_window_reader = WinitCreateWindowReader::default();

        handle_window_creation(&mut app.world, &event_loop, &mut create_window_reader.0);
        app.insert_resource(create_window_reader)
            .insert_non_send_resource(event_loop);

        info!("Loaded Plugin: Winit")
    }
}

pub fn close_system(
    mut windows: ResMut<Windows>,
    mut winit_windows: ResMut<WinitWindows>,
    mut events: EventReader<WindowEventCloseRequested>,
    mut closed_events: EventWriter<WindowEventClosed>,
) {
    for event in events.iter() {
        let _ = winit_windows.remove(event.id);
        let _ = windows.remove(event.id);
        closed_events.send(WindowEventClosed { id: event.id })
    }
}

pub fn update_windows_system(
    mut _acute_windows: ResMut<AcuteWindows>,
    mut _winit_windows: ResMut<WinitWindows>,
) {
}

#[derive(Default)]
struct WinitCreateWindowReader(ManualEventReader<WindowEventCreate>);

pub fn winit_runner(mut app: App) {
    let event_loop = app
        .world
        .remove_non_send_resource::<EventLoop<()>>()
        .unwrap();
    let mut window_create_reader = app
        .world
        .remove_resource::<WinitCreateWindowReader>()
        .unwrap()
        .0;

    let mut app_exit_event_reader = ManualEventReader::<AppEventExit>::default();

    app.world
        .insert_non_send_resource(event_loop.create_proxy());

    event_loop.run(move |event, event_loop, control_flow| match event {
        Event::WindowEvent {
            event,
            window_id: winit_window_id,
        } => {
            let winit_windows = app.world.non_send_resource_mut::<WinitWindows>();
            let window_id = if let Some(window_id) = winit_windows.get_window_id(winit_window_id) {
                window_id
            } else {
                return;
            };
            match event {
                WindowEvent::CloseRequested => {
                    let mut events = app.resource_mut::<Events<WindowEventCloseRequested>>();
                    events.send(WindowEventCloseRequested { id: window_id })
                }

                WindowEvent::KeyboardInput { ref input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        let mut events = app.resource_mut::<Events<KeyboardEvent>>();
                        events.send(KeyboardEvent {
                            key: Some(convert_key(key)),
                            state: convert_state(input.state),
                        });
                    }
                }
                WindowEvent::MouseInput { button, .. } => {
                    let mut events = app.resource_mut::<Events<MouseButtonEvent>>();
                    events.send(MouseButtonEvent {
                        button: Some(convert_mouse(button)),
                    });
                }

                WindowEvent::CursorMoved { position, .. } => {
                    let mut events = app.resource_mut::<Events<MouseMoveEvent>>();
                    events.send(MouseMoveEvent {
                        position: (position.x, position.y),
                    });
                }

                WindowEvent::MouseWheel { delta, .. } => match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        let mut events = app.resource_mut::<Events<MouseScrollEvent>>();
                        events.send(MouseScrollEvent { scroll: (x, y) });
                    }
                    MouseScrollDelta::PixelDelta(_) => {}
                },
                _ => {}
            }
        }
        Event::MainEventsCleared => {
            handle_window_creation(&mut app.world, event_loop, &mut window_create_reader);
            app.update();
        }
        Event::RedrawEventsCleared => {
            if let Some(app_exit_events) = app.get_resource::<Events<AppEventExit>>() {
                if app_exit_event_reader.iter(app_exit_events).last().is_some() {
                    *control_flow = ControlFlow::Exit;
                }
            }
        }
        _ => {}
    });
}

pub fn handle_window_creation(
    world: &mut World,
    event_loop: &EventLoopWindowTarget<()>,
    event_reader: &mut ManualEventReader<WindowEventCreate>,
) {
    let world = world.cell();
    let mut winit_windows = world.non_send_resource_mut::<WinitWindows>();
    let mut windows = world.resource_mut::<Windows>();

    let create_events = world.resource::<Events<WindowEventCreate>>();
    let mut window_created_events = world.resource_mut::<Events<WindowEventCreated>>();

    for window_create_event in event_reader.iter(&create_events) {
        let window = winit_windows.create_window(
            event_loop,
            window_create_event.id,
            &window_create_event.descriptor,
        );
        windows.add(window);
        window_created_events.send(WindowEventCreated {
            id: window_create_event.id,
        });
    }
}
