use crate::engine::MicroKernel;
use crate::engine::traits::ProtocolHandler;
use crate::registry::Plugin;
use crate::registry::PluginType;
use async_trait::async_trait;
use serde_yaml::Value;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use crate::errors::HandlerError;
use tokio::net::TcpStream;
pub struct TcpProtocolPlugin {
    config: HashMap<String, Value>,
}

struct TcpHandlerError {
    connection_failed: bool,
    timeout: bool,
    connection_reset: bool,
    invalid_target: bool,
    send_failed: bool,
}

// Сам обработчик протокола
pub struct TcpHandler {
    // Конфигурация TCP
    nodelay: bool,
    keepalive: bool,
    timeout: Duration,
    buffer_size: usize,
    // Внутреннее состояние
    // connections: ConnectionPool, если пул соединений
    // client: Option<TcpStream>, для single connection
}
#[async_trait]
impl ProtocolHandler for TcpHandler {

    fn protocol_name(&self) -> &str {
        "tcp"
    }

    async fn send(&self, target: &str, data: &[u8]) -> Result<(), HandlerError> {
        let mut stream = TcpStream::connect(target).await?;  // io::Error → HandlerError::IoError
        stream.set_nodelay(self.nodelay)?;             // io::Error → HandlerError::IoError
        stream.write_all(data).await?;                      // io::Error → HandlerError::IoError
        Ok(())
    }

async fn receive(&self) -> Result<Vec<u8>, HandlerError> {
    // Чтение из сокета
    // Обработка partial reads
    // Таймауты
}


}
