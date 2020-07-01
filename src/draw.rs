use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawMode, DrawParam, Mesh, Rect, Text, TextFragment};

use crate::{terminal::{Terminal, TermState}, TEXT_OFFSET};

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

    match term.state {
        TermState::Typing => {
            let mut new_text: Text = text.clone();
            for (i, frag) in new_text.fragments_mut().iter_mut().enumerate() {
                if i > term.counter as usize { *frag = TextFragment::default(); }
            }
            graphics::queue_text(ctx, &new_text, TEXT_OFFSET, None);
        },
        _ => {
            graphics::queue_text(ctx, text, TEXT_OFFSET, None);
        }
    }

    graphics::draw_queued_text(ctx, DrawParam::default(), None, graphics::FilterMode::Linear)
}
