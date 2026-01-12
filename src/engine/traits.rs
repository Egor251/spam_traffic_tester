use std::error::Error;
use async_trait::async_trait;
use std::clone::Clone;


// =========== УРОВЕНЬ ЯДРА ===========
// Эти трейты знает только MicroKernel
#[async_trait]
pub trait PacketGenerator: Send + Sync {
    /// Генерирует трафик
    async fn generate(&mut self) -> Result<(), Box<dyn Error + Send + Sync>>;

    /// Имя генератора
    fn generator_name(&self) -> &str;

    /// Тип генерируемого трафика
    fn traffic_type(&self) -> TrafficType;
}

/// Тип трафика
pub enum TrafficType {
    Http,
    Tcp,
    Udp,
    Icmp,
    // ... другие
}

#[async_trait]
pub trait ProtocolHandler: Send + Sync {
    async fn send(&mut self, target: &str, data: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn receive(&mut self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>>;
    fn protocol_name(&self) -> &str;

}

#[async_trait]
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    async fn register_with_kernel(&self, kernel: &mut crate::engine::MicroKernel) -> Result<(), Box<dyn Error>>;
}

// =========== УРОВЕНЬ TCP ===========
// Эти трейты только для TCP реализации
// Вынесены сюда, чтобы не плодить файлы
use crate::errors::HandlerError;

#[async_trait]
pub trait TcpProtocol: Send + Sync {
    async fn connect(&mut self, target: &str) -> Result<(), HandlerError>;
    async fn send_data(&mut self, data: &[u8]) -> Result<(), HandlerError>;
    async fn receive_data(&mut self) -> Result<Vec<u8>, HandlerError>;
    fn get_config(&self) -> &TcpConfig;

    /// Клонирование через Box (требуется для object safety)
    fn box_clone(&self) -> Box<dyn TcpProtocol>;
}

// Реализация Clone для Box<dyn TcpProtocol>
impl Clone for Box<dyn TcpProtocol> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

// Общая конфигурация для всех TCP обработчиков
#[derive(Debug, Clone)]
pub struct TcpConfig {
    pub nodelay: bool,
    pub timeout: std::time::Duration,
    pub buffer_size: usize,
}