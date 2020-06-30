use simpleterm::{color::*, terminal::Terminal};

fn main() {
    let fgc = [0.4, 0.8, 1.0, 1.0];
    let mut term: Terminal = Terminal::new("simpleterm test", [0.16, 0.16, 0.16, 1.0], fgc, "LeagueSpartan-Regular.ttf", 24);

    term.tell("This is pretty cool! \"Here's a really long sentence\" to test stuff\nThis should be on a new line!", YELLOW);
    let input_recieved: String = term.ask("Enter some input: ", fgc);
    term.tell(&format!("You said: {}", input_recieved), fgc);
    term.tell("01234567890123456789012345678900123456789012345678901234567890012345678901234567890123456789001234567890123456789012345678900123456789012345678901234567890", fgc);
}
