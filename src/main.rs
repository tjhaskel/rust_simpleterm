use std::time::Duration;

use simpleterm::{art::*, text::*, terminal::Terminal};

fn main() {
    // Create a window and display the GEO art (from art.rs) for 2 seconds.
    let mut term: Terminal = Terminal::new("simpleterm test", (800, 600), DARK_GREY, GOLD, "LeagueSpartan-Regular.ttf", 32);
    term.display_art(GEO, Duration::from_secs(2));

    // Show a welcome message for 2 seconds, then display the next message until the user hits enter.
    term.show("Welcome to Simpleterm!", Duration::from_secs(2));
    term.tell("Simpleterm lets you display text. It know's how to break up long sentences and words: 0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789. Neat!
        \nThis is on a new line because it also supports \\n!");

    // Change the font and colors, then ask a question and repeat back the input with new colors if we get some.
    term.set_font("LeagueMono-Regular.ttf", 24);
    term.set_colors(OFF_WHITE, DARK_PURPLE);
    if let Some(input_recieved) = term.ask("Enter some input: ") {
        term.fg_color = EMERALD;
        term.tell(&format!("You said: {}", input_recieved));
    } else {
        println!("The window was interrupted before you could enter input!");
    }

    // Make the art font size smaller, resize the window to better frame it, change the colors, and display MONA from art.rs.
    term.art_font_size = 9;
    term.resize((600, 800).into());
    term.set_colors(DARK_GREY, EMERALD);
    term.display_art(MONA, Duration::from_millis(300));
    term.fg_color = GOLD;
    term.display_art(MONA, Duration::from_millis(300));
    term.fg_color = CRIMSON;
    term.display_art(MONA, Duration::from_millis(300));
    term.fg_color = LIGHT_PURPLE;
    term.display_art(MONA, Duration::from_millis(300));
    term.fg_color = LIGHT_BLUE;
    term.display_art(MONA, Duration::from_millis(300));

    // Change the font and colors and display some text. The window will close when the user hits enter.
    term.set_font("LeagueSpartan-Regular.ttf", 30);
    term.tell("Thus concludes the demo!");
}
