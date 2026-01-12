use std::collections::HashMap;
use std::sync::Arc;
use crate::engine::traits::{PacketGenerator, ProtocolHandler};

pub struct MicroKernel {
    generators: Vec<Arc<dyn PacketGenerator>>,
    handlers: HashMap<String, Arc<dyn ProtocolHandler>>,
}

impl MicroKernel {
    pub fn new() -> Self {
        Self {
            generators: Vec::new(),
            handlers: HashMap::new(),
        }
    }

    pub fn register_generator(&mut self, generator: Arc<dyn PacketGenerator>) {
        self.generators.push(generator);
    }

    pub fn register_handler(&mut self, handler: Arc<dyn ProtocolHandler>) {
        self.handlers.insert(handler.protocol_name().to_string(), handler);
    }

    pub async fn run(&self) {
        println!("🚀 MicroKernel запущен!");

        for generator in &self.generators {
            // TODO: Генерация и отправка пакетов
            println!("Генератор: {:?}", generator.target());
        }
    }
}