//! Модуль для работы с сетевыми протоколами

pub mod http;
pub mod tcp;

// Реэкспортируем публичное API
pub use http::HttpClient;
pub use crate::engine::traits::TcpProtocol;
