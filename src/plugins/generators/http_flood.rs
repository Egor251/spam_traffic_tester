use crate::core::PacketGenerator;

pub struct HttpFloodGenerator {
    target: String,
    request_count: usize,
}

impl HttpFloodGenerator {
    pub fn new(target: String, count: usize) -> Self {
        Self { target, request_count: count }
    }
}

impl PacketGenerator for HttpFloodGenerator {
    fn generate(&self) -> Vec<u8> {
        // Генерация пакетов без знания о протоколах
        (0..self.request_count)
            .map(|i| format!("GET /?{} HTTP/1.1\r\nHost: {}\r\n\r\n", i, self.target))
            .collect::<String>()
            .into_bytes()
    }

    fn target(&self) -> &str {
        &self.target
    }
}