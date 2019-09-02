use std::fmt;
use termion::{color, cursor};

use crate::model::{Card, Color};

use super::{
    bounds::Bounds,
    coords::Coords,
    format_str::FormattedString,
    frame::{Direction, FrameStyle, FrameWidget, Title},
    Widget,
};

pub static CARD_SIZE: Coords = Coords::from_xy(8, 4);
pub static SLICE_SIZE: Coords = Coords::from_xy(8, 2);

pub static CARD_FRAME_STYLE: FrameStyle = FrameStyle {
    top_left: '╭',
    top: '─',
    top_right: '╮',
    left: '│',
    right: '│',
    bottom_left: '╰',
    bottom: '─',
    bottom_right: '╯',
    title_left: '╴',
    title_right: '╶',
};

impl color::Color for Color {
    fn write_fg(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Black => color::Reset.write_fg(fmt),
            Color::Red => color::Red.write_fg(fmt),
        }
    }

    fn write_bg(&self, _fmt: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CardWidgetMode {
    FullFaceUp,
    FullFaceDown,
    SliceFaceUp,
    SliceFaceDown(usize),
}

impl CardWidgetMode {
    fn size(&self) -> Coords {
        match self {
            CardWidgetMode::FullFaceUp | CardWidgetMode::FullFaceDown => CARD_SIZE,
            CardWidgetMode::SliceFaceUp | CardWidgetMode::SliceFaceDown(_) => SLICE_SIZE,
        }
    }
}

#[derive(Debug)]
pub struct CardWidget<'a> {
    pub card: &'a Card,
    pub coords: Coords,
    pub mode: CardWidgetMode,
}

impl<'a> CardWidget<'a> {
    fn fmt_frame(&self, title: Option<FormattedString>, fmt: &mut fmt::Formatter) -> fmt::Result {
        let frame = FrameWidget {
            bounds: self.bounds(),
            top_title: title.map(|title| Title(title, Direction::Right)),
            bottom_title: None,
            frame_style: &CARD_FRAME_STYLE,
        };

        write!(fmt, "{}", frame)?;
        Ok(())
    }
}

impl<'a> Widget for CardWidget<'a> {
    fn bounds(&self) -> Bounds {
        Bounds::with_size(self.coords, self.mode.size())
    }
}

impl<'a> fmt::Display for CardWidget<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.mode {
            CardWidgetMode::FullFaceUp => {
                let interior_coords = self.coords + Coords::from_xy(2, 1);

                let color = color::Fg(self.card.color());
                let start: cursor::Goto = interior_coords.into();
                let next = format!("{}{}", cursor::Left(4), cursor::Down(1));

                let rank_str = format!("{}", self.card.rank);
                let suit_str = format!("{}", self.card.suit);

                let offset = cursor::Right(3 - rank_str.len() as u16);

                self.fmt_frame(None, fmt)?;
                write!(fmt, "{}{}", start, color)?;
                write!(fmt, "{}{}{}{}", rank_str, offset, suit_str, next)?;
                write!(fmt, "{}{}{}{}", suit_str, offset, rank_str, next)?;
            }

            CardWidgetMode::FullFaceDown => {
                let interior_coords = self.coords + Coords::from_xy(2, 1);

                let start: cursor::Goto = interior_coords.into();
                let next = format!("{}{}", cursor::Left(4), cursor::Down(1));

                self.fmt_frame(None, fmt)?;
                write!(fmt, "{}{}", start, color::Fg(color::LightBlue))?;
                write!(fmt, "░░░░{}", next)?;
                write!(fmt, "░░░░{}", next)?;
            }

            CardWidgetMode::SliceFaceUp => {
                let color = color::Fg(self.card.color());
                let white = color::Fg(color::White);

                let rank_str = format!("{}", self.card.rank);
                let suit_str = format!("{}", self.card.suit);

                let spacer = if rank_str.len() == 2 { " " } else { "╶╴" };

                let title = FormattedString::new()
                    .push_formatting(color)
                    .push_content(rank_str)
                    .push_formatting(white)
                    .push_content(spacer)
                    .push_formatting(color)
                    .push_formatting(suit_str);

                self.fmt_frame(Some(title), fmt)?;
            }

            CardWidgetMode::SliceFaceDown(count) => {
                let gray = color::Fg(color::LightBlack);

                let formatted_count = format!("{}×", count);

                let title = FormattedString::new()
                    .push_formatting(gray)
                    .push_content(formatted_count);

                self.fmt_frame(Some(title), fmt)?;
            }
        }

        Ok(())
    }
}
