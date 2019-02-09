use std::io::Write;

use termion::color;
use termion::cursor;

use super::coords::Coords;
use super::Result;

pub trait SelectorPainter {
    fn draw_horizontal_selector(&mut self, coords: Coords, len: i32) -> Result;
    fn draw_vertical_selector(&mut self, coords: Coords, len: i32) -> Result;
}

impl<W> SelectorPainter for W where W: Write {
    fn draw_horizontal_selector(&mut self, coords: Coords, len: i32) -> Result {
        let (row, col) = coords.as_row_col();

        let start = cursor::Goto(col, row);

        write!(self, "{}{}", start, color::Fg(color::LightWhite))?;
        write!(self, "{}", "╘")?;

        for _ in 1..(len - 1) {
            write!(self, "{}", "═")?;
        }

        write!(self, "{}", "╛")?;

        debug!("coords: {:?}, len: {}", coords, len);
        Ok(())
    }

    fn draw_vertical_selector(&mut self, coords: Coords, len: i32) -> Result {
        let (row, col) = coords.as_row_col();

        let start = cursor::Goto(col, row);
        let next = format!("{}{}", cursor::Left(1), cursor::Down(1));

        write!(self, "{}{}", start, color::Fg(color::LightWhite))?;
        write!(self, "{}{}", "╓", next)?;

        for _ in 1..(len - 1) {
            write!(self, "{}{}", "║", next)?;
        }

        write!(self, "{}{}", "╙", next)?;

        debug!("coords: {:?}, len: {}", coords, len);
        Ok(())
    }
}
