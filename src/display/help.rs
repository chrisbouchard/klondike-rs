use std::io;
use termion::{color, cursor};

use super::{coords::Coords, Result};
use crate::utils::usize::BoundedSub;

static TOP_LEFT: Coords = Coords::from_xy(2, 2);
static INSET: Coords = Coords::from_xy(2, 2);
static BOX_SIZE: Coords = Coords::from_xy(17, 6);

pub trait HelpPainter {
    fn draw_help_message(&mut self) -> Result;
}

impl<W> HelpPainter for W
where
    W: io::Write,
{
    fn draw_help_message(&mut self) -> Result {
        let cyan = color::Fg(color::Cyan);
        let white = color::Fg(color::White);
        let reset = color::Fg(color::Reset);

        write!(
            self,
            "{goto}{white}╔{bar}╗",
            goto = cursor::Goto::from(TOP_LEFT),
            white = white,
            bar = "═".repeat((BOX_SIZE.x as usize).bounded_sub(2))
        )?;

        for i in 1..(BOX_SIZE.y - 1) {
            write!(
                self,
                "{goto}{white}║{space}║",
                goto = cursor::Goto::from(TOP_LEFT + Coords::from_y(i)),
                white = white,
                space = " ".repeat((BOX_SIZE.x as usize).bounded_sub(2))
            )?;
        }

        write!(
            self,
            "{goto}{white}╚{bar}╝",
            goto = cursor::Goto::from(TOP_LEFT + BOX_SIZE.to_y() - Coords::from_y(1)),
            white = white,
            bar = "═".repeat((BOX_SIZE.x as usize).bounded_sub(2))
        )?;

        write!(
            self,
            "{goto}{cyan}h{reset}/{cyan}j{reset}/{cyan}k{reset}/{cyan}l{reset}: {white}Move",
            goto = cursor::Goto::from(TOP_LEFT + INSET),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        write!(
            self,
            "{goto}{cyan}←{reset}/{cyan}↓{reset}/{cyan}↑{reset}/{cyan}→{reset}: {white}Move",
            goto = cursor::Goto::from(TOP_LEFT + INSET + Coords::from_y(1)),
            cyan = cyan,
            reset = reset,
            white = white
        )?;

        Ok(())
    }
}
