use crate::{WindowDescriptor, WindowId};

pub struct WindowCreateEvent {
    pub id: WindowId,
    pub descriptor: WindowDescriptor,
}

pub struct WindowCreatedEvent {
    pub id: WindowId,
}

pub struct WindowCloseRequestedEvent {
    pub id: WindowId,
}

pub struct WindowResizedEvent {
    pub id: WindowId,
    pub width: f32,
    pub height: f32,
}
