pub mod assets;
pub mod types;

use acute_app::{App, Plugin};
pub use assets::Assets;
pub use types::*;

#[derive(Default)]
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_resource(Assets::default());
    }
}
