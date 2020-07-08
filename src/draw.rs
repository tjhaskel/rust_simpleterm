use piston_window::{*, types::{Color, FontSize}};
use std::{time::Duration, time::Instant};

use crate::{color::TermColor, FLASH_TIME, TEXT_OFFSET};

pub fn display_box(win_size: Size, bgc: Color, fgc: Color, lines: bool, context: Context, graphics: &mut G2d) {
    rectangle(fgc, [10.0, 10.0, win_size.width - 20.0, win_size.height - 20.0], context.transform, graphics);
    rectangle(bgc, [15.0, 15.0, win_size.width - 30.0, win_size.height - 30.0], context.transform, graphics);

    if lines {
        let line_color: Color = if fgc.brighter_than(bgc) {
            [bgc[0] - 0.2, bgc[1] - 0.2, bgc[2] - 0.2, 0.5]
        } else {
            [bgc[0] + 0.15, bgc[1] + 0.15, bgc[2] + 0.15, 0.4]
        };
        
        for i in 0..((win_size.height - 30.0) as i32 / 3) {
            rectangle(line_color, [15.0, (i * 3) as f64 + 15.0, win_size.width - 30.0, 0.5], context.transform, graphics);
        }
    }
}

pub fn display_message(message: &Vec<String>, glyphs: &mut Glyphs, font_size: FontSize, fgc: Color, context: Context, graphics: &mut G2d)  {
    let x = TEXT_OFFSET.0;
    let y = TEXT_OFFSET.1;

    let mut y_offset: f64 = 0.0;
    for line in message.iter() {
        text::Text::new_color(fgc, font_size).draw(
            line,
            glyphs,
            &context.draw_state,
            context.transform.trans(x, y + y_offset),
            graphics,
        ).unwrap();

        y_offset += (font_size as f64) * 0.8;
    }
}

pub fn display_input_marker(win_size: Size, glyphs: &mut Glyphs, font_size: FontSize, fgc: Color, context: Context, graphics: &mut G2d) {
    let x = TEXT_OFFSET.0;
    let y = (win_size.height - TEXT_OFFSET.1) + 20.0;

    text::Text::new_color(fgc, font_size).draw(
        "> ",
        glyphs,
        &context.draw_state,
        context.transform.trans(x, y),
        graphics,
    ).unwrap();
}

pub fn display_input(win_size: Size, message: &str, glyphs: &mut Glyphs, font_size: FontSize, fgc: Color, context: Context, graphics: &mut G2d)  {
    let x = TEXT_OFFSET.0 + 20.0;
    let y = (win_size.height - TEXT_OFFSET.1) + 20.0;

    text::Text::new_color(fgc, font_size).draw(
        message,
        glyphs,
        &context.draw_state,
        context.transform.trans(x, y),
        graphics,
    ).unwrap();
}

pub fn display_filter(win_size: Size, bgc: Color, lines: bool, context: Context, graphics: &mut G2d) {
    if lines {
        let line_color: Color = [bgc[0], bgc[1], bgc[2], 0.4];
        
        for i in 0..((win_size.height - 30.0) as i32 / 3) {
            rectangle(line_color, [15.0, (i * 3) as f64 + 15.0, win_size.width - 30.0, 0.5], context.transform, graphics);
        }
    }

    rectangle(bgc, [0.0, 0.0, win_size.width, 10.0], context.transform, graphics);
    rectangle(bgc, [0.0, 0.0, 10.0, win_size.height], context.transform, graphics);
    rectangle(bgc, [win_size.width - 10.0, 0.0, 10.0, win_size.height], context.transform, graphics);
    rectangle(bgc, [0.0, win_size.height - 10.0, win_size.width, 10.0], context.transform, graphics);
}

pub fn check_flash(now: Instant, then: &mut Instant) -> bool {
    let time_since: Duration = now.duration_since(*then);
    if time_since > (FLASH_TIME * 2) {
        *then = now;
        true
    } else if time_since > FLASH_TIME { 
        true
    } else {
        false
    }
}
