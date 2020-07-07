use simpleterm::{terminal::Terminal, DEFAULT_BGC, DEFAULT_FGC};

fn main() {
    let mut term: Terminal = Terminal::new("simpleterm test", DEFAULT_BGC, DEFAULT_FGC, "LeagueSpartan-Regular.ttf", 32);

    term.tell("This is pretty cool! Here's a really long sentence to test stuff Here's a really long sentence to test stuff Here's a really long sentence to test stuff Here's a really long sentence to test stuff\nThis should be on a new line!");
    let input_recieved: String = term.ask("Enter some input: ");
    term.set_colors(DEFAULT_FGC, DEFAULT_BGC);
    term.tell(&format!("You said: {}", input_recieved));
    term.tell("01234567890123456789012345678900123456789012345678901234567890012345678901234567890123456789001234567890123456789012345678900123456789012345678901234567890");
}
