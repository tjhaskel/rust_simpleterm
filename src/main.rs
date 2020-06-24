use simpleterm::terminal::Terminal;

fn main() {
    let mut term: Terminal = Terminal::new("simpleterm test");

    term.tell("This is pretty cool! Here's a really long sentence to test stuff");
    let input_recieved: String = term.ask("Enter some input: ");
    term.tell(&format!("You said: {}", input_recieved));
}
