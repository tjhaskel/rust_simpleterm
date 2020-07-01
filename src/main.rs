//! This example demonstrates how to use `Text` to draw TrueType font texts efficiently.

use ggez::GameResult;

use simpleterm::{terminal::{new_window, Terminal}, DARK_GREY, LIGHT_BLUE};

pub fn main() -> GameResult {
    let (ctx, events_loop) = &mut new_window("simpleterm test")?;
    let term = &mut Terminal::new(ctx, "/LeagueSpartan-Regular.ttf", 24.0, DARK_GREY, LIGHT_BLUE)?;
    term.tell(ctx, events_loop, "MESSAGE TEXT \nMESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT")
}
