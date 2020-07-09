use std::time::Duration;

use simpleterm::{art::*, text::*, terminal::Terminal};

fn main() {
    let mut term: Terminal = Terminal::new("simpleterm test", DARK_GREY, GOLD, "LeagueSpartan-Regular.ttf", 32);

    term.display_art(GEO, Duration::from_secs(2));
    term.show("Welcome to Simpleterm!", Duration::from_secs(2));
    term.tell("This is pretty cool! Here's a really long word to test stuff: 0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789
        \nThis should be on a new line!");

    term.set_colors(OFF_WHITE, EMERALD);
    let input_recieved: String = term.ask("Enter some input: ");

    term.set_fgc(DARK_PURPLE);
    term.tell(&format!("You said: {}", input_recieved));

    term.set_colors(DARK_GREY, LIGHT_BLUE);
    term.display_art(MONA, Duration::from_secs(2));

    term.set_fgc(CRIMSON);
    term.tell("Thus concludes the demo!");
}
