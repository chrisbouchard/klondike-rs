use std::fmt;
use termion::cursor;

use super::bounds::Bounds;

pub trait Widget: fmt::Display {
    fn bounds(&self) -> Bounds;

    fn goto_coords(&self) -> impl fmt::Display {
        let (row, col) = self.bounds().top_left.as_row_col();
        cursor::Goto(col, row)
    }
}
