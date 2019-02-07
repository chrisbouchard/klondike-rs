use std::fmt;
use std::io;

pub use self::coords::Coords;
pub use self::game::GamePainter;

pub mod card;
pub mod coords;
pub mod game;
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
