//! Module to clear part of all of the display.

use std::fmt;

use termion::{color, cursor};

use super::{bounds::Bounds, coords::Coords, widget::Widget};

pub struct BlankWidget {
    pub bounds: Bounds,
}

impl Widget for BlankWidget {
    fn bounds(&self) -> Bounds {
        self.bounds
    }
}

impl fmt::Display for BlankWidget {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let color = color::Fg(color::Reset);
        write!(fmt, "{}", color)?;

        let width = self.bounds.width();
        let blank_line = " ".repeat(width as usize);

        for y in 0..self.bounds.height() {
            let line_coords = self.bounds.top_left + Coords::from_y(y);
            let goto: cursor::Goto = line_coords.into();
            write!(fmt, "{}{}", goto, blank_line)?;
        }

        debug!("Blanked {:?}", self.bounds);

        Ok(())
    }
}
