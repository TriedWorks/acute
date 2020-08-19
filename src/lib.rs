pub mod prelude;
mod defaults;

pub use acute_app as app;
pub use acute_assets as assets;
pub use acute_core as core;
pub use acute_ecs as ecs;
pub use acute_input as input;
pub use acute_render as render;
// Don't expose backend, nor wgpu after finishing acute_render
pub use acute_render_backend as render_backend;
pub use acute_render_backend::wgpu;

pub use acute_scenes as scenes;
pub use acute_window as window;

pub use defaults::DefaultAddons;