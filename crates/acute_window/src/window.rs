use std::collections::HashMap;
use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WindowId(Uuid);

impl WindowId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone)]
pub struct Window {
    pub(crate) id: WindowId,
    title: String,
    width: u32,
    height: u32,
    resizable: bool,
    primary: bool,
}

impl Window {
    pub fn new(id: WindowId, descriptor: &WindowDescriptor) -> Self {
        Self {
            id,
            title: descriptor.title.clone(),
            width: descriptor.width,
            height: descriptor.height,
            resizable: descriptor.resizable,
            primary: descriptor.primary,
        }
    }

    pub fn id(&self) -> WindowId {
        self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn resizable(&self) -> bool {
        self.resizable
    }
    pub fn primary(&self) -> bool {
        self.primary
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }
    pub fn set_resizable(&mut self, resizable: bool) {
        self.resizable = resizable;
    }
    pub fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }
}

#[derive(Debug, Clone)]
pub struct WindowDescriptor {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub primary: bool,
}

impl Default for WindowDescriptor {
    fn default() -> Self {
        Self {
            title: String::from("Acute"),
            width: 1280,
            height: 720,
            resizable: true,
            primary: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Windowed,
    Borderless,
    Fullscreen,
}
