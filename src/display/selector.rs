use std::io;
use termion::{color, cursor};

use super::{bounds::Bounds, coords::Coords, error::Result};

pub trait SelectorPainter {
    fn draw_horizontal_selector(&mut self, coords: Coords, len: u16, held: bool) -> Result<Bounds>;
    fn draw_vertical_selector(&mut self, coords: Coords, len: u16, held: bool) -> Result<Bounds>;
}

impl<W> SelectorPainter for W
where
    W: io::Write,
{
    fn draw_horizontal_selector(&mut self, coords: Coords, len: u16, held: bool) -> Result<Bounds> {
        let start: cursor::Goto = coords.into();
        let color = color::Fg(color::LightWhite);

        write!(self, "{}{}", start, color)?;

        for i in 0..len {
            if i == 0 {
                write!(self, "╘")?;
            } else if i == len - 1 {
                write!(self, "╛")?;
            } else {
                write!(self, "═")?;
            }
        }

        debug!("coords: {:?}, len: {}", coords, len);
        Ok(Bounds::with_size(
            coords,
            Coords::from_xy(i32::from(len), 1),
        ))
    }

    fn draw_vertical_selector(&mut self, coords: Coords, len: u16, held: bool) -> Result<Bounds> {
        let start: cursor::Goto = coords.into();
        let next = format!("{}{}", cursor::Left(2), cursor::Down(1));
        let color = color::Fg(color::LightWhite);

        write!(self, "{}{}", start, color)?;

        for i in 0..len {
            if i == 0 {
                write!(self, "╓╴")?;
            } else if i == len - 1 {
                write!(self, "╙╴")?;
            } else {
                write!(self, "║ ")?;
            }

            write!(self, "{}", next)?;
        }

        debug!("coords: {:?}, len: {}", coords, len);
        Ok(Bounds::with_size(
            coords,
            Coords::from_xy(2, i32::from(len)),
        ))
    }
}
