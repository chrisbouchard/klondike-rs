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
        let inner_bounds = Bounds::new(inner_top_left, inner_bottom_right);

        let inner_top_middle = inner_top_left + Coords::from_x(inner_bounds.width() / 2);

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
            "{goto}{cyan}s{reset} :  {white}Move to Stock/Deck",
            goto = cursor::Goto::from(inner_top_left + Coords::from_y(3)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            self,
            "{goto}{cyan}t{reset} :  {white}Move to Talon/Waste",
            goto = cursor::Goto::from(inner_top_left + Coords::from_y(4)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            self,
            "{goto}{cyan}f{reset} :  {white}Move to Next Foundation",
            goto = cursor::Goto::from(inner_top_left + Coords::from_y(5)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            self,
            "{goto}{cyan}-{reset} :  {white}Move to Previous",
            goto = cursor::Goto::from(inner_top_left + Coords::from_y(6)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            self,
            "{goto}{cyan}F1{reset} ... {cyan}F4{reset} :  {white}Move to Foundation",
            goto = cursor::Goto::from(inner_top_middle),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            self,
            "{goto}{cyan}1{reset} ... {cyan}7{reset} :  {white}Move to Tableaux",
            goto = cursor::Goto::from(inner_top_middle + Coords::from_y(1)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            self,
            "{goto}{cyan}SPACE{reset} :  {white}Pick Up/Activate",
            goto = cursor::Goto::from(inner_top_middle + Coords::from_y(3)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            self,
            "{goto}{cyan}?{reset} :  {white}Display Help",
            goto = cursor::Goto::from(inner_top_middle + Coords::from_y(5)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            self,
            "{goto}{cyan}q{reset} :  {white}Quit",
            goto = cursor::Goto::from(inner_top_middle + Coords::from_y(6)),
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
