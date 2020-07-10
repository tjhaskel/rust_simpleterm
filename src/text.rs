use piston_window::{*, types::{Color, FontSize}};
use std::{path::Path, {time::Duration, time::Instant}};

use crate::FLASH_TIME;

/// <span style="color:#DB143D; text-shadow: 1px 0.5px #555">█</span>
pub const CRIMSON: Color =      [0.86, 0.08, 0.24, 1.0];

/// <span style="color:#292929; text-shadow: 1px 0.5px #555">█</span>
pub const DARK_GREY: Color =    [0.16, 0.16, 0.16, 1.0];

/// <span style="color:#6633B3; text-shadow: 1px 0.5px #555">█</span>
pub const DARK_PURPLE: Color =  [0.4,  0.2,  0.7,  1.0];

/// <span style="color:#00C957; text-shadow: 1px 0.5px #555">█</span>
pub const EMERALD: Color =      [0.0,  0.79, 0.34, 1.0];

/// <span style="color:#FFA61A; text-shadow: 1px 0.5px #555">█</span>
pub const GOLD: Color =         [1.0,  0.65, 0.10, 1.0];

/// <span style="color:#66CCFF; text-shadow: 1px 0.5px #555">█</span>
pub const LIGHT_BLUE: Color =   [0.4,  0.8,  1.0,  1.0];

/// <span style="color:#9966FF; text-shadow: 1px 0.5px #555">█</span>
pub const LIGHT_PURPLE: Color = [0.6,  0.4,  1.0,  1.0];

/// <span style="color:#FAF5F0; text-shadow: 1px 0.5px #555">█</span>
pub const OFF_WHITE: Color =    [0.98, 0.96, 0.94, 1.0];

/// Adds brightness functions to PistonWindow's Color type
pub trait TermColor {
    /// Uses a [weighted](https://www.nbdtech.com/Blog/archive/2008/04/27/Calculating-the-Perceived-Brightness-of-a-Color.aspx) color axis to determine percieved brightness of a color.
    /// ```
    /// # use simpleterm::text::*;
    /// assert_eq!(EMERALD.brightness(), 0.6626567);
    /// ```
    fn brightness(&self) -> f32;
    
    /// Returns true if this color is brighter than the given other color.
    /// ```
    /// # use simpleterm::text::*;
    /// assert!(LIGHT_PURPLE.brighter_than(DARK_PURPLE));
    /// ```
    fn brighter_than(&self, other: Color) -> bool;
}

impl TermColor for Color {
    fn brighter_than(&self, other: Color) -> bool {
        self.brightness() > other.brightness()
    }
    
    fn brightness(&self) -> f32 {
        let weighted_add: f32 =
            (self[0] * self[0] * 0.241) +
            (self[1] * self[1] * 0.691) +
            (self[2] * self[2] * 0.068);
    
        weighted_add.sqrt() * self[3]
    }
}

/// Returns the Glyph cache generated from the given font file opened in the given PistonWindow.
pub fn load_font(window: &mut PistonWindow, name: &str) -> Glyphs {
    let resources: &Path = Path::new("resources");
    window.load_font(resources.join(name)).unwrap()
}

/// Returns a vector of strings corresponding to a word split up at the given number of characters.
/// first_split may be smaller than rest_split to allow the first part of a word to fit on a line with previous words.
/// ```
/// # use simpleterm::text::*;
/// let long_word: &str = "supercalifragilisticexpialidocious";
/// assert_eq!(
///     split_word(long_word, 5, 10),
///     vec!(
///         String::from("super"),
///         String::from("califragil"),
///         String::from("isticexpia"),
///         String::from("lidocious"),
///     )
/// );
/// ```
pub fn split_word(x: &str, first_split: usize, rest_split: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let mut do_first: bool = true;
    let mut count: usize = 0;
    let mut current_string: String = String::default();
    for c in x.chars() {
        if do_first {
            if count >= first_split {
                result.push(current_string);
                current_string = format!("{}", c);
                do_first = false;
                count = 1;
            } else {
                current_string.push(c);
                count += 1;
            }
        } else if count >= rest_split {
            result.push(current_string);
            current_string = format!("{}", c);
            count = 1;
        } else {
            current_string.push(c);
            count += 1;
        }
    }
    result.push(current_string);

    result
}

/// Determines if enough time has passed since the last flash toggle. If so, save the current time and toggle the current flash state.
/// ```
/// # use std::{thread, time::{Duration, Instant}};
/// # use simpleterm::{text::*, FLASH_TIME};
/// let mut start: Instant = Instant::now();
/// thread::sleep(FLASH_TIME);
/// assert!(check_flash(Instant::now(), &mut start));
/// ```
pub fn check_flash(now: Instant, then: &mut Instant) -> bool {
    let time_since: Duration = now.duration_since(*then);
    if time_since > (FLASH_TIME * 2) {
        *then = now;
        true
    } else {
        time_since > FLASH_TIME
    }
}

/// Determines the top left corner of the given art in the given window, in order for the art to be centered.
/// ```
/// # use simpleterm::{art::*, text::*};
/// let art: Vec<String> = GEO.split('\n').map(String::from).collect();
/// assert_eq!(place_art((800, 600).into(), &art, 10), (26.25, 40.0));
/// ```
pub fn place_art(win_size: Size, art: &[String], font_size: FontSize) -> (f64, f64) {
    let mid_x: f64 = win_size.width / 2.0;
    let mid_y: f64 = win_size.height / 2.0;

    let art_mid_x: f64 = (art[0].len() as f64 / 2.0) * (font_size as f64 * 0.65);
    let art_mid_y: f64 = (art.len() as f64 / 2.0) * (font_size as f64 * 0.8);
    
    (mid_x - art_mid_x, mid_y - art_mid_y)
}
