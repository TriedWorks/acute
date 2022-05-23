use crate::convert::{convert_key, convert_mouse, convert_state};
use crate::window::WinitWindows;
use acute_app::{App, CoreStage, Plugin};
use acute_ecs::event::{Events, ManualEventReader};
use acute_ecs::system::ResMut;
use acute_ecs::world::World;
use acute_input::events::{KeyboardEvent, MouseButtonEvent, MouseMoveEvent, MouseScrollEvent};
use acute_window::{WindowCreateEvent, WindowCreatedEvent, Windows as AcuteWindows, Windows};
use winit::event::WindowEvent;
use winit::event::{Event, MouseScrollDelta};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};

mod convert;
pub mod window;

#[derive(Default)]
pub struct WinitPlugin;

impl Plugin for WinitPlugin {
    fn build(&self, app: &mut App) {
        app.init_non_send_resource::<WinitWindows>()
            .set_runner(winit_runner)
            .add_system_to_stage(CoreStage::PostUpdate, update_windows_system);

        let event_loop = EventLoop::new();
        let mut create_window_reader = WinitCreateWindowReader::default();

        handle_window_creation(&mut app.world, &event_loop, &mut create_window_reader.0);
        app.insert_resource(create_window_reader)
            .insert_non_send_resource(event_loop);
    }
}

pub fn update_windows_system(
    mut _acute_windows: ResMut<AcuteWindows>,
    mut _winit_windows: ResMut<WinitWindows>,
) {
}

#[derive(Default)]
struct WinitCreateWindowReader(ManualEventReader<WindowCreateEvent>);

pub fn winit_runner(mut app: App) {
    println!("hello!!");
    let event_loop = app
        .world
        .remove_non_send_resource::<EventLoop<()>>()
        .unwrap();
    println!("hello!!!");
    let mut window_create_reader = app
        .world
        .remove_resource::<WinitCreateWindowReader>()
        .unwrap()
        .0;

    app.world
        .insert_non_send_resource(event_loop.create_proxy());

    println!("hello!!!!");

    event_loop.run(move |event, event_loop, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
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
        },
        Event::MainEventsCleared => {
            handle_window_creation(&mut app.world, event_loop, &mut window_create_reader);
            app.update();
        }
        Event::RedrawEventsCleared => {}
        _ => {}
    });
}

pub fn handle_window_creation(
    world: &mut World,
    event_loop: &EventLoopWindowTarget<()>,
    event_reader: &mut ManualEventReader<WindowCreateEvent>,
) {
    let world = world.cell();
    let mut winit_windows = world.non_send_resource_mut::<WinitWindows>();
    let mut windows = world.resource_mut::<Windows>();

    let create_events = world.resource::<Events<WindowCreateEvent>>();
    let mut window_created_events = world.resource_mut::<Events<WindowCreatedEvent>>();

    for window_create_event in event_reader.iter(&create_events) {
        let window = winit_windows.create_window(
            event_loop,
            window_create_event.id,
            &window_create_event.descriptor,
        );
        windows.add(window);
        window_created_events.send(WindowCreatedEvent {
            id: window_create_event.id,
        });
    }
}
