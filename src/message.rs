use piston_window::types::Color;

pub struct Message {
    text: String,
    color: Color,
    message_type: MessageType,
}

#[derive(Clone, Copy)]
pub enum MessageType {
    normal,
    quote,
    continued_quote
}

fn process_message(message_vec: Vec<Message>, max_chars: usize) -> Vec<Message> {
    let mut new_message_vec: Vec<Message> = Vec::new();

    for old_message in message_vec.iter() {
        let color: Color = old_message.color;
        let message_type: MessageType = old_message.message_type;

        let mut new_message: String = String::new();

        for word in old_message.text.split_whitespace() {
            if word.len() > max_chars {
                new_message_vec.append(&mut split_every_nth(word, max_chars, color, message_type));
            } else if new_message.len() + word.len() > max_chars {
                new_message_vec.push(Message { text: new_message, color: color, message_type: message_type });
                new_message = String::from(word);
            } else {
                new_message = format!("{} {}", new_message, word);
            }
        }

        new_message_vec.push(Message { text: new_message, color: color, message_type: message_type });
    }

    new_message_vec
}

fn split_every_nth(x: &str, n: usize, c: Color, t: MessageType) -> Vec<Message> {
    let mut result: Vec<Message> = Vec::new();

    let mut count: usize = 0;
    let mut current_string: String = String::default();
    for ch in x.chars() {
        if count >= n {
            result.push(Message { text: current_string, color: c, message_type: t });
            current_string = format!("{}", ch);
            count = 1;
        } else {
            current_string.push(ch);
            count += 1;
        }
    }
    result.push(Message { text: current_string, color: c, message_type: t });

    result
}
