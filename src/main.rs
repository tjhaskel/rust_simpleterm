use piston_window::*;
use std::path::Path;
use std::{thread, time};

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
            "piston: hello_world",
            [800, 600]
        )
        .exit_on_esc(true)
        .build()
        .unwrap();

    let resources: &Path = Path::new("resources");
    let mut glyphs = window.load_font(resources.join("LeagueSpartan-Regular.ttf")).unwrap();

    let text: &str = "Hello world!";
    let end: usize = text.len();
    let mut marker: usize = 1;

    let mut input_string: String = String::default();

    while let Some(e) = window.next() {
        e.text(|text| input_string.push_str(text));
        e.button(|button_args| {
            if let Button::Keyboard(key) = button_args.button {
                if button_args.state == ButtonState::Press {
                    if key == Key::Backspace { input_string.pop(); }
                    if key == Key::Return { input_string = String::default(); }
                }
            }
        });

        window.draw_2d(&e, |c, g, device| {
            let mut transform = c.transform.trans(10.0, 30.0);

            clear([0.0, 0.0, 0.0, 1.0], g);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                &text[..marker],
                &mut glyphs,
                &c.draw_state,
                transform, g
            ).unwrap();

            transform = c.transform.trans(10.0, 300.0);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                &input_string[..],
                &mut glyphs,
                &c.draw_state,
                transform, g
            ).unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });

        if marker < end {
            marker += 1;
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}
