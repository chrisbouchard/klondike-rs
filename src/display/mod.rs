use std::{fmt, io};

pub use self::{coords::Coords, game::GameDisplay};

pub mod blank;
pub mod bounds;
pub mod card;
pub mod coords;
pub mod game;
pub mod help;
pub mod selector;
pub mod stack;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Error writing to Write instance.")]
    FmtError(#[cause] fmt::Error),

    #[fail(display = "Error writing to Write instance.")]
    IoError(#[cause] io::Error),
}

pub type Result<T = ()> = ::std::result::Result<T, Error>;

impl From<fmt::Error> for Error {
    fn from(error: fmt::Error) -> Self {
        Error::FmtError(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum DisplayState {
    Playing,
    HelpMessageOpen,
    Quitting,
}
