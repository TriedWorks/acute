use crate::AppBuilder;
use std::collections::HashMap;
use std::any::{TypeId, Any};

pub trait Plugin: Any + Send + Sync {
    fn build(&self, app: &mut AppBuilder);
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub trait PluginBundle {
    fn build(&mut self, group: &mut PluginBundleBuilder);
}

pub struct PluginBundleEntry {
    plugin: Box<dyn Plugin>,
}

#[derive(Default)]
pub struct PluginBundleBuilder {
    plugins: HashMap<TypeId, PluginBundleEntry>
}

impl PluginBundleBuilder {
    pub fn add<T: Plugin + Default>(&mut self) -> &mut Self {
        self.plugins.insert(
            TypeId::of::<T>(),
            PluginBundleEntry {
                plugin: Box::new(T::default()),
            },
        );
        self
    }

    pub fn add_custom<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        self.plugins.insert(
            TypeId::of::<T>(),
            PluginBundleEntry {
                plugin: Box::new(plugin)
            }
        );
        self
    }

    pub fn finish(self, app: &mut AppBuilder) {
        for (_, entry) in self.plugins.iter() {
            entry.plugin.build(app);
        }
    }
}