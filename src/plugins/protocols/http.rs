use reqwest;
use std::error::Error;
use async_trait::async_trait;
use crate::engine::traits::ProtocolHandler;


#[derive(thiserror::Error, Debug)]
pub enum HttpError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Invalid target URL: {0}")]
    InvalidUrl(String),
}


/// HTTP-клиент для отправки и приема данных по HTTP
pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    /// Создает новый экземпляр HTTP-клиента
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl ProtocolHandler for HttpClient {
    fn protocol_name(&self) -> &str {
        "http"
    }
    async fn send(&self, target: &str, data: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>> {
        let response = self.client.post(target)
            .body(data.to_vec())
            .send()
            .await
            .map_err(|e| HttpError::RequestFailed(e))?;

        Ok(())
    }
}