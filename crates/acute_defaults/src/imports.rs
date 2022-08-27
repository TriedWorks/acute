pub mod app {
    pub use acute_app::*;
}

pub mod window {
    pub use acute_window::*;
}

pub mod winit {
    pub use acute_winit::*;
}

// pub mod wgpu {
//     pub use acute_wgpu::*;
// }

pub mod vulkan {
    pub use acute_vulkan::*;
}

pub mod input {
    pub use acute_input::*;
}

pub mod assets {
    pub use acute_assets::*;
}

pub mod ecs {
    pub use bevy_ecs::*;
}

pub mod defaults {
    pub use crate::*;
}

pub mod prelude {
    pub use crate::DefaultPlugins;
    pub use acute_app::prelude::*;
    pub use acute_input::prelude::*;
    pub use acute_vulkan::prelude::*;
    pub use bevy_ecs::prelude::*;

    pub fn default<T: Default>() -> T {
        std::default::Default::default()
    }
}
