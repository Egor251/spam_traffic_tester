// Импортируем библиотечный крейт
use trafficforge::{
    registry::{PluginRegistry, Plugin},
    engine::MicroKernel
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = PluginRegistry::new();
    let mut kernel = MicroKernel::new();

    // Регистрируем фабрики
    registry.register("http_flood", |config| {
        Box::new(plugins::http_flood::HttpFloodPlugin::new(config))
    });

    registry.register("http_protocol", |config| {
        Box::new(plugins::http_protocol::HttpProtocolPlugin::new(config))
    });

    // Загружаем конфиг
    registry.load_config("config/plugins.yaml")?;

    // Создаём и регистрируем плагины в ядре
    for plugin in registry.create_enabled() {
        plugin.register_with_kernel(&mut kernel).await?;
    }

    // Запускаем
    kernel.run().await;

    Ok(())
}