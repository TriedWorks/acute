use crate::{WindowDescriptor, WindowId};

pub struct WindowCreate {
    pub id: WindowId,
    pub descriptor: WindowDescriptor,
}

pub struct WindowCreated {
    pub id: WindowId,
}

pub struct WindowCloseRequested {
    pub id: WindowId,
}
