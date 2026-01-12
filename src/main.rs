// Импортируем библиотечный крейт
use trafficforge::{
    plugins::protocols::tcp::TcpProtocolPlugin,
    registry::{PluginRegistry, Plugin},
    engine::{MicroKernel, traits::TcpConfig}
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Создаём реестр
    let mut registry = PluginRegistry::new();

    // 2. Регистрируем фабрику TCP плагина
    registry.register("tcp_protocol", |config| {
        let tcp_config = TcpConfig::from(config);  // Конвертируем YAML → TcpConfig
        Box::new(TcpProtocolPlugin::new(tcp_config))
    });

    // 3. Загружаем конфигурацию из YAML файла
    registry.load_config("config/plugins.yaml")?;

    // 4. Создаём все включённые плагины
    let plugins = registry.create_enabled();

    // 5. Создаём ядро
    let mut kernel = MicroKernel::new();

    // 6. Регистрируем все плагины в ядре
    for plugin in plugins {
        plugin.register_with_kernel(&mut kernel).await?;
    }

    // 7. Запускаем ядро
    kernel.run().await;

    Ok(())
}