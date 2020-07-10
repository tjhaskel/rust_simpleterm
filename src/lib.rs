#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! Simpleterm is a bespoke fake terminal created with piston_window.

use std::time::Duration;

/// Ascii art strings.
pub mod art;

/// Draws rectangles and text on the terminal window.
pub mod draw;

/// Creates and interacts with a terminal window.
pub mod terminal;

/// Contains functions related to text color and bounds.
pub mod text;

/// Indicates the x and y offset of the text and surrounding box from the corners of the terminal window.
pub const TEXT_OFFSET: (f64, f64) = (25.0, 50.0);

/// How long should elements like "Press Enter to Continue" or the input cursor take before toggling their flash state.
pub const FLASH_TIME: Duration = Duration::from_millis(500);

/// How long should the terminal take to type a single character when displaying a message.
pub const TYPE_TIME: Duration = Duration::from_millis(30);