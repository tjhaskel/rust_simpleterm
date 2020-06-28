use piston_window::{*, types::Color};
use std::path::Path;
use std::{thread, time::Instant};

use crate::{draw::*, TYPE_TIME};

pub struct Terminal {
    window: PistonWindow,
    bg_color: Color,
    fg_color: Color,
    glyphs: Glyphs,
    message: Vec<String>,
    input: String,
    scanlines: bool,
}

impl Terminal {
    pub fn new(title: &str, bg: Color, fg: Color, font: &str) -> Terminal {
        let mut new_window: PistonWindow = WindowSettings::new(title, [800, 600]).exit_on_esc(true).build().unwrap();
        let resources: &Path = Path::new("resources");
        let loaded_glyphs = new_window.load_font(resources.join(font)).unwrap();

        Terminal {
            window: new_window,
            bg_color: bg,
            fg_color: fg,
            glyphs: loaded_glyphs,
            message: Vec::new(),
            input: String::default(),
            scanlines: true,
        }
    }

    pub fn scan_lines(&mut self, enabled: bool) {
        self.scanlines = enabled;
    }

    pub fn tell(&mut self, message: &str) {
        self.new_message(message);
        self.input = String::from("Press Enter to Continue");
        self.wait_for_continue();
    }

    pub fn ask(&mut self, message: &str) -> String {
        self.new_message(message);
        self.wait_for_input();
        self.get_input()
    }

    fn new_message(&mut self, message: &str) {
        self.message = message.split("\n").map(|x| String::from(x)).collect();
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

                        display_box(win_size, bgc, fgc, c, g);
                        display_message(&typed_message, glyphs, fgc, c, g);
                        display_input(win_size, current_input, glyphs, fgc, c, g);
                        if use_filter { display_filter(win_size, bgc, fgc, c, g); }
                    
                        glyphs.factory.encoder.flush(device);
                    });
                    thread::sleep(TYPE_TIME);
                }
                typed_message[i].pop();
                typed_message[i].pop();
            }
        }
    }

    fn wait_for_continue(&mut self) {
        let mut ready: bool = false;

        let bgc: Color = self.bg_color;
        let fgc: Color = self.fg_color;
        let message: &Vec<String> = &self.message;
        let current_input: &str = &(self.input);
        let glyphs: &mut Glyphs = &mut self.glyphs;
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
                display_message(message, glyphs, fgc, c, g);
                display_input_marker(win_size, glyphs, fgc, c, g);
                if check_flash(now, &mut start) { display_input(win_size, current_input, glyphs, fgc, c, g); }
                if use_filter { display_filter(win_size, bgc, fgc, c, g); }
            
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
                display_message(message, glyphs, fgc, c, g);
                display_input_marker(win_size, glyphs, fgc, c, g);

                if check_flash(now, &mut start) {
                    input_string.push_str("[]");
                    display_input(win_size, &input_string[..], glyphs, fgc, c, g);
                    input_string.pop();
                    input_string.pop();
                } else {
                    display_input(win_size, &input_string[..], glyphs, fgc, c, g);
                }
                
                if use_filter { display_filter(win_size, bgc, fgc, c, g); }
            
                glyphs.factory.encoder.flush(device);
            });

            if input_accepted { break; }
        }
    }
}
