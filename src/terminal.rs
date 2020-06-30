use piston_window::{*, types::{Color, FontSize}};
use std::path::Path;
use std::{thread, time::{Duration, Instant}};

use crate::{draw::*, message::*, TYPE_TIME};

pub struct Terminal {
    window: PistonWindow,
    bg_color: Color,
    fg_color: Color,
    font_size: FontSize,
    glyphs: Glyphs,
    message: Vec<Message>,
    input: String,
    scanlines: bool,
}

impl Terminal {
    pub fn new(title: &str, bg: Color, fg: Color, font: &str, font_size: u32) -> Terminal {
        let mut new_window: PistonWindow = WindowSettings::new(title, [800, 600]).exit_on_esc(true).build().unwrap();
        let resources: &Path = Path::new("resources");
        let loaded_glyphs = new_window.load_font(resources.join(font)).unwrap();

        Terminal {
            window: new_window,
            bg_color: bg,
            fg_color: fg,
            font_size: font_size,
            glyphs: loaded_glyphs,
            message: Vec::new(),
            input: String::default(),
            scanlines: true,
        }
    }

    pub fn scan_lines(&mut self, enabled: bool) {
        self.scanlines = enabled;
    }

    pub fn tell(&mut self, message: &str, color: Color) {
        self.display(message, color, contains_quote(message), false, true);
    }

    pub fn show(&mut self, message: &str, color: Color, time: f64) {
        self.display(message, color, contains_quote(message), true, false);
        thread::sleep(Duration::from_secs_f64(time));
    }

    pub fn display(&mut self, message: &str, color: Color, quote: bool, fast: bool, wait: bool) {
        self.new_message(message, color, quote);
        self.type_message();
        if wait { self.wait_for_continue(); }
    }

    pub fn ask(&mut self, message: &str, color: Color) -> String {
        self.display(message, color, contains_quote(message), false, false);
        self.wait_for_input();
        self.get_input()
    }

    fn new_message(&mut self, message: &str, color: Color, quote: bool) {
        for line in message.split("\n") {
            self.message.push(Message::new(line, color, quote));
        }
    }

    fn get_input(&self) -> String {
        self.input.clone()
    }

    fn type_message(&mut self) {
        self.process_message();
        let message: &Vec<Message> = &self.message;
        let bgc: Color = self.bg_color;
        let fgc: Color = self.fg_color;
        let current_input: &str = &(self.input[..]);
        let glyphs = &mut self.glyphs;
        let font_size: FontSize = self.font_size;

        let use_filter: bool = self.scanlines;

        if let Some(e) = self.window.next() {
            let win_size: Size = self.window.window.size();

            self.window.draw_2d(&e, |c, g, device| {
                clear(bgc, g);

                display_box(win_size, bgc, fgc, c, g);
                display_messages(message, glyphs, font_size, fgc, c, g);
                display_input(win_size, current_input, glyphs, font_size, fgc, c, g);
                display_filter(win_size, bgc, fgc, use_filter, c, g);
            
                glyphs.factory.encoder.flush(device);
            });
            thread::sleep(TYPE_TIME);
        }
    }

    fn wait_for_continue(&mut self) {
        let mut ready: bool = false;
        self.input = String::from("Press Enter to continue.");

        let bgc: Color = self.bg_color;
        let fgc: Color = self.fg_color;

        self.process_message();
        let message: &Vec<Message> = &self.message;
        let current_input: &str = &(self.input);
        let glyphs: &mut Glyphs = &mut self.glyphs;
        let font_size: FontSize = self.font_size;
        let use_filter: bool = self.scanlines;
        
        let mut start: Instant = Instant::now();
        while let Some(e) = self.window.next() {
            let win_size: Size = self.window.window.size();

            e.button(|button_args| {
                if let Button::Keyboard(key) = button_args.button {
                    if button_args.state == ButtonState::Press {
                        if key == Key::Return { ready = true; }
                    }
                }
            });

            if ready { break; }

            let now: Instant = Instant::now();
            self.window.draw_2d(&e, |c, g, device| {
                clear(bgc, g);

                display_box(win_size, bgc, fgc, c, g);
                display_messages(message, glyphs, font_size, fgc, c, g);
                display_input_marker(win_size, glyphs, font_size, fgc, c, g);
                if check_flash(now, &mut start) { display_input(win_size, current_input, glyphs, font_size, fgc, c, g); }
                display_filter(win_size, bgc, fgc, use_filter, c, g);
            
                glyphs.factory.encoder.flush(device);
            });
        }
    }

    fn wait_for_input(&mut self) {
        let mut input_string: String = String::default();
        let mut input_accepted: bool = false;

        let bgc: Color = self.bg_color;
        let fgc: Color = self.fg_color;

        self.process_message();
        let message: &Vec<Message> = &self.message;
        let glyphs: &mut Glyphs = &mut self.glyphs;
        let font_size: FontSize = self.font_size;
        let use_filter: bool = self.scanlines;
        
        let mut start: Instant = Instant::now();
        while let Some(e) = self.window.next() {
            let win_size: Size = self.window.window.size();

            e.text(|text| input_string.push_str(text));
            e.button(|button_args| {
                if let Button::Keyboard(key) = button_args.button {
                    if button_args.state == ButtonState::Press {
                        if key == Key::Backspace { input_string.pop(); }
                        if key == Key::Return && input_string != "" { input_accepted = true; }
                    }
                }
            });

            if input_accepted {
                self.input = input_string.clone();
                input_string = String::default();
            }
            
            let now: Instant = Instant::now();
            self.window.draw_2d(&e, |c, g, device| {
                clear(bgc, g);

                display_box(win_size, bgc, fgc, c, g);
                display_messages(message, glyphs, font_size, fgc, c, g);
                display_input_marker(win_size, glyphs, font_size, fgc, c, g);

                if check_flash(now, &mut start) {
                    input_string.push_str("[]");
                    display_input(win_size, &input_string[..], glyphs, font_size, fgc, c, g);
                    input_string.pop();
                    input_string.pop();
                } else {
                    display_input(win_size, &input_string[..], glyphs, font_size, fgc, c, g);
                }
                
                display_filter(win_size, bgc, fgc, use_filter, c, g);
            
                glyphs.factory.encoder.flush(device);
            });

            if input_accepted { break; }
        }
    }

    fn process_message(&mut self) {
        self.message = process_messages(&self.message, self.get_max_characters());
    }

    fn get_max_characters(&self) -> usize {
        ((self.window.window.size().width / self.font_size as f64) * 2.0) as usize
    }
}

fn split_every_nth(x: &str, n: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let mut count: usize = 0;
    let mut current_string: String = String::default();
    for c in x.chars() {
        if count >= n {
            result.push(current_string);
            current_string = format!("{}", c);
            count = 1;
        } else {
            current_string.push(c);
            count += 1;
        }
    }
    result.push(current_string);

    result
}

// Returns true if the line contains two quotation marks.
fn contains_quote(line: &str) -> bool {
    let mut seen_quote: bool = false;

    for c in line.chars() {
        if c == '"' && seen_quote {
            return true;
        } else {
            seen_quote = true;
        }
    }

    false
}
