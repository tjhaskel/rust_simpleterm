use cgmath::Point2;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::graphics::{self, Align, Color, DrawParam, Font, Scale, Text, TextFragment};
use ggez::timer;
use ggez::{Context, ContextBuilder, event::EventsLoop, GameResult};
use std::env;
use std::f32;
use std::path;

pub struct Terminal {
    message: Vec<Text>,
    input: Text,
    font: Font,
    font_size: Scale,
    bg_color: Color,
    fg_color: Color,
    scan_lines: bool,
    state: TermState,
    timer: f64,
}

enum TermState {
    Continue,
    Typing,
    WaitContinue,
    WaitTimer,
    WaitInput,
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
            state: TermState::Continue,
            timer: 0.0,
        })
    }

    pub fn say(&mut self, ctx: &mut Context, events: &mut EventsLoop, text: &str) -> GameResult {
        let new_message = Text::new( TextFragment {
            text: text.to_string(),
            color: Some(self.fg_color),
            font: Some(self.font),
            scale: Some(self.font_size),
        });

        self.message = vec!(new_message);

        event::run(ctx, events, self)
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

pub fn new_window(title: &str) -> GameResult<(Context, EventsLoop)> {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    ContextBuilder::new(title, "simpleterm")
        .window_setup(WindowSetup::default().title(title))
        .window_mode(WindowMode::default().dimensions(800.0, 600.0).resizable(true))
        .add_resource_path(resource_dir)
        .build()
}
