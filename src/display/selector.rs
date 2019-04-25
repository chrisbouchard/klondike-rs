use std::io::Write;

use termion::{color, cursor};

use super::{bounds::Bounds, coords::Coords, Result};

pub trait SelectorPainter {
    fn draw_horizontal_selector(
        &mut self,
        full_coords: Coords,
        full_len: u16,
        selected_index: u16,
        selected_len: u16,
    ) -> Result<Bounds>;

    fn draw_vertical_selector(
        &mut self,
        full_coords: Coords,
        full_len: u16,
        selected_index: u16,
        selected_len: u16,
    ) -> Result<Bounds>;
}

impl<W> SelectorPainter for W
where
    W: Write,
{
    fn draw_horizontal_selector(
        &mut self,
        full_coords: Coords,
        full_len: u16,
        selected_index: u16,
        selected_len: u16,
    ) -> Result<Bounds> {
        let (row, col) = full_coords.as_row_col();

        let selected_end_index = selected_index + selected_len;

        let start = cursor::Goto(col, row);
        let color = color::Fg(color::LightWhite);

        write!(self, "{}{}", start, color)?;

        for i in 0..full_len {
            if i == selected_index {
                write!(self, "╘")?;
            } else if i > selected_index && i < selected_end_index {
                write!(self, "═")?;
            } else if i == selected_end_index {
                write!(self, "╛")?;
            } else {
                write!(self, " ")?;
            }
        }

        debug!(
            "full_coords: {:?}, full_len: {}, selected_index: {}, selected_len: {}",
            full_coords, full_len, selected_index, selected_len
        );
        Ok(Bounds::with_size(
            full_coords,
            Coords::from_xy(full_len as i32, 1),
        ))
    }

    fn draw_vertical_selector(
        &mut self,
        full_coords: Coords,
        full_len: u16,
        selected_index: u16,
        selected_len: u16,
    ) -> Result<Bounds> {
        let (row, col) = full_coords.as_row_col();

        let selected_end_index = selected_index + selected_len;

        let start = cursor::Goto(col, row);
        let color = color::Fg(color::LightWhite);
        let next = format!("{}{}", cursor::Left(1), cursor::Down(1));

        write!(self, "{}{}", start, color)?;

        for i in 0..full_len {
            if i == selected_index {
                write!(self, "╓")?;
            } else if i > selected_index && i < selected_end_index {
                write!(self, "║")?;
            } else if i == selected_end_index {
                write!(self, "╙")?;
            } else {
                write!(self, " ")?;
            }

            write!(self, "{}", next)?;
        }

        debug!(
            "full_coords: {:?}, full_len: {}, selected_index: {}, selected_len: {}",
            full_coords, full_len, selected_index, selected_len
        );
        Ok(Bounds::with_size(
            full_coords,
            Coords::from_xy(1, full_len as i32),
        ))
    }
}
