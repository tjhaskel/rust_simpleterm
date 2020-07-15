use piston_window::{*, types::{Color, FontSize}};
use std::{thread, time::{Duration, Instant}};

use crate::{draw::*, text::*, TYPE_TIME};

/// A terminal stores a PistonWindow, background and foreground colors,
/// a font, fontsize, and glyph cache, and the current message and input strings.
pub struct Terminal {
    title: String,
    active: bool,
    /// The window that displays our terminal.
    pub window: PistonWindow,
    /// The background color of our terminal.
    pub bg_color: Color,
    /// The foreground color of our terminal.
    pub fg_color: Color,
    /// Whether or not to use scanlines
    pub scanlines: bool,
    glyphs: Glyphs,
    font: String,
    art_font: String,
    /// The font size of normal text in our terminal.
    pub font_size: FontSize,
    /// The font size of art in our terminal.
    pub art_font_size: FontSize,
    art_mode: bool,
    message: Vec<String>,
    input: String,
}

impl Terminal {
    /// Creates a new window with the given title, colors, and font info
    /// 
    /// ```no_run
    /// # use simpleterm::text::*;
    /// # use simpleterm::terminal::Terminal;
    /// let mut term: Terminal = Terminal::new("simpleterm test", (800, 600), DARK_GREY, GOLD, "LeagueSpartan-Regular.ttf", 32);
    /// ```
    pub fn new(title: &str, size: (u32, u32), bg: Color, fg: Color, font: &str, font_size: u32) -> Terminal {
        let mut new_window: PistonWindow = WindowSettings::new(title, size).exit_on_esc(true).build().unwrap();
        let loaded_glyphs = load_font(&mut new_window, font);

        Terminal {
            title: String::from(title),
            active: true,
            window: new_window,
            bg_color: bg,
            fg_color: fg,
            scanlines: true,
            glyphs: loaded_glyphs,
            font: String::from(font),
            art_font: String::from("LeagueMono-Regular.ttf"),
            font_size,
            art_font_size: 10,
            art_mode: false,
            message: Vec::new(),
            input: String::default(),
        }
    }

    /// Types out the given message, then waits for the user to type something and returns Some(input string).
    /// If the window is closed before input can be returned, returns None.
    /// 
    /// ```no_run
    /// # use simpleterm::text::*;
    /// # use simpleterm::terminal::Terminal;
    /// # let mut term: Terminal = Terminal::new("simpleterm test", (800, 600), DARK_GREY, GOLD, "LeagueSpartan-Regular.ttf", 32);
    /// let user_input: String = term.ask("This will wait for the user enter input!").unwrap();
    /// ```
    pub fn ask(&mut self, message: &str) -> Option<String> {
        if self.active {
            self.check_text_mode();
            self.new_message(message);
            self.wait_for_input();
            Some(self.input.clone())
        } else {
            None
        }
    }

    /// Displays an ascii art string centered on the terminal. This uses 10pt font and a monospace font.
    /// 
    /// ```no_run
    /// # use std::time::Duration;
    /// # use simpleterm::{art::*, text::*};
    /// # use simpleterm::terminal::Terminal;
    /// # let mut term: Terminal = Terminal::new("simpleterm test", (800, 600), DARK_GREY, GOLD, "LeagueSpartan-Regular.ttf", 32);
    /// term.display_art(GEO, Duration::from_secs(2));
    /// ```
    pub fn display_art(&mut self, art: &str, time: Duration) {
        if self.active {
            self.check_art_mode();
            self.message = art.split('\n').map(String::from).collect();
            self.input = String::default();
            self.show_art(time);
        }
    }
    
    /// Types out the given message, then waits for the given amount of time to continue.
    /// 
    /// ```no_run
    /// # use std::time::Duration;
    /// # use simpleterm::text::*;
    /// # use simpleterm::terminal::Terminal;
    /// # let mut term: Terminal = Terminal::new("simpleterm test", (800, 600), DARK_GREY, GOLD, "LeagueSpartan-Regular.ttf", 32);
    /// term.show("This will wait for 1 second!", Duration::from_secs(1));
    /// ```
    pub fn show(&mut self, message: &str, time: Duration) {
        if self.active {
            self.check_text_mode();
            self.new_message(message);
            self.wait_for_timer(time);
        }
    }

    /// Types out the given message, then waits for the user to press Enter to continue.
    /// 
    /// ```no_run
    /// # use simpleterm::text::*;
    /// # use simpleterm::terminal::Terminal;
    /// # let mut term: Terminal = Terminal::new("simpleterm test", (800, 600), DARK_GREY, GOLD, "LeagueSpartan-Regular.ttf", 32);
    /// term.tell("This will wait for the user to hit enter!");
    /// ```
    pub fn tell(&mut self, message: &str) {
        if self.active {
            self.check_text_mode();
            self.new_message(message);
            self.input = String::from("Press Enter to Continue");
            self.wait_for_continue();
        }
    }

    /// Closes the current window and creates a new one with the given (x, y) Size.
    /// 
    /// ```no_run
    /// # use simpleterm::text::*;
    /// # use simpleterm::terminal::Terminal;
    /// # let mut term: Terminal = Terminal::new("simpleterm test", (800, 600), DARK_GREY, GOLD, "LeagueSpartan-Regular.ttf", 32);
    /// term.resize((800, 600).into());
    /// ```
    pub fn resize(&mut self, new_size: Size) {
        if self.active {
            let new_window: PistonWindow = WindowSettings::new(self.title.clone(), new_size).exit_on_esc(true).build().unwrap();
            self.window = new_window;
        }
    }

    /// Loads a new font from the given font filename and sets the given font size
    /// 
    /// ```no_run
    /// # use simpleterm::text::*;
    /// # use simpleterm::terminal::Terminal;
    /// # let mut term: Terminal = Terminal::new("simpleterm test", (800, 600), DARK_GREY, GOLD, "LeagueSpartan-Regular.ttf", 32);
    /// term.set_font("LeagueSpartan-Regular.ttf", 24);
    /// ```
    pub fn set_font(&mut self, font: &str, size: FontSize) {
        if self.active {
            if !self.art_mode { self.glyphs = load_font(&mut self.window, font); }
            self.font = String::from(font);
            self.font_size = size;
        }
    }

    /// Loads a new art font from the given font filename and sets the given font size.
    /// You probably want to use a mono-space font here, and a small size.
    /// 
    /// The default is LeagueMono-Regular.ttf at 10pt.
    /// 
    /// ```no_run
    /// # use simpleterm::text::*;
    /// # use simpleterm::terminal::Terminal;
    /// # let mut term: Terminal = Terminal::new("simpleterm test", (800, 600), DARK_GREY, GOLD, "LeagueSpartan-Regular.ttf", 32);
    /// term.set_art_font("LeagueMono-Regular.ttf", 10);
    /// ```
    pub fn set_art_font(&mut self, font: &str, size: FontSize) {
        if self.active {
            if self.art_mode { self.glyphs = load_font(&mut self.window, font); }
            self.art_font = String::from(font);
            self.art_font_size = size;
        }
    }

    /// Changes the terminal's background and foreground to the given colors. The change will be apparent in the next text command.
    /// 
    /// ```no_run
    /// # use simpleterm::text::*;
    /// # use simpleterm::terminal::Terminal;
    /// # let mut term: Terminal = Terminal::new("simpleterm test", (800, 600), DARK_GREY, GOLD, "LeagueSpartan-Regular.ttf", 32);
    /// term.set_colors(DARK_GREY, CRIMSON);
    /// ```
    pub fn set_colors(&mut self, bgc: Color, fgc: Color) {
        self.bg_color = bgc;
        self.fg_color = fgc;
    }

    fn check_art_mode(&mut self) {
        if !self.art_mode {
            self.glyphs = load_font(&mut self.window, &self.art_font);
            self.art_mode = true;
        }
    }

    fn check_text_mode(&mut self) {
        if self.art_mode {
            self.glyphs = load_font(&mut self.window, &self.font);
            self.art_mode = false;
        }
    }

    // Displays an art string along with the rest of the terminal.
    fn show_art(&mut self, timer: Duration) {
        let bgc: Color = self.bg_color;
        let fgc: Color = self.fg_color;

        let art: &Vec<String> = &self.message;
        let glyphs: &mut Glyphs = &mut self.glyphs;
        let font_size: FontSize = self.art_font_size;
        let use_filter: bool = self.scanlines;
        
        let start: Instant = Instant::now();
        let mut active: bool = self.active;
        while let Some(e) = self.window.next() {
            e.close(|_| { active = false; });

            let win_size: Size = self.window.window.size();

            let now: Instant = Instant::now();
            if now.duration_since(start) > timer { break; }

            self.window.draw_2d(&e, |c, g, device| {
                clear(bgc, g);

                draw_background(win_size, bgc, fgc, use_filter, c, g);
                draw_art(win_size, art, glyphs, font_size, fgc, c, g);
                draw_foreground(win_size, bgc, use_filter, c, g);
            
                glyphs.factory.encoder.flush(device);
            });
        }
        self.active = active;
    }

    // Types a message one character at a time, waiting TYPE_TIME between each character.
    fn type_message(&mut self) {
        let bgc: Color = self.bg_color;
        let fgc: Color = self.fg_color;
        let current_input: &str = &(self.input[..]);
        let glyphs = &mut self.glyphs;
        let font_size: FontSize = self.font_size;

        let mut typed_message: Vec<String> = Vec::new();
        let use_filter: bool = self.scanlines;

        let mut active: bool = self.active;
        for (i, line) in self.message.iter().enumerate() {
            typed_message.push(String::default());

            let line_len: usize = line.len();
            for j in 1..line_len {
                typed_message[i] = String::from(&line[..=j]);
                typed_message[i].push_str("[]");
                if let Some(e) = self.window.next() {
                    e.close(|_| { active = false; });

                    let win_size: Size = self.window.window.size();

                    self.window.draw_2d(&e, |c, g, device| {
                        clear(bgc, g);

                        draw_background(win_size, bgc, fgc, use_filter, c, g);
                        draw_message(&typed_message, glyphs, font_size, fgc, c, g);
                        draw_input(win_size, current_input, glyphs, font_size, fgc, c, g);
                        draw_foreground(win_size, bgc, use_filter, c, g);
                    
                        glyphs.factory.encoder.flush(device);
                    });
                    thread::sleep(TYPE_TIME);
                }
                typed_message[i].pop();
                typed_message[i].pop();
            }
        }
        self.active = active;
    }

    // Displays the current terminal until the user presses Enter.
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
        let mut active: bool = self.active;
        while let Some(e) = self.window.next() {
            e.close(|_| { active = false; });

            let win_size: Size = self.window.window.size();

            e.button(|button_args| {
                if let Button::Keyboard(key) = button_args.button {
                    if button_args.state == ButtonState::Press && key == Key::Return { ready = true; }
                }
            });

            if ready { break; }

            let now: Instant = Instant::now();
            self.window.draw_2d(&e, |c, g, device| {
                clear(bgc, g);

                draw_background(win_size, bgc, fgc, use_filter, c, g);
                draw_message(message, glyphs, font_size, fgc, c, g);
                draw_input_marker(win_size, glyphs, font_size, fgc, c, g);
                if check_flash(now, &mut start) { draw_input(win_size, current_input, glyphs, font_size, fgc, c, g); }
                draw_foreground(win_size, bgc, use_filter, c, g);
            
                glyphs.factory.encoder.flush(device);
            });
        }
        self.active = active;
    }

    // Displays the current terminal until the user submits some input.
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
        let mut active: bool = self.active;
        while let Some(e) = self.window.next() {
            e.close(|_| { active = false; });

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

                draw_background(win_size, bgc, fgc, use_filter, c, g);
                draw_message(message, glyphs, font_size, fgc, c, g);
                draw_input_marker(win_size, glyphs, font_size, fgc, c, g);

                if check_flash(now, &mut start) {
                    input_string.push_str("[]");
                    draw_input(win_size, &input_string[..], glyphs, font_size, fgc, c, g);
                    input_string.pop();
                    input_string.pop();
                } else {
                    draw_input(win_size, &input_string[..], glyphs, font_size, fgc, c, g);
                }
                
                draw_foreground(win_size, bgc, use_filter, c, g);
            
                glyphs.factory.encoder.flush(device);
            });

            if input_accepted { break; }
        }
        self.active = active;
    }

    // Displays an the current terminal until the timer runs out.
    fn wait_for_timer(&mut self, timer: Duration) {
        let bgc: Color = self.bg_color;
        let fgc: Color = self.fg_color;

        let message: &Vec<String> = &self.message;
        let glyphs: &mut Glyphs = &mut self.glyphs;
        let font_size: FontSize = self.font_size;
        let use_filter: bool = self.scanlines;
        
        let start: Instant = Instant::now();
        let mut active: bool = self.active;
        while let Some(e) = self.window.next() {
            e.close(|_| { active = false; });

            let win_size: Size = self.window.window.size();

            let now: Instant = Instant::now();
            if now.duration_since(start) > timer { break; }

            self.window.draw_2d(&e, |c, g, device| {
                clear(bgc, g);

                draw_background(win_size, bgc, fgc, use_filter, c, g);
                draw_message(message, glyphs, font_size, fgc, c, g);
                draw_foreground(win_size, bgc, use_filter, c, g);
            
                glyphs.factory.encoder.flush(device);
            });
        }
        self.active = active;
    }

    // Processes a new message and types it out.
    fn new_message(&mut self, message: &str) {
        self.message = message.split('\n').map(String::from).collect();
        self.process_message();
        self.input = String::default();
        self.type_message();
    }

    // Splits a message into a vector of strings that can fit in the current window's bounds.
    fn process_message(&mut self) {
        let max_chars: usize = self.get_max_characters();

        let mut new_message_vec: Vec<String> = Vec::new();

        for old_message in self.message.iter() {
            let mut new_message: String = String::new();

            for word in old_message.split_whitespace() {
                let word_len: usize = word.len();
                let message_len: usize = new_message.len();

                if word_len > max_chars {
                    if message_len > 0 {
                        let word_vec = split_word(word, max_chars - (message_len + 1), max_chars);
                        let mut word_iter = word_vec.iter();
                        new_message_vec.push(format!("{} {}", new_message, word_iter.next().unwrap()));
                        for continued_word in word_iter {
                            new_message_vec.push(continued_word.to_string());
                        }
                        new_message = new_message_vec.pop().unwrap();
                    } else {
                        new_message_vec.append(&mut split_word(word, max_chars, max_chars));
                    }
                } else if message_len + word_len > max_chars {
                    new_message_vec.push(new_message);
                    new_message = String::from(word);
                } else if message_len > 0 {
                    new_message = format!("{} {}", new_message, word);
                } else {
                    new_message = String::from(word);
                }
            }
            if !new_message.is_empty() { new_message_vec.push(new_message); }
        }
        self.message = new_message_vec;
    }

    // Determines the max number of characters based on window and font size.
    fn get_max_characters(&self) -> usize {
        ((self.window.window.size().width / self.font_size as f64) * 2.15) as usize
    }
}
