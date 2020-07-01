use cgmath::Point2;
use ggez::graphics::Color;
use std::time::Duration;

pub mod draw;
pub mod terminal;

pub const TEXT_OFFSET: Point2<f32> = Point2::new(30.0, 25.0);
pub const FLASH_TIME: Duration = Duration::from_millis(500);
pub const TYPE_TIME: Duration = Duration::from_millis(30);

pub const DARK_GREY: Color = Color::new(0.16, 0.16, 0.16, 1.0);
pub const LIGHT_BLUE: Color = Color::new(0.4, 0.8, 1.0, 1.0);
