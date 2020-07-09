#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use std::time::Duration;

pub mod art;
pub mod draw;
pub mod terminal;
pub mod text;

pub const TEXT_OFFSET: (f64, f64) = (25.0, 50.0);
pub const FLASH_TIME: Duration = Duration::from_millis(500);
pub const TYPE_TIME: Duration = Duration::from_millis(30);