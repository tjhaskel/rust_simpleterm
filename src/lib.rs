use std::time::Duration;

pub mod color;
pub mod draw;
pub mod message;
pub mod terminal;

const FLASH_TIME: Duration = Duration::from_millis(500);
const TEXT_OFFSET: (f64, f64) = (25.0, 50.0);
const TYPE_TIME: Duration = Duration::from_millis(30);