use std::fmt;
use termion::{color, cursor};

use crate::{
    model::{Card, Color},
    utils::format_str::FormattedString,
};

use super::{
    blank::BlankWidget,
    frame::{FrameStyle, FrameWidget, Title},
    geometry, Widget,
};

lazy_static! {
    pub static ref CARD_SIZE: geometry::Size2D<u16> = geometry::size2(8, 4);
    pub static ref SLICE_SIZE: geometry::Size2D<u16> = geometry::size2(8, 2);
}

pub static CARD_FRAME_STYLE: FrameStyle = FrameStyle {
    top_left: "╭",
    top: "─",
    top_right: "╮",
    left: "│",
    right: "│",
    bottom_left: "╰",
    bottom: "─",
    bottom_right: "╯",
    title_left: "╴",
    title_right: "╶",
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

#[derive(Debug)]
pub struct CardWidget<'a> {
    pub card: &'a Card,
    pub origin: geometry::Point2D<u16>,
    pub mode: CardWidgetMode,
}

impl<'a> CardWidget<'a> {
    fn fmt_frame(&self, title: Option<FormattedString>, fmt: &mut fmt::Formatter) -> fmt::Result {
        let bounds = self.bounds();

        if bounds.size.height > 2 && bounds.size.width > 2 {
            let blank = BlankWidget {
                bounds: bounds.inner_rect(geometry::SideOffsets2D::new_all_same(1)),
            };
            write!(fmt, "{}", blank)?;
        }

        let frame = FrameWidget {
            bounds,
            top_title: title.map(|title| Title::right(title)),
            bottom_title: None,
            frame_style: &CARD_FRAME_STYLE,
        };

        write!(fmt, "{}", frame)?;
        Ok(())
    }
}

impl<'a> Widget for CardWidget<'a> {
    fn bounds(&self) -> geometry::Rect<u16> {
        geometry::Rect::new(self.origin, *CARD_SIZE)
    }
}

impl<'a> fmt::Display for CardWidget<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.mode {
            CardWidgetMode::FullFaceUp => {
                let interior_coords = self.origin + geometry::vec2(2, 1);

                let color = color::Fg(self.card.color());
                let start = geometry::goto(interior_coords);
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
                let interior_coords = self.origin + geometry::vec2(2, 1);

                let start = geometry::goto(interior_coords);
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

                let title = FormattedString::new_with_formatting(color)
                    .push_content(rank_str)
                    .push_formatting(white)
                    .push_content(spacer)
                    .push_formatting(color)
                    .push_content(suit_str);

                self.fmt_frame(Some(title), fmt)?;
            }

            CardWidgetMode::SliceFaceDown(count) => {
                let gray = color::Fg(color::LightBlack);

                let formatted_count = format!("{}×", count);

                let title =
                    FormattedString::new_with_formatting(gray).push_content(formatted_count);

                self.fmt_frame(Some(title), fmt)?;
            }
        }

        Ok(())
    }
}
