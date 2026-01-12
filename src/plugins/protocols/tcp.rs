use crate::errors::HandlerError;
use crate::engine::traits::{TcpProtocol, TcpConfig};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use async_trait::async_trait;
use std::collections::HashMap;
use serde_yaml::Value;


/// Обработчик TCP через TcpStream (обычный режим)
pub struct TcpStreamHandler {
    config: TcpConfig,
    stream: Option<TcpStream>,
}

impl From<&HashMap<String, Value>> for TcpConfig {
    fn from(config: &HashMap<String, Value>) -> Self {
        TcpConfig {
            nodelay: config.get("nodelay")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            timeout: std::time::Duration::from_secs(
                config.get("timeout")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(30)
            ),
            buffer_size: config.get("buffer_size")
                .and_then(|v| v.as_u64())
                .unwrap_or(1024) as usize,
        }
    }
}

/// Плагин, который оборачивает TCP обработчик
pub struct TcpProtocolPlugin {
    handler: Box<dyn TcpProtocol>,
}

impl TcpProtocolPlugin {
    pub fn new(config: TcpConfig) -> Self {
        let handler = create_tcp_handler(TcpHandlerType::Stream, config);
        Self { handler }
    }
}

#[async_trait]
impl crate::registry::Plugin for TcpProtocolPlugin {
    fn name(&self) -> &str {
        "tcp_protocol"
    }

    async fn register_with_kernel(
        &self,
        kernel: &mut crate::engine::MicroKernel
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Создаём адаптер TcpProtocol → ProtocolHandler
        struct TcpAdapter {
            inner: Box<dyn TcpProtocol>,
        }

        #[async_trait]
        impl crate::engine::traits::ProtocolHandler for TcpAdapter {
            async fn send(&mut self, target: &str, data: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                self.inner.connect(target).await?;
                self.inner.send_data(data).await?;
                Ok(())
            }

            async fn receive(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
                self.inner.receive_data().await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }

            fn protocol_name(&self) -> &str {
                "tcp"
            }
        }

        let adapter = TcpAdapter {
            inner: self.handler.clone(), // или нужно клонировать?
        };

        kernel.register_handler(std::sync::Arc::new(adapter));
        Ok(())
    }
}

impl TcpStreamHandler {
    pub fn new(config: TcpConfig) -> Self {
        Self { config, stream: None }
    }
}

#[async_trait]
impl TcpProtocol for TcpStreamHandler {
    async fn connect(&mut self, target: &str) -> Result<(), HandlerError> {
        let stream = TcpStream::connect(target).await?;
        stream.set_nodelay(self.config.nodelay)?;
        self.stream = Some(stream);
        Ok(())
    }

    async fn send_data(&mut self, data: &[u8]) -> Result<(), HandlerError> {
        match &mut self.stream {
            Some(stream) => {
                stream.write_all(data).await?;
                Ok(())
            }
            None => Err(HandlerError::Generic("Not connected".to_string())),
        }
    }

    async fn receive_data(&mut self) -> Result<Vec<u8>, HandlerError> {
        match &mut self.stream {
            Some(stream) => {
                let mut buffer = vec![0u8; self.config.buffer_size];
                let bytes_read = stream.read(&mut buffer).await?;

                if bytes_read == 0 {
                    return Err(HandlerError::ConnectionClosed);
                }

                buffer.truncate(bytes_read);
                Ok(buffer)
            }
            None => Err(HandlerError::Generic("Not connected".to_string())),
        }
    }

    fn get_config(&self) -> &TcpConfig {
        &self.config
    }
    fn box_clone(&self) -> Box<dyn TcpProtocol> {
        // Создаём новый handler с тем же config
        Box::new(TcpStreamHandler {
            config: self.config.clone(),
            stream: None,  // НОВОЕ соединение, не клонируем старый stream!
        })
    }
}

/// Фабрика для создания TCP обработчиков
pub enum TcpHandlerType {
    Stream,  // Обычный TcpStream
    Raw,     // Raw sockets (позже)
}

pub fn create_tcp_handler(
    handler_type: TcpHandlerType,
    config: TcpConfig
) -> Box<dyn TcpProtocol> {
    match handler_type {
        TcpHandlerType::Stream => Box::new(TcpStreamHandler::new(config)),
        TcpHandlerType::Raw => {
            // TODO: Реализовать позже
            unimplemented!("Raw TCP handler not yet implemented")
        }
    }
}



