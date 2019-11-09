//! Module to clear part of all of the display.

use std::fmt;

use log::debug;
use termion::{color, cursor};

use super::{geometry, Widget};

#[derive(Debug)]
pub struct BlankWidget {
    pub bounds: geometry::Rect<u16>,
}

impl Widget for BlankWidget {
    fn bounds(&self) -> geometry::Rect<u16> {
        self.bounds
    }
}

impl fmt::Display for BlankWidget {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let goto = geometry::goto(self.bounds.origin);
        let color = color::Fg(color::Reset);
        write!(fmt, "{}{}", goto, color)?;

        let width = self.bounds.size.width;

        let next = format!("{}{}", cursor::Down(1), cursor::Left(width));

        for _ in 0..self.bounds.size.height {
            for _ in 0..width {
                write!(fmt, " ")?;
            }

            write!(fmt, "{}", next)?;
        }

        debug!("Blanked {:?}", self.bounds);

        Ok(())
    }
}
