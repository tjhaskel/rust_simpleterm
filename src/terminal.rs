use piston_window::*;
use std::path::Path;
use std::{thread, time};

pub struct Terminal {
    window: PistonWindow,
    message: String,
    input: String,
}

impl Terminal {
    pub fn new(title: &str) -> Terminal {
        Terminal {
            window: WindowSettings::new(title, [800, 600])
                    .exit_on_esc(true)
                    .build()
                    .unwrap(),
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
        let resources: &Path = Path::new("resources");
        let mut glyphs = self.window.load_font(resources.join("LeagueSpartan-Regular.ttf")).unwrap();
        let current_input: &str = &(self.input[..]);

        let message_len: usize = self.message.len();
        for i in 1..message_len {
            let new_message: &str = &(self.message[..i]);
            if let Some(e) = self.window.next() {
                self.window.draw_2d(&e, |c, g, device| {
                    clear([0.0, 0.0, 0.0, 1.0], g);

                    text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                        new_message,
                        &mut glyphs,
                        &c.draw_state,
                        c.transform.trans(10.0, 30.0), g
                    ).unwrap();
                
                    text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                        current_input,
                        &mut glyphs,
                        &c.draw_state,
                        c.transform.trans(10.0, 300.0), g
                    ).unwrap();
                
                    glyphs.factory.encoder.flush(device);
                });
                thread::sleep(time::Duration::from_millis(20));
            }
        }
    }

    fn wait_for_continue(&mut self) {
        let mut ready: bool = false;

        let resources: &Path = Path::new("resources");
        let mut glyphs = self.window.load_font(resources.join("LeagueSpartan-Regular.ttf")).unwrap();
        let current_message: &str = &(self.message);
        let current_input: &str = &(self.input);
        
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
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(10.0, 30.0), g
                ).unwrap();
            
                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                    current_input,
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(10.0, 300.0), g
                ).unwrap();
            
                // Update glyphs before rendering.
                glyphs.factory.encoder.flush(device);
            });
        }
    }

    fn wait_for_input(&mut self) {
        let mut input_string: String = String::default();
        let mut input_accepted: bool = false;

        let resources: &Path = Path::new("resources");
        let mut glyphs = self.window.load_font(resources.join("LeagueSpartan-Regular.ttf")).unwrap();
        let current_message: &str = &(self.message);
        
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
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(10.0, 30.0), g
                ).unwrap();
            
                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                    &input_string[..],
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(10.0, 300.0), g
                ).unwrap();
            
                // Update glyphs before rendering.
                glyphs.factory.encoder.flush(device);
            });

            if input_accepted { break; }
        }
    }
}
