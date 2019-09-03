use std::fmt;
use termion::{color, cursor};

use crate::{
    model::stack::Orientation,
    utils::{bounds::Bounds, coords::Coords},
};

use super::Widget;

mod horizontal {
    use super::{fmt, Bounds, Coords};

    pub fn write_start(fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "╘")
    }

    pub fn write_middle(fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "═")
    }

    pub fn write_end(fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "╛")
    }

    pub fn write_next(_fmt: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }

    pub fn bounds(coords: Coords, len: u16) -> Bounds {
        Bounds::with_size(coords, Coords::from_xy(i32::from(len), 1))
    }
}

mod vertical {
    use super::{cursor, fmt, Bounds, Coords};

    pub fn write_start(fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "╓╴")
    }

    pub fn write_middle(fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "║ ")
    }

    pub fn write_end(fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "╙╴")
    }

    pub fn write_next(fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}{}", cursor::Left(2), cursor::Down(1))
    }

    pub fn bounds(coords: Coords, len: u16) -> Bounds {
        Bounds::with_size(coords, Coords::from_xy(2, i32::from(len)))
    }
}

#[derive(Debug)]
pub struct SelectorWidget {
    pub coords: Coords,
    pub len: u16,
    pub orientation: Orientation,
}

impl Widget for SelectorWidget {
    fn bounds(&self) -> Bounds {
        match self.orientation {
            Orientation::Horizontal => horizontal::bounds(self.coords, self.len),
            Orientation::Vertical => vertical::bounds(self.coords, self.len),
        }
    }
}

impl fmt::Display for SelectorWidget {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let start: cursor::Goto = self.coords.into();
        let color = color::Fg(color::LightWhite);

        write!(fmt, "{}{}", start, color)?;

        for i in 0..self.len {
            if i == 0 {
                self.write_start(fmt)?;
            } else if i == self.len - 1 {
                self.write_end(fmt)?;
            } else {
                self.write_middle(fmt)?;
            }

            self.write_next(fmt)?;
        }

        Ok(())
    }
}

impl SelectorWidget {
    fn write_start(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.orientation {
            Orientation::Horizontal => horizontal::write_start(fmt),
            Orientation::Vertical => vertical::write_start(fmt),
        }
    }

    fn write_middle(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.orientation {
            Orientation::Horizontal => horizontal::write_middle(fmt),
            Orientation::Vertical => vertical::write_middle(fmt),
        }
    }

    fn write_end(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.orientation {
            Orientation::Horizontal => horizontal::write_end(fmt),
            Orientation::Vertical => vertical::write_end(fmt),
        }
    }

    fn write_next(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.orientation {
            Orientation::Horizontal => horizontal::write_next(fmt),
            Orientation::Vertical => vertical::write_next(fmt),
        }
    }
}
