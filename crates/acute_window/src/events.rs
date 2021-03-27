use crate::{WindowDescriptor, WindowId};

#[derive(Debug)]
pub struct WindowCreateEvent {
    pub id: WindowId,
    pub descriptor: WindowDescriptor,
}
#[derive(Debug)]
pub struct WindowCreatedEvent {
    pub id: WindowId,
}
#[derive(Debug)]
pub struct WindowCloseRequestedEvent {
    pub id: WindowId,
}
#[derive(Debug)]
pub struct WindowResizedEvent {
    pub id: WindowId,
    pub width: f32,
    pub height: f32,
}
