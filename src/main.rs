use std::time::Duration;

use simpleterm::{art::*, text::*, terminal::Terminal};

fn main() {
    let mut term: Terminal = Terminal::new("simpleterm test", DARK_GREY, GOLD, "LeagueSpartan-Regular.ttf", 32);
    term.display_art(GEO, Duration::from_secs(2));

    term.show("Welcome to Simpleterm!", Duration::from_secs(2));
    term.tell("Simpleterm lets you display text. It know's how to break up long sentences and words: 0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789. Neat!
        \nThis is on a new line because it also supports \\n!");

    term.set_font("LeagueMono-Regular.ttf", 24);
    term.set_colors(OFF_WHITE, EMERALD);
    let input_recieved: String = term.ask("Enter some input: ");

    term.fg_color = DARK_PURPLE;
    term.tell(&format!("You said: {}", input_recieved));

    term.set_font("LeagueSpartan-Regular.ttf", 30);
    term.resize((600, 800).into());
    term.set_colors(DARK_GREY, CRIMSON);
    term.display_art(MONA, Duration::from_secs(2));

    term.fg_color = LIGHT_BLUE;
    term.tell("Thus concludes the demo!");
}
