use thiserror::Error;
use std::io;
use std::net::AddrParseError;
use std::error::Error;

/// Общий тип ошибок для всех обработчиков протоколов
#[derive(Error, Debug)]
pub enum HandlerError {
    /// Ошибка подключения
    #[error("Connection failed to {addr}: {source}")]
    ConnectionFailed {
        addr: String,
        #[source]
        source: io::Error,
    },

    /// Операция не поддерживается протоколом
    #[error("Operation '{operation}' not supported for this protocol: {reason}")]
    UnsupportedOperation {
        operation: String,
        reason: String,
    },

    /// Таймаут операции
    #[error("Operation timed out after {duration}ms")]
    Timeout {
        duration: u64,
    },

    /// Ошибка разбора адреса (192.168.1.1:80)
    #[error("Failed to parse address '{addr}': {source}")]
    InvalidAddress {
        addr: String,
        #[source]
        source: AddrParseError,
    },

    /// Ошибка отправки данных
    #[error("Failed to send data: {source}")]
    SendError {
        #[source]
        source: io::Error,
    },

    /// Ошибка приёма данных
    #[error("Failed to receive data: {source}")]
    ReceiveError {
        #[source]
        source: io::Error,
    },

    /// Соединение разорвано
    #[error("Connection was closed by remote host")]
    ConnectionClosed,

    #[error("Receive is not supported")]
    ReceiveNotSupported,

    #[error("Invalid target")]
    InvalidTarget,

    #[error("Send failed: {0}")]
    SendFailed(#[from] io::Error),

    #[error("Connection reset")]
    ConnectionReset,

    /// Произвольная текстовая ошибка (заглушка)
    #[error("{0}")]
    Generic(String),
}

impl From<AddrParseError> for HandlerError {
    fn from(err: AddrParseError) -> Self {
        HandlerError::Generic(format!("Address parse error: {}", err))
    }
}

// Упрощённый конструктор для быстрого создания ошибок
impl HandlerError {
    /// Быстро создать ошибку "не поддерживается"
    pub fn unsupported(operation: &str, reason: &str) -> Self {
        HandlerError::UnsupportedOperation {
            operation: operation.to_string(),
            reason: reason.to_string(),
        }
    }

    /// Быстро создать ошибку таймаута
    pub fn timeout_ms(duration: u64) -> Self {
        HandlerError::Timeout { duration }
    }
}

