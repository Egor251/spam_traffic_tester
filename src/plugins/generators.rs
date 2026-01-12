//! Модуль для работы с генераторами трафика

/// Базовый трейт для всех генераторов трафика
pub trait TrafficGenerator {
    /// Генерирует пакет данных
    fn generate_packet(&self) -> Vec<u8>;
    
    /// Возвращает задержку до следующего пакета в миллисекундах
    fn get_delay_ms(&self) -> u64;
}

// Пример реализации простого генератора
pub struct SimpleGenerator {
    packet: Vec<u8>,
    delay_ms: u64,
}

impl SimpleGenerator {
    pub fn new(packet: Vec<u8>, delay_ms: u64) -> Self {
        Self { packet, delay_ms }
    }
}

impl TrafficGenerator for SimpleGenerator {
    fn generate_packet(&self) -> Vec<u8> {
        self.packet.clone()
    }
    
    fn get_delay_ms(&self) -> u64 {
        self.delay_ms
    }
}