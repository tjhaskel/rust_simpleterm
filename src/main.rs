use simpleterm::terminal::Terminal;

fn main() {
    let mut term: Terminal = Terminal::new("simpleterm test", [0.16, 0.16, 0.16, 1.0], [0.4, 0.8, 1.0, 1.0], "LeagueSpartan-Regular.ttf");

    term.scan_lines(false);
    term.tell("This is pretty cool! Here's a really long sentence to test stuff\nThis should be on a new line!");
    let input_recieved: String = term.ask("Enter some input: ");
    term.tell(&format!("You said: {}", input_recieved));
}
