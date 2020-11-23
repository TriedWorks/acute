use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Window {
    id: WindowId,
    title: String,
    width: u32,
    height: u32,
    resizable: bool,
    primary: bool,
}

impl Window {
    pub fn id(&self) -> WindowId {
        self.id
    }

    pub fn title(&self) -> String {
        self.title.clone()
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
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WindowId(Uuid);

impl WindowId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for Window {
    fn default() -> Self {
        Self {
            id: WindowId::new(),
            title: String::from("Acute"),
            width: 1280,
            height: 720,
            resizable: true,
            primary: false
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Windowed,
    Borderless,
    Fullscreen
}

pub struct Windows {
    pub windows: HashMap<WindowId, Window>
}

impl Windows {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new()
        }
    }

    pub fn get(&self, id: WindowId) -> Option<&Window> {
        self.windows.get(&id)
    }

    pub fn add(&mut self, window: Window) {
        self.windows.insert(window.id.clone(), window);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Window> {
        self.windows.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Window> {
        self.windows.values_mut()
    }

}