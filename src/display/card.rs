use std::fmt::{self, Formatter};
use std::io::Write;

use termion::color;
use termion::cursor;

use crate::display::coords::Coords;
use crate::display::Result;
use crate::game::card::{Card, Color};

pub static CARD_SIZE: Coords = Coords::from_xy(8, 4);


impl color::Color for Color {
    fn write_fg(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Color::Black => color::White.write_fg(f),
            Color::Red => color::Red.write_fg(f),
        }
    }

    fn write_bg(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Color::Black => color::White.write_bg(f),
            Color::Red => color::Red.write_bg(f),
        }
    }
}


pub trait CardPainter {
    fn draw_card(&mut self, coords: Coords, card: &Card) -> Result;
}

impl<W> CardPainter for W where W: Write {
    fn draw_card(&mut self, coords: Coords, card: &Card) -> Result {
        draw_card_frame(self, coords)?;

        let interior_coords = coords + Coords::from_xy(2, 1);
        let (row, col) = interior_coords.as_row_col();

        let start = cursor::Goto(col, row);
        let next = format!("{}{}", cursor::Left(4), cursor::Down(1));

        if card.face_up {
            let rank_str = card.rank.label();
            let suit_str = card.suit.symbol();

            let offset = cursor::Right(3 - card.rank.label().len() as u16);

            write!(self, "{}{}", start, color::Fg(card.color()))?;
            write!(self, "{}{}{}{}", rank_str, offset, suit_str, next)?;
            write!(self, "{}{}{}{}", suit_str, offset, rank_str, next)?;
        } else {
            write!(self, "{}{}", start, color::Fg(color::Blue))?;
            write!(self, "{}{}", "░░░░", next)?;
            write!(self, "{}{}", "░░░░", next)?;
        }

        Ok(())
    }
}

fn draw_card_frame<W>(writer: &mut W, coords: Coords) -> Result where W: Write {
    let (row, col) = coords.as_row_col();

    // TODO: Use CARD_SIZE?
    let start = cursor::Goto(col, row);
    let next = format!("{}{}", cursor::Left(8), cursor::Down(1));

    write!(writer, "{}{}", start, color::Fg(color::White))?;
    write!(writer, "{}{}", "╭──────╮", next)?;
    write!(writer, "{}{}", "│      │", next)?;
    write!(writer, "{}{}", "│      │", next)?;
    write!(writer, "{}{}", "╰──────╯", next)?;

    Ok(())
}
