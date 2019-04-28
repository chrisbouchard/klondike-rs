//! Module to manage a TTY in alternate mode.

use std::fmt::{self, Debug, Formatter};
use std::fs::File;
use std::io::{self, Write};

use termion::{
    self, cursor,
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Unable to open TTY.")]
    TtyError(#[cause] io::Error),
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::TtyError(error)
    }
}

pub struct Terminal {
    input: File,
    output: AlternateScreen<RawTerminal<File>>,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let input = termion::get_tty()?;
        let mut output = AlternateScreen::from(termion::get_tty()?.into_raw_mode()?);

        write!(output, "{}", cursor::Hide)?;

        Ok(Terminal { input, output })
    }

    pub fn input(&self) -> Result<File> {
        self.input.try_clone().map_err(Into::into)
    }

    pub fn output(&self) -> Result<File> {
        self.output.try_clone().map_err(Into::into)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        write!(self.output, "{}", cursor::Show).unwrap();
    }
}

impl Debug for Terminal {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("Terminal")
            .field("input", &self.input)
            .field("output", &"...")
            .finish()
    }
}
