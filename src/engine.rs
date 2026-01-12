// Объявляем подмодули
pub mod traits;
pub mod kernel;

// Реэкспортируем публичное API для удобного импорта
pub use traits::{PacketGenerator, ProtocolHandler};
pub use kernel::MicroKernel;