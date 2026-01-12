//! Реестр плагинов
//! Автоматически загружает плагины из директории plugins


use std::collections::HashMap;
use std::sync::Arc;
use serde_yaml::Value;
use async_trait::async_trait;
use crate::engine::MicroKernel;

// Тип фабричной функции
pub type PluginFactory = fn(&HashMap<String, Value>) -> Box<dyn Plugin>;

// Базовый трейт плагина
#[async_trait]
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;

    // Плагин сам решает что регистрировать
    async fn register_with_kernel(
        &self,
        kernel: &mut MicroKernel
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug, Clone, Copy)]
pub enum PluginType {
    Generator,
    Protocol,
}

pub struct PluginRegistry {
    factories: HashMap<String, PluginFactory>,
    configs: HashMap<String, HashMap<String, Value>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
            configs: HashMap::new(),
        }
    }

    // Регистрация фабрики
    pub fn register(&mut self, name: &str, factory: PluginFactory) {
        self.factories.insert(name.to_string(), factory);
    }

    // Загрузка конфигурации
    pub fn load_config(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: HashMap<String, HashMap<String, Value>> = serde_yaml::from_str(&content)?;

        for (name, plugin_config) in config {
            self.configs.insert(name, plugin_config);
        }

        Ok(())
    }

    // Создание плагина по имени
    pub fn create(&self, name: &str) -> Option<Box<dyn Plugin>> {
        let factory = self.factories.get(name)?;
        let config = self.configs.get(name).cloned().unwrap_or_default();

        Some(factory(&config))
    }

    // Создание всех включённых плагинов
    pub fn create_enabled(&self) -> Vec<Box<dyn Plugin>> {
        let mut plugins = vec![];

        for (name, config) in &self.configs {
            if let Some(enabled) = config.get("enabled") {
                if enabled.as_bool().unwrap_or(false) {
                    if let Some(plugin) = self.create(name) {
                        plugins.push(plugin);
                    }
                }
            }
        }

        plugins
    }
}