use std::io::Write;

use termion::color;
use termion::cursor;

use crate::display::*;

pub trait SelectorPainter {
    fn draw_horizontal_selector(&mut self, coords: Coords, len: i32);
    fn draw_vertical_selector(&mut self, coords: Coords, len: i32);
}

impl<W> SelectorPainter for W where W: Write {
    fn draw_horizontal_selector(&mut self, coords: Coords, len: i32) {
        let (row, col) = coords.as_row_col();

        let start = cursor::Goto(row, col);

        write!(self, "{}{}", start, color::Fg(color::LightWhite));
        write!(self, "{}", "╘");

        for i in 1..(len - 1) {
            write!(self, "{}", "═");
        }

        write!(self, "{}", "╛");

        debug!("coords: {:?}, len: {}", coords, len);
    }

    fn draw_vertical_selector(&mut self, coords: Coords, len: i32) {
        let (row, col) = coords.as_row_col();

        let start = cursor::Goto(row, col);
        let next = format!("{}{}", cursor::Left(1), cursor::Down(1));

        write!(self, "{}{}", start, color::Fg(color::LightWhite));
        write!(self, "{}{}", "╓", next);

        for i in 1..(len - 1) {
            write!(self, "{}{}", "║", next);
        }

        write!(self, "{}{}", "╙", next);

        debug!("coords: {:?}, len: {}", coords, len);
    }
}
