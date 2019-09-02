//! Module for things related to displaying a Klondike game in the terminal.

use snafu::IntoError;
use std::{fmt, io, num};
use termion::terminal_size;

mod blank;
mod bounds;
mod card;
mod coords;
mod format_str;
mod frame;
mod game;
mod help;
mod selector;
mod stack;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Error writing to the terminal: {}", source))]
    DisplayFmtError { source: fmt::Error },

    #[snafu(display("Error writing to the terminal: {}", source))]
    DisplayIoError { source: io::Error },

    #[snafu(display("Error converting value to int: {}", source))]
    NumberError { source: num::TryFromIntError },
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

impl From<fmt::Error> for Error {
    fn from(error: fmt::Error) -> Self {
        DisplayFmtError.into_error(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        DisplayIoError.into_error(error)
    }
}

impl From<num::TryFromIntError> for Error {
    fn from(error: num::TryFromIntError) -> Self {
        NumberError.into_error(error)
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum DisplayState {
    Playing,
    HelpMessageOpen,
    Quitting,
}

pub trait Widget: fmt::Display {
    fn bounds(&self) -> Bounds;
}

pub fn terminal_bounds() -> Result<Bounds> {
    let size: Coords = terminal_size()?.into();
    Ok(Bounds::with_size_from_zero(size))
}
