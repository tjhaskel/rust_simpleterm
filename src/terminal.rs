use piston_window::{*, types::{Color, FontSize}};
use std::path::Path;
use std::{thread, time::{Duration, Instant}};

use crate::{draw::*, TYPE_TIME};

pub struct Terminal {
    window: PistonWindow,
    bg_color: Color,
    fg_color: Color,
    font_size: FontSize,
    glyphs: Glyphs,
    message: Vec<String>,
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

    pub fn set_bgc(&mut self, color: Color) {
        self.bg_color = color;
    }

    pub fn set_fgc(&mut self, color: Color) {
        self.fg_color = color;
    }

    pub fn set_colors(&mut self, bgc: Color, fgc: Color) {
        self.set_bgc(bgc);
        self.set_fgc(fgc);
    }

    pub fn scan_lines(&mut self, enabled: bool) {
        self.scanlines = enabled;
    }

    pub fn tell(&mut self, message: &str) {
        self.new_message(message);
        self.input = String::from("Press Enter to Continue");
        self.wait_for_continue();
    }

    pub fn show(&mut self, message: &str, time: Duration) {
        self.new_message(message);
        self.wait_for_timer(time);
    }

    pub fn ask(&mut self, message: &str) -> String {
        self.new_message(message);
        self.wait_for_input();
        self.get_input()
    }

    fn new_message(&mut self, message: &str) {
        self.message = message.split("\n").map(|x| String::from(x)).collect();
        self.process_message();
        self.input = String::default();
        self.type_message();
    }

    fn get_input(&self) -> String {
        self.input.clone()
    }

    fn type_message(&mut self) {
        let bgc: Color = self.bg_color;
        let fgc: Color = self.fg_color;
        let current_input: &str = &(self.input[..]);
        let glyphs = &mut self.glyphs;
        let font_size: FontSize = self.font_size;

        let mut typed_message: Vec<String> = Vec::new();
        let use_filter: bool = self.scanlines;

        for (i, line) in self.message.iter().enumerate() {
            typed_message.push(String::default());

            let line_len: usize = line.len();
            for j in 1..line_len {
                typed_message[i] = String::from(&line[..=j]);
                typed_message[i].push_str("[]");
                if let Some(e) = self.window.next() {
                    let win_size: Size = self.window.window.size();

                    self.window.draw_2d(&e, |c, g, device| {
                        clear(bgc, g);

                        display_box(win_size, bgc, fgc, use_filter, c, g);
                        display_message(&typed_message, glyphs, font_size, fgc, c, g);
                        display_input(win_size, current_input, glyphs, font_size, fgc, c, g);
                        display_filter(win_size, bgc, use_filter, c, g);
                    
                        glyphs.factory.encoder.flush(device);
                    });
                    thread::sleep(TYPE_TIME);
                }
                typed_message[i].pop();
                typed_message[i].pop();
            }
        }
    }

    fn wait_for_timer(&mut self, timer: Duration) {
        let bgc: Color = self.bg_color;
        let fgc: Color = self.fg_color;

        let message: &Vec<String> = &self.message;
        let glyphs: &mut Glyphs = &mut self.glyphs;
        let font_size: FontSize = self.font_size;
        let use_filter: bool = self.scanlines;
        
        let start: Instant = Instant::now();
        while let Some(e) = self.window.next() {
            let win_size: Size = self.window.window.size();

            let now: Instant = Instant::now();
            if now.duration_since(start) > timer { break; }

            self.window.draw_2d(&e, |c, g, device| {
                clear(bgc, g);

                display_box(win_size, bgc, fgc, use_filter, c, g);
                display_message(message, glyphs, font_size, fgc, c, g);
                display_filter(win_size, bgc, use_filter, c, g);
            
                glyphs.factory.encoder.flush(device);
            });
        }
    }

    fn wait_for_continue(&mut self) {
        let mut ready: bool = false;

        let bgc: Color = self.bg_color;
        let fgc: Color = self.fg_color;

        let message: &Vec<String> = &self.message;
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

                display_box(win_size, bgc, fgc, use_filter, c, g);
                display_message(message, glyphs, font_size, fgc, c, g);
                display_input_marker(win_size, glyphs, font_size, fgc, c, g);
                if check_flash(now, &mut start) { display_input(win_size, current_input, glyphs, font_size, fgc, c, g); }
                display_filter(win_size, bgc, use_filter, c, g);
            
                glyphs.factory.encoder.flush(device);
            });
        }
    }

    fn wait_for_input(&mut self) {
        let mut input_string: String = String::default();
        let mut input_accepted: bool = false;

        let bgc: Color = self.bg_color;
        let fgc: Color = self.fg_color;

        let message: &Vec<String> = &self.message;
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

                display_box(win_size, bgc, fgc, use_filter, c, g);
                display_message(message, glyphs, font_size, fgc, c, g);
                display_input_marker(win_size, glyphs, font_size, fgc, c, g);

                if check_flash(now, &mut start) {
                    input_string.push_str("[]");
                    display_input(win_size, &input_string[..], glyphs, font_size, fgc, c, g);
                    input_string.pop();
                    input_string.pop();
                } else {
                    display_input(win_size, &input_string[..], glyphs, font_size, fgc, c, g);
                }
                
                display_filter(win_size, bgc, use_filter, c, g);
            
                glyphs.factory.encoder.flush(device);
            });

            if input_accepted { break; }
        }
    }

    fn process_message(&mut self) {
        let max_chars: usize = self.get_max_characters();

        let mut new_message_vec: Vec<String> = Vec::new();

        for old_message in self.message.iter() {
            let mut new_message: String = String::new();

            for word in old_message.split_whitespace() {
                if word.len() > max_chars {
                    new_message_vec.append(&mut split_every_nth(word, max_chars));
                } else if new_message.len() + word.len() > max_chars {
                    new_message_vec.push(new_message);
                    new_message = String::from(word);
                } else if new_message.len() > 0 {
                    new_message = format!("{} {}", new_message, word);
                } else {
                    new_message = String::from(word);
                }
            }

            new_message_vec.push(new_message);
        }

        self.message = new_message_vec;
    }

    fn get_max_characters(&self) -> usize {
        ((self.window.window.size().width / self.font_size as f64) * 2.15) as usize
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
