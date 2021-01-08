use nannou::color::{encoding::Srgb, rgb::Rgb};
use nannou::prelude::*;

/// Represents a drawable axis.
pub struct Axis {
    color: Rgb<Srgb, u8>,
    text: bool,
    textcolor: Option<Rgb<Srgb, u8>>,
    textsize: Option<u32>,
    padding: f32,
}

impl Default for Axis {
    fn default() -> Self {
        Axis {
            color: BLACK,
            text: true,
            textcolor: Some(BLACK),
            textsize: Some(14),
            padding: 10.,
        }
    }
}

impl Axis {
    pub fn draw(&self, draw: &Draw, win: &Rect) {
        let padded_win = win.pad(self.padding);

        let x_offset = 3. * self.padding;
        let y_offset = 2. * self.padding;

        let ends = [
            win.mid_top(),
            win.mid_right(),
            win.mid_bottom(),
            win.mid_left(),
        ];

        for &end in &ends {
            draw.arrow()
                .start_cap_round()
                .head_length(10.0)
                .head_width(4.0)
                .color(self.color)
                .end(end);
        }

        if self.text {
            let top = win.top().round().to_i32().unwrap().to_string();
            let bottom = win.bottom().round().to_i32().unwrap().to_string();
            let left = win.top().round().to_i32().unwrap().to_string();
            let right = win.top().round().to_i32().unwrap().to_string();

            let text = vec![
                draw.text("0.0").x_y(x_offset, y_offset),
                draw.text(&top)
                    .x(x_offset)
                    .align_text_top()
                    .h(padded_win.h()),
                draw.text(&bottom)
                    .x(x_offset)
                    .align_text_bottom()
                    .h(padded_win.h()),
                draw.text(&left)
                    .y(y_offset)
                    .left_justify()
                    .w(padded_win.w()),
                draw.text(&right)
                    .y(y_offset)
                    .right_justify()
                    .w(padded_win.w()),
            ];

            for t in text {
                t.font_size(match self.textsize {
                    Some(s) => s,
                    None => 14,
                })
                .color(match self.textcolor {
                    Some(c) => c,
                    None => self.color,
                });
            }
        }
    }
}

pub struct Xticks {
    increment: f32,
    y_pos: f32,
    ticksize: f32,
    tickwidth: f32,
    color: Rgb<Srgb, u8>,
}

impl Xticks {
    pub fn draw(&self, draw: &Draw, win: &Rect) {
        let step_by = || (0..).map(|i| i as f32 * self.increment);
        let r_iter = step_by().take_while(|&f| f < win.right());
        let l_iter = step_by().map(|f| -f).take_while(|&f| f > win.left());
        let x_iter = r_iter.chain(l_iter);
        for x in x_iter {
            draw.line()
                .weight(self.tickwidth)
                .points(
                    pt2(x, self.y_pos - self.ticksize / 2.),
                    pt2(x, self.y_pos + self.ticksize / 2.),
                )
                .color(self.color);
        }
    }
}

impl Default for Xticks {
    fn default() -> Self {
        Xticks {
            increment: 10.,
            y_pos: 0.,
            ticksize: 5.,
            tickwidth: 1.,
            color: BLACK,
        }
    }
}

pub struct Yticks {
    increment: f32,
    x_pos: f32,
    ticksize: f32,
    tickwidth: f32,
    color: Rgb<Srgb, u8>,
}

impl Yticks {
    pub fn draw(&self, draw: &Draw, win: &Rect) {
        let step_by = || (0..).map(|i| i as f32 * self.increment);

        let t_iter = step_by().take_while(|&f| f < win.top());
        let b_iter = step_by().map(|f| -f).take_while(|&f| f > win.bottom());
        let y_iter = t_iter.chain(b_iter);
        for y in y_iter {
            draw.line()
                .weight(self.tickwidth)
                .points(
                    pt2(self.x_pos - self.ticksize / 2., y),
                    pt2(self.x_pos + self.ticksize / 2., y),
                )
                .color(self.color);
        }
    }
}

impl Default for Yticks {
    fn default() -> Self {
        Yticks {
            increment: 10.,
            x_pos: 0.,
            ticksize: 5.,
            tickwidth: 1.,
            color: BLACK,
        }
    }
}
