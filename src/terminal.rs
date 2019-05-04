//! Module to manage a TTY in alternate mode.

use std::{fmt, fs::File, io::Write};

use termion::{
    self, cursor,
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};

use super::error::Result;

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

impl fmt::Debug for Terminal {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Terminal")
            .field("input", &self.input)
            .field("output", &"...")
            .finish()
    }
}
