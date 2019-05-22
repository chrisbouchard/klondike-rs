use snafu::IntoError;
use std::{fmt, io};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Error writing to the terminal: {}", source))]
    DisplayFmtError { source: fmt::Error },

    #[snafu(display("Error writing to the terminal: {}", source))]
    DisplayIoError { source: io::Error },
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
