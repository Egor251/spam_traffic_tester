use std::error::Error;
use crate::errors::HandlerError;
use async_trait::async_trait;


/// Генератор пакетов для атак/тестов
#[async_trait]
pub trait PacketGenerator: Send + Sync {
    /// Генерирует пакеты (без ошибок - генерация не должна падать)
    async fn generate(&self) -> Vec<u8>;

    /// Цель для отправки
    fn target(&self) -> &str;
}

/// Обработчик протоколов для отправки
#[async_trait]
pub trait ProtocolHandler: Send + Sync {
    /// Имя протокола (http, tcp, udp)
    fn protocol_name(&self) -> &str;

    /// Отправка данных на целевой адрес
    async fn send(&self, target: &str, data: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>>;

    /// Получение данных от целевого адреса
    async fn receive(&self) -> Result<Vec<u8>, HandlerError> {
        Err(HandlerError::ReceiveNotSupported)
    }
}