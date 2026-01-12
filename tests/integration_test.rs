// tests/integration_test.rs
#[cfg(test)]
mod tests {
    use trafficforge::plugins::protocols::tcp::TcpProtocolPlugin;
    use trafficforge::registry::{PluginRegistry, Plugin};
    use trafficforge::engine::traits::TcpConfig;
    use std::collections::HashMap;
    use serde_yaml::Value;
    use std::time::Duration;

    #[test]
    fn test_tcp_plugin_creation() {
        let mut registry = PluginRegistry::new();

        // Регистрируем фабрику - важно: она получает HashMap, но создаёт TcpConfig внутри
        registry.register("tcp_protocol", |config: &HashMap<String, Value>| {
            // Преобразуем HashMap → TcpConfig
            let tcp_config = TcpConfig {
                nodelay: config.get("nodelay")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true),
                timeout: Duration::from_secs(
                    config.get("timeout")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(30)
                ),
                buffer_size: config.get("buffer_size")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1024) as usize,
            };

            Box::new(TcpProtocolPlugin::new(tcp_config))
        });

        // Создаём конфиг в формате YAML (как будет в реальном config/plugins.yaml)
        let mut config = HashMap::new();
        config.insert("enabled".to_string(), Value::Bool(true));
        config.insert("nodelay".to_string(), Value::Bool(true));
        config.insert("timeout".to_string(), Value::Number(30.into()));
        config.insert("buffer_size".to_string(), Value::Number(1024.into()));

        // Добавляем конфиг в реестр
        registry.add_config("tcp_protocol".to_string(), config);

        // Создаём плагин
        let plugin = registry.create("tcp_protocol");
        assert!(plugin.is_some(), "TCP плагин должен создаваться");

        // Проверяем имя плагина
        let plugin = plugin.unwrap();
        assert_eq!(plugin.name(), "tcp_protocol");
    }
}