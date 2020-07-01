//! This example demonstrates how to use `Text` to draw TrueType font texts efficiently.

use cgmath;
use ggez;

use cgmath::Point2;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::graphics::{self, Align, Color, DrawParam, Font, Scale, Text, TextFragment};
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use std::env;
use std::f32;
use std::path;

struct Terminal {
    message: Vec<Text>,
    input: Text,
    font: Font,
    font_size: Scale,
    bg_color: Color,
    fg_color: Color,
    scan_lines: bool,
}

impl Terminal {
    pub fn new(ctx: &mut Context, font_file: &str, font_size: f32, bgc: Color, fcg: Color) -> GameResult<Terminal> {
        Ok( Terminal {
            message: Vec::new(),
            input: Text::default(),
            font: Font::new(ctx, font_file)?,
            font_size: Scale::uniform(font_size),
            bg_color: bgc,
            fg_color: fcg,
            scan_lines: true,
        })
    }

    pub fn say(&mut self, ctx: &mut Context, text: &str) {
        let new_message = Text::new( TextFragment {
            text: text.to_string(),
            color: Some(self.fg_color),
            font: Some(self.font),
            scale: Some(self.font_size),
        });

        self.message = vec!(new_message);
    }
}

impl event::EventHandler for Terminal {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, self.bg_color);

        let mut height = 0.0;
        for text in &self.message {
            graphics::queue_text(ctx, text, Point2::new(20.0, 20.0 + height), None);
            height += 20.0 + text.height(ctx) as f32;
        }

        graphics::draw_queued_text(
            ctx,
            DrawParam::default(),
            None,
            graphics::FilterMode::Linear,
        )?;

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        for message in self.message.iter_mut() {
            message.set_bounds(Point2::new(width - 40.0, 400.0), Align::Left);
        }

        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (ctx, events_loop) = &mut ContextBuilder::new("simpleterm example", "simpleterm")
        .window_setup(WindowSetup::default().title("simpleterm example"))
        .window_mode(
            WindowMode::default()
                .dimensions(800.0, 600.0)
                .resizable(true),
        )
        .add_resource_path(resource_dir)
        .build()?;

    let term = &mut Terminal::new(ctx, "/LeagueSpartan-Regular.ttf", 24.0, graphics::BLACK, graphics::WHITE)?;
    term.say(ctx, "MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT MESSAGE TEXT");
    event::run(ctx, events_loop, term)
}
