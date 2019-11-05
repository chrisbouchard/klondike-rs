//! Module for things related to displaying a Klondike game in the terminal.

use std::{fmt, io};
use termion::terminal_size;

pub mod blank;
pub mod card;
pub mod frame;
pub mod game;
pub mod geometry;
pub mod help;
pub mod selector;
pub mod stack;
pub mod win;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum DisplayState {
    Playing,
    HelpMessageOpen,
    Quitting,
    WinMessageOpen,
}

pub trait Widget: fmt::Display {
    fn bounds(&self) -> geometry::Rect<u16>;
}

pub fn terminal_bounds() -> io::Result<geometry::Size2D<u16>> {
    let (cols, rows) = terminal_size()?;
    Ok(geometry::size2(cols, rows))
}
