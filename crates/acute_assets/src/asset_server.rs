use std::collections::HashMap;
use uuid::Uuid;
use crate::types::Asset;
use crate::{AssetKind, Shader};
use crate::types::Image;
use std::path::PathBuf;

pub struct Assets {
    asset_folder: PathBuf,
    assets: HashMap<Uuid, Asset>,
}

impl Assets {
    pub fn new(asset_path: PathBuf) -> Self {
        Self {
            asset_folder: asset_path,
            assets: Default::default(),
        }
    }

    pub fn add(&mut self, path: &str, kind: AssetKind) -> Uuid {
        let mut location = self.asset_folder.clone();
        location.push(path);
        let uuid = Uuid::new_v4();
        match kind {
            AssetKind::Image => {
                self.assets.insert(uuid, Asset::Image(Image::load(location.to_str().unwrap())));
            }
            AssetKind::Texture => {}
            AssetKind::Object => {}
            AssetKind::Shader(kind) => {
                let shader = Shader::load(path, kind);
                self.assets.insert(uuid, Asset::Shader(shader));
            }
        }
        uuid
    }
}
unsafe impl Send for Assets { }
unsafe impl Sync for Assets { }

impl Default for Assets {
    fn default() -> Self {
        let mut asset_path = std::env::current_dir().unwrap();
        asset_path.push("assets");
        Self {
            asset_folder: asset_path,
            assets: Default::default(),
        }
    }
}