use std::io;
use termion::{color, cursor, terminal_size};

use super::{bounds::Bounds, coords::Coords, Result};
use crate::utils::usize::BoundedSub;

static MARGIN: Coords = Coords::from_xy(2, 1);
static BORDER: Coords = Coords::from_xy(1, 1);
static PADDING: Coords = Coords::from_xy(2, 1);

pub trait HelpPainter {
    fn draw_help_message(&mut self) -> Result;
}

impl<W> HelpPainter for W
where
    W: io::Write,
{
    fn draw_help_message(&mut self) -> Result {
        let top_left = MARGIN;
        let bottom_right = Coords::from(terminal_size()?) - MARGIN;
        let bounds = Bounds::new(top_left, bottom_right);

        let cyan = color::Fg(color::Cyan);
        let white = color::Fg(color::White);
        let reset = color::Fg(color::Reset);

        write!(
            self,
            "{goto}{white}╔{title:═^width$}╗",
            goto = cursor::Goto::from(top_left),
            white = white,
            title = "╡ H E L P ╞",
            width = (bounds.width() as usize).bounded_sub(2)
        )?;

        for i in 1..(bounds.height() - 1) {
            write!(
                self,
                "{goto}{white}║{skip}║",
                goto = cursor::Goto::from(top_left + Coords::from_y(i)),
                white = white,
                skip = cursor::Right((bounds.width() as usize).bounded_sub(2) as u16)
            )?;
        }

        write!(
            self,
            "{goto}{white}╚{empty:═^width$}╝",
            goto = cursor::Goto::from(top_left.to_x() + bottom_right.to_y()),
            white = white,
            empty = "",
            width = (bounds.width() as usize).bounded_sub(2),
        )?;

        let inner_top_left = top_left + BORDER + PADDING;
        let inner_bottom_right = bottom_right - BORDER - PADDING;

        write!(
            self,
            "{goto}{cyan}h{reset} / {cyan}j{reset} / {cyan}k{reset} / {cyan}l{reset} :  {white}Move",
            goto = cursor::Goto::from(inner_top_left),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            self,
            "{goto}{cyan}←{reset} / {cyan}↓{reset} / {cyan}↑{reset} / {cyan}→{reset} :  {white}Move",
            goto = cursor::Goto::from(inner_top_left + Coords::from_y(1)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            self,
            "{goto}{cyan}s {reset}:  {white}Move to Stock (Deck)",
            goto = cursor::Goto::from(inner_top_left + Coords::from_y(3)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            self,
            "{goto}{cyan}t {reset}:  {white}Move to Talon (Waste)",
            goto = cursor::Goto::from(inner_top_left + Coords::from_y(4)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            self,
            "{goto}{reset}Press any key to continue . . .",
            goto = cursor::Goto::from(inner_top_left.to_x() + inner_bottom_right.to_y()),
            reset = reset,
        )?;

        Ok(())
    }
}
