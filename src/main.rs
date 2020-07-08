use std::time::Duration;

use simpleterm::{art::*, color::*, terminal::Terminal};

fn main() {
    let mut term: Terminal = Terminal::new("simpleterm test", DARK_GREY, LIGHT_PURPLE, "LeagueSpartan-Regular.ttf", 32);

    term.display_art(GEO, Duration::from_secs(2));
    term.show("Welcome to Simpleterm!", Duration::from_secs(2));
    term.tell("This is pretty cool! Here's a really long sentence to test stuff Here's a really long sentence to test stuff Here's a really long sentence to test stuff Here's a really long sentence to test stuff\nThis should be on a new line!");
    let input_recieved: String = term.ask("Enter some input: ");
    term.set_colors(OFF_WHITE, DARK_PURPLE);
    term.tell(&format!("You said: {}", input_recieved));
    term.show("01234567890123456789012345678900123456789012345678901234567890012345678901234567890123456789001234567890123456789012345678900123456789012345678901234567890", Duration::from_millis(500));
    term.set_colors(DARK_GREY, LIGHT_BLUE);
    term.tell("Thus concludes the demo!");
    term.display_art(GEO, Duration::from_secs(2));
}
