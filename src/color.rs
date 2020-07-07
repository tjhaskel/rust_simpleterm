use piston_window::types::Color;

pub const DARK_GREY: Color = [0.16, 0.16, 0.16, 1.0];
pub const DARK_PURPLE: Color = [0.4, 0.2, 0.7, 1.0];
pub const LIGHT_BLUE: Color = [0.4, 0.8, 1.0, 1.0];
pub const LIGHT_PURPLE: Color = [0.6, 0.4, 1.0, 1.0];
pub const OFF_WHITE: Color = [0.98, 0.96, 0.94, 1.0];

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
