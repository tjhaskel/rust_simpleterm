use piston_window::*;
use std::path::Path;
use std::{thread, time::Duration};

const TYPE_TIME: Duration = Duration::from_millis(30);
const MESSAGE_LOC: (f64, f64) = (10.0, 30.0);
const INPUT_LOC: (f64, f64) = (10.0, 580.0);

pub struct Terminal {
    window: PistonWindow,
    glyphs: Glyphs,
    message: String,
    input: String,
}

impl Terminal {
    pub fn new(title: &str) -> Terminal {
        let mut new_window: PistonWindow = WindowSettings::new(title, [800, 600]).exit_on_esc(true).build().unwrap();
        let resources: &Path = Path::new("resources");
        let loaded_glyphs = new_window.load_font(resources.join("LeagueSpartan-Regular.ttf")).unwrap();

        Terminal {
            window: new_window,
            glyphs: loaded_glyphs,
            message: String::default(),
            input: String::default(),
        }
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
        self.message = String::from(message);
        self.write_message();
    }

    fn get_input(&self) -> String {
        self.input.clone()
    }

    fn write_message(&mut self) {
        let current_input: &str = &(self.input[..]);
        let glyphs = &mut self.glyphs;

        let message_len: usize = self.message.len();
        for i in 1..message_len {
            let new_message: &str = &(self.message[..i]);
            if let Some(e) = self.window.next() {
                self.window.draw_2d(&e, |c, g, device| {
                    clear([0.0, 0.0, 0.0, 1.0], g);

                    text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                        new_message,
                        glyphs,
                        &c.draw_state,
                        c.transform.trans(MESSAGE_LOC.0, MESSAGE_LOC.1), g
                    ).unwrap();
                
                    text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                        current_input,
                        glyphs,
                        &c.draw_state,
                        c.transform.trans(INPUT_LOC.0, INPUT_LOC.1), g
                    ).unwrap();
                
                    glyphs.factory.encoder.flush(device);
                });
                thread::sleep(TYPE_TIME);
            }
        }
    }

    fn wait_for_continue(&mut self) {
        let mut ready: bool = false;

        let current_message: &str = &(self.message);
        let current_input: &str = &(self.input);
        let glyphs = &mut self.glyphs;
        
        while let Some(e) = self.window.next() {
            e.button(|button_args| {
                if let Button::Keyboard(key) = button_args.button {
                    if button_args.state == ButtonState::Press {
                        if key == Key::Return { ready = true; }
                    }
                }
            });

            if ready { break; }
            
            self.window.draw_2d(&e, |c, g, device| {
                clear([0.0, 0.0, 0.0, 1.0], g);

                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                    current_message,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(MESSAGE_LOC.0, MESSAGE_LOC.1), g
                ).unwrap();
            
                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                    current_input,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(INPUT_LOC.0, INPUT_LOC.1), g
                ).unwrap();
            
                // Update glyphs before rendering.
                glyphs.factory.encoder.flush(device);
            });
        }
    }

    fn wait_for_input(&mut self) {
        let mut input_string: String = String::default();
        let mut input_accepted: bool = false;

        let current_message: &str = &(self.message);
        let glyphs = &mut self.glyphs;
        
        while let Some(e) = self.window.next() {
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
                clear([0.0, 0.0, 0.0, 1.0], g);

                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                    current_message,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(MESSAGE_LOC.0, MESSAGE_LOC.1), g
                ).unwrap();
            
                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                    &input_string[..],
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(INPUT_LOC.0, INPUT_LOC.1), g
                ).unwrap();
            
                // Update glyphs before rendering.
                glyphs.factory.encoder.flush(device);
            });

            if input_accepted { break; }
        }
    }
}
