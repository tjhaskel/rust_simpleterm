use piston_window::*;
use std::path::Path;

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

    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            let transform = c.transform.trans(10.0, 30.0);

            clear([0.0, 0.0, 0.0, 1.0], g);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                "Hello world!",
                &mut glyphs,
                &c.draw_state,
                transform, g
            ).unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    }
}
