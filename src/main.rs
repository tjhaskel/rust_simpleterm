use simpleterm::terminal::Terminal;

fn main() {
    let mut term: Terminal = Terminal::new("simpleterm test");

    term.tell("This is pretty cool!");
    let input_recieved: String = term.ask("Enter some input: ");
    term.tell(&format!("You said: {}", input_recieved));
}
