//! Module to manage a TTY in alternate mode.

use snafu::ResultExt;
use std::{
    fmt, fs,
    io::{self, Write},
};

use termion::{
    self, cursor,
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Unable switch output to raw mode: {}", source))]
    RawModeError { source: io::Error },

    #[snafu(display("Unable to get TTY: {}", source))]
    TtyError { source: io::Error },

    #[snafu(display("Unable write to output: {}", source))]
    WriteError { source: io::Error },
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

pub struct Terminal {
    input: fs::File,
    output: AlternateScreen<RawTerminal<fs::File>>,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let input = termion::get_tty().context(TtyError)?;
        let mut output = AlternateScreen::from(
            termion::get_tty()
                .context(TtyError)?
                .into_raw_mode()
                .context(RawModeError)?,
        );

        write!(output, "{}", cursor::Hide).context(WriteError)?;

        Ok(Terminal { input, output })
    }

    pub fn input(&self) -> Result<fs::File> {
        self.input.try_clone().context(TtyError)
    }

    pub fn output(&self) -> Result<fs::File> {
        self.output.try_clone().context(TtyError)
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
