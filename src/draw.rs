use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawMode, DrawParam, Mesh, Rect};

use crate::{terminal::Terminal, TEXT_OFFSET};

pub fn draw_background(term: &mut Terminal, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, term.bg_color);

    let screen: Rect = graphics::screen_coordinates(ctx);
    let text_box_bounds: Rect = Rect::new(
        TEXT_OFFSET.x - 15.0,
        TEXT_OFFSET.y - 10.0,
        screen.w - ((TEXT_OFFSET.x * 2.0) - 30.0),
        screen.h - ((TEXT_OFFSET.y * 2.0) - 20.0)
    );
    let text_box: Mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(10.0), text_box_bounds, term.fg_color)?;
    graphics::draw(ctx, &text_box, DrawParam::default())
}

pub fn draw_text(term: &mut Terminal, ctx: &mut Context) -> GameResult {
    let text = &term.message;
    graphics::queue_text(ctx, text, TEXT_OFFSET, None);
    graphics::draw_queued_text(ctx, DrawParam::default(), None, graphics::FilterMode::Linear)
}
