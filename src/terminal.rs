use cgmath::Point2;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{event, timer};
use ggez::graphics::{self, Align, Color, Font, Scale, Text, TextFragment};
use ggez::{Context, ContextBuilder, event::EventsLoop, GameResult};
use std::{env, f32, path};

use crate::{draw::{draw_background, draw_text}, TEXT_OFFSET};

pub struct Terminal {
    pub message: Text,
    pub input: Text,
    font: Font,
    font_size: Scale,
    pub bg_color: Color,
    pub fg_color: Color,
    pub scan_lines: bool,
    pub state: TermState,
    pub counter: u32,
    pub timer: f64,
}

pub enum TermState {
    Continue,
    Typing,
    WaitContinue,
    WaitTimer,
    WaitInput,
}

impl Terminal {
    pub fn new(ctx: &mut Context, font_file: &str, font_size: f32, bgc: Color, fcg: Color) -> GameResult<Terminal> {
        Ok( Terminal {
            message: Text::default(),
            input: Text::default(),
            font: Font::new(ctx, font_file)?,
            font_size: Scale::uniform(font_size),
            bg_color: bgc,
            fg_color: fcg,
            scan_lines: true,
            state: TermState::Continue,
            counter: 0,
            timer: 0.0,
        })
    }

    pub fn tell(&mut self, ctx: &mut Context, events: &mut EventsLoop, text: &str) -> GameResult {
        self.message = Text::new( TextFragment {
            text: text.to_string(),
            color: Some(self.fg_color),
            font: Some(self.font),
            scale: Some(self.font_size),
        });

        self.state = TermState::Typing;
        self.counter = 0;
        self.timer = 0.0;

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
        draw_background(self, ctx)?;
        draw_text(self, ctx)?;
        
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        self.message.set_bounds(Point2::new(width - (TEXT_OFFSET.x * 2.0), (height * 0.8) - (TEXT_OFFSET.y * 2.0)), Align::Left);
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height)).unwrap();
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
