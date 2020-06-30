use piston_window::types::Color;

pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
pub const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
pub const CYAN: Color = [0.0, 1.0, 1.0, 1.0];
pub const MAGENTA: Color = [1.0, 0.0, 1.0, 1.0];
pub const WHITE: Color = [1.0, 1.0, 1.0, 1.0];

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
