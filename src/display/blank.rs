use std::io::Write;

use termion::{color, cursor, cursor::DetectCursorPos};

use super::{bounds::Bounds, coords::Coords, Result};

pub trait BlankPainter {
    fn draw_blank_excess(&mut self, old_bounds: Bounds, new_bounds: Bounds) -> Result;
}

impl<W> BlankPainter for W
where
    W: Write + DetectCursorPos,
{
    fn draw_blank_excess(&mut self, old_bounds: Bounds, new_bounds: Bounds) -> Result {
        let color = color::Fg(color::Reset);
        write!(self, "{}", color)?;

        for coords in old_bounds.coords_iter() {
            if !new_bounds.contains(coords) {
                let cursor_coords: Coords = self.cursor_pos()?.into();

                if coords != cursor_coords {
                    let (row, col) = coords.as_row_col();
                    write!(self, "{}", cursor::Goto(row, col))?;
                }

                write!(self, " ")?;
            }
        }

        Ok(())
    }
}
