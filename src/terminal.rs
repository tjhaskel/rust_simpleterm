use piston_window::{*, types::Color};
use std::path::Path;
use std::{thread, time::Duration};

const TYPE_TIME: Duration = Duration::from_millis(30);
const TEXT_OFFSET: (f64, f64) = (25.0, 50.0);

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
                if let Some(e) = self.window.next() {
                    let win_size: Size = self.window.window.size();

                    self.window.draw_2d(&e, |c, g, device| {
                        clear(bgc, g);

                        display_box(win_size, bgc, fgc, c, g);
                        display_message(&typed_message, glyphs, fgc, c, g);
                        display_input(win_size, current_input, glyphs, fgc, c, g);
                        if use_filter { display_filter(win_size, true, bgc, fgc, c, g); }
                    
                        glyphs.factory.encoder.flush(device);
                    });
                    thread::sleep(TYPE_TIME);
                }
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
            
            self.window.draw_2d(&e, |c, g, device| {
                clear(bgc, g);

                display_box(win_size, bgc, fgc, c, g);
                display_message(message, glyphs, fgc, c, g);
                display_input(win_size, current_input, glyphs, fgc, c, g);
                if use_filter { display_filter(win_size, true, bgc, fgc, c, g); }
            
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
            
            self.window.draw_2d(&e, |c, g, device| {
                clear(bgc, g);

                display_box(win_size, bgc, fgc, c, g);
                display_message(message, glyphs, fgc, c, g);
                display_input(win_size, &input_string[..], glyphs, fgc, c, g);
                if use_filter { display_filter(win_size, true, bgc, fgc, c, g); }
            
                glyphs.factory.encoder.flush(device);
            });

            if input_accepted { break; }
        }
    }
}

fn display_box(win_size: Size, bcg: Color, fgc: Color, context: Context, graphics: &mut G2d) {
    rectangle(fgc, [10.0, 10.0, win_size.width - 20.0, win_size.height - 20.0], context.transform, graphics);
    rectangle(bcg, [15.0, 15.0, win_size.width - 30.0, win_size.height - 30.0], context.transform, graphics);
}

fn display_message(message: &Vec<String>, glyphs: &mut Glyphs, fgc: Color, context: Context, graphics: &mut G2d)  {
    let x = TEXT_OFFSET.0;
    let y = TEXT_OFFSET.1;

    let mut y_offset: f64 = 0.0;
    for line in message.iter() {
        text::Text::new_color(fgc, 32).draw(
            line,
            glyphs,
            &context.draw_state,
            context.transform.trans(x, y + y_offset),
            graphics,
        ).unwrap();

        y_offset += 30.0;
    }
}

fn display_input(win_size: Size, message: &str, glyphs: &mut Glyphs, fgc: Color, context: Context, graphics: &mut G2d)  {
    let x = TEXT_OFFSET.0;
    let y = (win_size.height - TEXT_OFFSET.1) + 20.0;

    text::Text::new_color(fgc, 32).draw(
        message,
        glyphs,
        &context.draw_state,
        context.transform.trans(x, y),
        graphics,
    ).unwrap();
}

fn display_filter(win_size: Size, contain_box: bool, bgc: Color, fgc: Color, context: Context, graphics: &mut G2d) {
    let mut line_color: Color = [bgc[0] - 0.2, bgc[1] - 0.2, bgc[2] - 0.2, 0.5];
    if color_brighter(fgc, bgc) {
        line_color = [line_color[0] - 0.2, line_color[1] - 0.2, line_color[2] - 0.2, 0.5];
    } else {
        line_color = [line_color[0] + 0.15, line_color[1] + 0.15, line_color[2] + 0.15, 0.4];
    }
    
    if contain_box {
        for i in 0..((win_size.height - 30.0) as i32 / 5) {
            rectangle(line_color, [15.0, (i * 5) as f64 + 15.0, win_size.width - 30.0, 2.0], context.transform, graphics);
        }
    } else {
        for i in 0..(win_size.height as i32 / 5) {
            rectangle(line_color, [0.0, (i * 5) as f64, win_size.width, 2.0], context.transform, graphics);
        }
    }
}

fn color_brighter(a: Color, b: Color) -> bool {
    brightness(a) > brightness(b)
}

fn brightness(c: Color) -> f32 {
    let weighted_add: f32 =
        (c[0] * c[0] * 0.241) +
        (c[1] * c[1] * 0.691) +
        (c[2] * c[2] * 0.068);

    weighted_add.sqrt() * c[3]
}
