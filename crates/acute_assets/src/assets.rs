use std::collections::HashMap;
use uuid::Uuid;
use crate::types::Asset;
use crate::{AssetKind, Shader};
use crate::types::Image;
use std::path::PathBuf;

pub struct Assets {
    asset_folder: PathBuf,
    assets: HashMap<Uuid, Box<dyn Asset>>,
}

impl Assets {
    pub fn new(asset_path: PathBuf) -> Self {
        Self {
            asset_folder: asset_path,
            assets: Default::default(),
        }
    }

    pub fn load(&mut self, path: &str, kind: AssetKind) -> Uuid {
        let mut location = self.asset_folder.clone();
        location.push(path);
        let location = location.to_str().unwrap();
        let uuid = Uuid::new_v4();
        match kind {
            AssetKind::Image => {
                self.assets.insert(uuid, Box::new(Image::load(location)));
            }
            AssetKind::Shader(kind) => {
                let shader = Box::new(Shader::load(location, kind));
                self.assets.insert(uuid, shader);
            }
            _ => {}
        }
        uuid
    }
    pub fn add<T: Asset>(&mut self, asset: T) -> (Uuid, &T) {
        let uuid = Uuid::new_v4();

        self.assets.insert(uuid, Box::new(asset));
        (uuid, self.get(&uuid).unwrap())
    }

    pub fn get<T: Asset>(&self, id: &Uuid) -> Option<&T> {
        self.assets.get(id).unwrap().downcast_ref::<T>()
    }

    pub fn get_mut<T: Asset>(&mut self, id: &Uuid) -> Option<&mut T> {
        self.assets.get_mut(id).unwrap().downcast_mut::<T>()
    }
}
unsafe impl Send for Assets { }
unsafe impl Sync for Assets { }

impl Default for Assets {
    fn default() -> Self {
        let mut asset_folder = std::env::current_dir().unwrap();
        asset_folder.push("assets");
        Self {
            asset_folder,
            assets: Default::default(),
        }
    }
}