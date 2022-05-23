use crate::App;
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub trait Plugin: Any + Send + Sync {
    fn build(&self, app: &mut App);
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
    plugins: HashMap<TypeId, PluginBundleEntry>,
    order: Vec<TypeId>,
}

impl PluginBundleBuilder {
    pub fn add<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        self.plugins.insert(
            TypeId::of::<T>(),
            PluginBundleEntry {
                plugin: Box::new(plugin),
            },
        );
        self.order.push(TypeId::of::<T>());
        self
    }

    pub fn add_init<T: Plugin + Default>(&mut self) -> &mut Self {
        self.plugins.insert(
            TypeId::of::<T>(),
            PluginBundleEntry {
                plugin: Box::new(T::default()),
            },
        );
        self.order.push(TypeId::of::<T>());
        self
    }

    pub fn finish(self, app: &mut App) {
        for plugin_id in &self.order {
            if let Some(entry) = self.plugins.get(plugin_id) {
                entry.plugin.build(app);
            }
        }
    }
}
