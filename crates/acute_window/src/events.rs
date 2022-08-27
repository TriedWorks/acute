use crate::{WindowDescriptor, WindowId};

#[derive(Debug)]
pub struct WindowEventCreate {
    pub id: WindowId,
    pub descriptor: WindowDescriptor,
}
#[derive(Debug)]
pub struct WindowEventCreated {
    pub id: WindowId,
}
#[derive(Debug)]
pub struct WindowEventCloseRequested {
    pub id: WindowId,
}

#[derive(Debug)]
pub struct WindowEventClosed {
    pub id: WindowId,
}

#[derive(Debug)]
pub struct WindowEventResized {
    pub id: WindowId,
    pub width: f32,
    pub height: f32,
}
