use std::time::Duration;

pub mod art;
pub mod color;
pub mod draw;
pub mod terminal;

pub const TEXT_OFFSET: (f64, f64) = (25.0, 50.0);
pub const FLASH_TIME: Duration = Duration::from_millis(500);
pub const TYPE_TIME: Duration = Duration::from_millis(30);