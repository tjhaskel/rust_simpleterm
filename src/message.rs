use piston_window::{*, types::{Color, FontSize}};
use std::thread;

use crate::{TEXT_OFFSET, TYPE_TIME};

pub struct Message {
    text: String,
    color: Color,
    message_type: MessageType,
}

#[derive(Clone, Copy, PartialEq)]
pub enum MessageType {
    Normal,
    Quote,
    ContinuedQuote
}

impl Message {
    pub fn new(text: &str, color: Color, quote: bool) -> Message {
        Message {
            text: String::from(text),
            color: color,
            message_type: if quote { MessageType::Quote } else { MessageType::Normal },
        }
    }

    pub fn default() -> Message {
        Message::new("", [1.0, 1.0, 1.0, 1.0], false)
    }

    pub fn display_message(&self, glyphs: &mut Glyphs, font_size: FontSize, fgc: Color, y_offset: f64, fast: bool, context: Context, graphics: &mut G2d) {
        let color: Color = self.color;
        let mut use_color = self.start_colored();

        let mut x_offset: f64 = 0.0;
        for c in self.text.chars() {
            if c == '"' && self.has_quote() {
                text::Text::new_color(color, font_size).draw(
                    c.encode_utf8(&mut [0; 1]),
                    glyphs,
                    &context.draw_state,
                    context.transform.trans(TEXT_OFFSET.0 + x_offset, TEXT_OFFSET.1 + y_offset),
                    graphics,
                ).unwrap();
                use_color = !use_color;
            } else {
                if use_color {
                    text::Text::new_color(color, font_size).draw(
                        c.encode_utf8(&mut [0; 1]),
                        glyphs,
                        &context.draw_state,
                        context.transform.trans(TEXT_OFFSET.0 + x_offset, TEXT_OFFSET.1 + y_offset),
                        graphics,
                    ).unwrap();
                } else {
                    text::Text::new_color(fgc, font_size).draw(
                        c.encode_utf8(&mut [0; 1]),
                        glyphs,
                        &context.draw_state,
                        context.transform.trans(TEXT_OFFSET.0 + x_offset, TEXT_OFFSET.1 + y_offset),
                        graphics,
                    ).unwrap();
                }
            }
            match c {
                '!' | 'I' | 'i' | 'j' | 'l' | '\'' => { x_offset += font_size as f64 / 4.0; }
                '"' | '1' | ' ' | 'f' | 'r' | 's' | 't'=> { x_offset += font_size as f64 / 3.0; }
                'F' | 'J' | 'L' | 'c' | 'k' | 'u' | 'x' | 'y' | 'z' => { x_offset += font_size as f64 / 2.5; }
                'G' | 'M' | 'N' | 'O' | 'Q' | 'W' | 'm' | 'w' => { x_offset += font_size as f64 / 1.5; }
                _ => { x_offset += font_size as f64 / 2.0; }
            }
           
            if !fast { thread::sleep(TYPE_TIME); }
        }
    }

    fn has_quote(&self) -> bool {
        !(self.message_type == MessageType::Normal)
    }

    fn start_colored(&self) -> bool {
        !(self.message_type == MessageType::Quote)
    }

    fn process_message(&self, max_chars: usize) -> Vec<Message> {
        let mut new_messages: Vec<Message> = Vec::new();
        let color: Color = self.color;
        let message_type: MessageType = self.message_type;

        let mut new_message: String = String::new();

        for word in self.text.split_whitespace() {
            if word.len() > max_chars {
                new_messages.append(&mut split_every_nth(word, max_chars, color, message_type));
            } else if new_message.len() + word.len() > max_chars {
                new_messages.push(Message { text: new_message, color: color, message_type: message_type });
                new_message = String::from(word);
            } else {
                new_message = format!("{} {}", new_message, word);
            }
        }
        new_messages.push(Message { text: new_message, color: color, message_type: message_type });

        new_messages
    }
}

pub fn display_messages(message: &Vec<Message>, glyphs: &mut Glyphs, font_size: FontSize, fgc: Color, context: Context, graphics: &mut G2d)  {
    let mut y_offset: f64 = 0.0;
    for line in message.iter() {
        line.display_message(glyphs, font_size, fgc, y_offset, true, context, graphics);
        y_offset += font_size as f64;
    }
}

pub fn type_messages(message: &Vec<Message>, glyphs: &mut Glyphs, font_size: FontSize, fgc: Color, context: Context, graphics: &mut G2d)  {
    let mut y_offset: f64 = 0.0;
    for line in message.iter() {
        line.display_message(glyphs, font_size, fgc, y_offset, false, context, graphics);
        y_offset += font_size as f64;
    }
}

pub fn process_messages(message_vec: &Vec<Message>, max_chars: usize) -> Vec<Message> {
    let mut new_messages: Vec<Message> = Vec::new();

    for old_message in message_vec.iter() {
        new_messages.append(&mut old_message.process_message(max_chars));
    }

    new_messages
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
