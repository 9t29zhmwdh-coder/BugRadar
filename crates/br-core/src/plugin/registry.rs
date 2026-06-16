use std::collections::HashMap;
use std::sync::Arc;

use super::{LogParserPlugin, PluginFactory, PluginMetadata};
use crate::plugin::builtin::{nginx::NginxPluginFactory, docker::DockerPluginFactory};

pub struct PluginRegistry {
    factories: HashMap<String, Arc<dyn PluginFactory>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            factories: HashMap::new(),
        };
        registry.register(Arc::new(NginxPluginFactory));
        registry.register(Arc::new(DockerPluginFactory));
        registry
    }

    pub fn register(&mut self, factory: Arc<dyn PluginFactory>) {
        self.factories.insert(factory.id().to_string(), factory);
    }

    pub fn create(&self, plugin_id: &str, source_id: &str, source_path: &str) -> Option<Box<dyn LogParserPlugin>> {
        self.factories
            .get(plugin_id)
            .map(|f| f.create(source_id, source_path))
    }

    pub fn detect(&self, source_path: &str) -> Option<&str> {
        for (id, factory) in &self.factories {
            if factory.create("", source_path).can_handle(source_path) {
                return Some(id.as_str());
            }
        }
        None
    }

    pub fn list_metadata(&self) -> Vec<PluginMetadata> {
        self.factories
            .values()
            .map(|f| f.create("", "").metadata())
            .collect()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
