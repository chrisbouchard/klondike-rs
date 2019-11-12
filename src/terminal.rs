//! Module to manage a TTY in alternate mode.

use snafu::ResultExt;
use std::{
    fmt, fs,
    io::{self, Write as _},
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

pub struct TtyInput {
    tty: fs::File,
}

impl TtyInput {
    pub fn new() -> Result<Self> {
        let tty = termion::get_tty().context(TtyError)?;
        Ok(TtyInput { tty })
    }
}

impl io::Read for TtyInput {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.tty.read(buf)
    }
}

impl fmt::Debug for TtyInput {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("TtyInput").field("tty", &"...").finish()
    }
}

pub struct TtyOutput {
    tty: AlternateScreen<RawTerminal<fs::File>>,
}

impl TtyOutput {
    pub fn new() -> Result<Self> {
        let mut tty = AlternateScreen::from(
            termion::get_tty()
                .context(TtyError)?
                .into_raw_mode()
                .context(RawModeError)?,
        );

        write!(tty, "{}", cursor::Hide).context(WriteError)?;
        Ok(TtyOutput { tty })
    }
}

impl io::Write for TtyOutput {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.tty.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.tty.flush()
    }
}

impl Drop for TtyOutput {
    fn drop(&mut self) {
        write!(self.tty, "{}", cursor::Show).unwrap();
    }
}

impl fmt::Debug for TtyOutput {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("TtyOutput").field("tty", &"...").finish()
    }
}
