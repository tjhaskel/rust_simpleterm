use cgmath::Point2;
use std::time::Duration;

pub mod terminal;

pub const TEXT_OFFSET: Point2<f32> = Point2::new(25.0, 20.0);
pub const FLASH_TIME: Duration = Duration::from_millis(500);
pub const TYPE_TIME: Duration = Duration::from_millis(30);
