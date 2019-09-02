//! Module for things related to displaying a Klondike game in the terminal.

use std::{fmt, io};
use termion::terminal_size;

use crate::utils::{
    bounds::Bounds,
    coords::{self, Coords},
};

pub mod blank;
pub mod card;
pub mod format_str;
pub mod frame;
pub mod game;
pub mod help;
pub mod selector;
pub mod stack;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum DisplayState {
    Playing,
    HelpMessageOpen,
    Quitting,
}

pub trait Widget: fmt::Display {
    fn bounds(&self) -> Bounds;
}

pub fn terminal_bounds() -> io::Result<Bounds> {
    let bottom_right: Coords = terminal_size()?.into();
    Ok(Bounds::new(coords::ZERO, bottom_right))
}
