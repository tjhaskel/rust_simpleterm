use piston_window::types::Color;
use std::time::Duration;

pub mod color;
pub mod draw;
pub mod terminal;

use crate::color::*;
pub const DEFAULT_BGC: Color = DARK_GREY;
pub const DEFAULT_FGC: Color = LIGHT_PURPLE;

pub const TEXT_OFFSET: (f64, f64) = (25.0, 50.0);
pub const FLASH_TIME: Duration = Duration::from_millis(500);
pub const TYPE_TIME: Duration = Duration::from_millis(30);