//! This example demonstrates how to use `Text` to draw TrueType font texts efficiently.

use ggez::graphics;
use ggez::GameResult;

use simpleterm::terminal::{new_window, Terminal};

pub fn main() -> GameResult {
    let (ctx, events_loop) = &mut new_window("simpleterm test")?;
    let term = &mut Terminal::new(ctx, "/LeagueSpartan-Regular.ttf", 24.0, graphics::BLACK, graphics::WHITE)?;
    term.tell(ctx, events_loop, "MESSAGE TEXT \nMESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT")
}
