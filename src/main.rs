//! This example demonstrates how to use `Text` to draw TrueType font texts efficiently.

use ggez::GameResult;

use simpleterm::{terminal::{new_window, Terminal}, DARK_GREY, LIGHT_BLUE};

pub fn main() -> GameResult {
    let (ctx, events_loop) = &mut new_window("simpleterm test")?;
    let term = &mut Terminal::new(ctx, "/LeagueSpartan-Regular.ttf", 24.0, DARK_GREY, LIGHT_BLUE)?;

    term.ask("this is the ask command");
    term.show("this is the show command for=> 2.5");
    term.tell("this is the tell command");
    term.start(ctx, events_loop)
}
