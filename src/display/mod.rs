//! Module for things related to displaying a Klondike game in the terminal.

use snafu::IntoError;
use std::{fmt, io, num};

pub use self::{coords::Coords, game::GameDisplay};

pub mod blank;
pub mod bounds;
pub mod card;
pub mod coords;
pub mod frame;
pub mod game;
pub mod help;
pub mod selector;
pub mod stack;

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
