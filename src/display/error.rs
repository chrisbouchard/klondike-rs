use crate::error::ErrorExt;
use std::{fmt, io};

#[derive(Debug, Display)]
pub enum ErrorKind {
    #[display(fmt = "Error writing to the terminal")]
    DisplayError,
}

impl crate::error::ErrorKind for ErrorKind {}

pub type Error = crate::error::Error<ErrorKind>;
pub type Result<T> = ::std::result::Result<T, Error>;

impl From<fmt::Error> for Error {
    fn from(error: fmt::Error) -> Self {
        error.wrap_as_source_of(ErrorKind::DisplayError)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        error.wrap_as_source_of(ErrorKind::DisplayError)
    }
}
