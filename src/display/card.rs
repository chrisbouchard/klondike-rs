use std::{fmt, io};
use termion::{color, cursor};

use crate::model::{Card, Color};

use super::{bounds::Bounds, coords::Coords};
use crate::error::Result;

pub static CARD_SIZE: Coords = Coords::from_xy(8, 4);

impl color::Color for Color {
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Black => color::Reset.write_fg(f),
            Color::Red => color::Red.write_fg(f),
        }
    }

    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

pub trait CardPainter {
    fn draw_card_face_up(&mut self, coords: Coords, card: &Card) -> Result<Bounds>;
    fn draw_card_face_up_slice(&mut self, coords: Coords, card: &Card) -> Result<Bounds>;
    fn draw_card_face_down(&mut self, coords: Coords) -> Result<Bounds>;
    fn draw_card_face_down_with_count(&mut self, coords: Coords, count: usize) -> Result<Bounds>;
}

impl<W> CardPainter for W
where
    W: io::Write,
{
    fn draw_card_face_up(&mut self, coords: Coords, card: &Card) -> Result<Bounds> {
        draw_card_frame(self, coords)?;

        let interior_coords = coords + Coords::from_xy(2, 1);

        let start: cursor::Goto = interior_coords.into();
        let next = format!("{}{}", cursor::Left(4), cursor::Down(1));

        let rank_str = card.rank.label();
        let suit_str = card.suit.symbol();

        let offset = cursor::Right(3 - card.rank.label().len() as u16);

        write!(self, "{}{}", start, color::Fg(card.color()))?;
        write!(self, "{}{}{}{}", rank_str, offset, suit_str, next)?;
        write!(self, "{}{}{}{}", suit_str, offset, rank_str, next)?;

        Ok(Bounds::with_size(coords, CARD_SIZE))
    }

    fn draw_card_face_up_slice(&mut self, coords: Coords, card: &Card) -> Result<Bounds> {
        draw_card_frame(self, coords)?;

        let interior_coords = coords + Coords::from_x(1);

        let start: cursor::Goto = interior_coords.into();

        let rank_str = card.rank.label();
        let suit_str = card.suit.symbol();

        let spacer = if card.rank.label().len() == 2 {
            " "
        } else {
            "╶╴"
        };

        let color = color::Fg(card.color());
        let white = color::Fg(color::Reset);

        write!(self, "{}{}", start, color::Fg(card.color()))?;
        write!(
            self,
            "{}{}╴{}{}{}{}{}{}{}╶",
            start, white, color, rank_str, white, spacer, color, suit_str, white
        )?;

        Ok(Bounds::with_size(coords, Coords::from_xy(CARD_SIZE.x, 1)))
    }

    fn draw_card_face_down(&mut self, coords: Coords) -> Result<Bounds> {
        draw_card_frame(self, coords)?;

        let interior_coords = coords + Coords::from_xy(2, 1);

        let start: cursor::Goto = interior_coords.into();
        let next = format!("{}{}", cursor::Left(4), cursor::Down(1));

        write!(self, "{}{}", start, color::Fg(color::LightBlue))?;
        write!(self, "░░░░{}", next)?;
        write!(self, "░░░░{}", next)?;

        Ok(Bounds::with_size(coords, CARD_SIZE))
    }

    fn draw_card_face_down_with_count(&mut self, coords: Coords, count: usize) -> Result<Bounds> {
        let bounds = self.draw_card_face_down(coords)?;

        let formatted_count = format!("{}×", count);

        let count_coords =
            coords + CARD_SIZE.to_x() - Coords::from_x(formatted_count.chars().count() as i32 + 3);
        let goto: cursor::Goto = count_coords.into();

        let gray = color::Fg(color::LightBlack);
        let white = color::Fg(color::Reset);

        write!(
            self,
            "{}{}╴{}{}{}╶",
            goto, white, gray, formatted_count, white
        )?;

        Ok(bounds)
    }
}

fn draw_card_frame<W>(writer: &mut W, coords: Coords) -> Result<()>
where
    W: io::Write,
{
    let (row, col) = coords.as_row_col();

    // TODO: Use CARD_SIZE?
    let start = cursor::Goto(col, row);
    let next = format!("{}{}", cursor::Left(8), cursor::Down(1));

    write!(writer, "{}{}", start, color::Fg(color::Reset))?;
    write!(writer, "╭──────╮{}", next)?;
    write!(writer, "│      │{}", next)?;
    write!(writer, "│      │{}", next)?;
    write!(writer, "╰──────╯{}", next)?;

    Ok(())
}
