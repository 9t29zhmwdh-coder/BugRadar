use std::collections::HashMap;
use std::sync::Arc;

use super::{LogParserPlugin, PluginFactory, PluginMetadata};
use crate::collector::parser::{json_parser::JsonPluginFactory, plaintext_parser::PlaintextPluginFactory};
use crate::plugin::builtin::{nginx::NginxPluginFactory, docker::DockerPluginFactory};

/// Specific formats first; plaintext is the catch-all.
const DETECTION_ORDER: [&str; 3] = ["nginx", "docker", "json"];
pub const FALLBACK_PARSER: &str = "plaintext";

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
        // The collector falls back to these two, but they were never
        // registered, so watching any file panicked on the fallback.
        registry.register(Arc::new(JsonPluginFactory));
        registry.register(Arc::new(PlaintextPluginFactory));
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

    /// The parser for a path, checked in a fixed order: iterating the map
    /// directly picked a random match whenever two parsers claimed a path.
    pub fn detect(&self, source_path: &str) -> &str {
        DETECTION_ORDER
            .iter()
            .copied()
            .find(|id| {
                self.factories
                    .get(*id)
                    .is_some_and(|f| f.create("", source_path).can_handle(source_path))
            })
            .unwrap_or(FALLBACK_PARSER)
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
