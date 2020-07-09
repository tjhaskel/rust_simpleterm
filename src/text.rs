use piston_window::{*, types::Color};
use std::path::Path;

pub const CRIMSON: Color =      [0.86, 0.08, 0.24, 1.0];
pub const DARK_GREY: Color =    [0.16, 0.16, 0.16, 1.0];
pub const DARK_PURPLE: Color =  [0.4,  0.2,  0.7,  1.0];
pub const EMERALD: Color =      [0.0,  0.79, 0.34, 1.0];
pub const GOLD: Color =         [1.0,  0.65, 0.10, 1.0];
pub const LIGHT_BLUE: Color =   [0.4,  0.8,  1.0,  1.0];
pub const LIGHT_PURPLE: Color = [0.6,  0.4,  1.0,  1.0];
pub const OFF_WHITE: Color =    [0.98, 0.96, 0.94, 1.0];

pub trait TermColor {
    fn brightness(&self) -> f32;
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


pub fn load_font(window: &mut PistonWindow, name: &str) -> Glyphs {
    let resources: &Path = Path::new("resources");
    window.load_font(resources.join(name)).unwrap()
}

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
