use std::{fmt, io};

use crate::model::{Game, GameResult};

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

#[derive(Clone, Copy, Debug, Hash)]
pub enum DisplayState {
    Playing,
    HelpMessageOpen,
    Quitting,
}

#[derive(Debug)]
pub struct DisplayResult<'a>(pub GameResult<'a>, pub Option<DisplayState>);

impl<'a> DisplayResult<'a> {
    pub fn new(game_result: GameResult<'a>, new_state: DisplayState) -> DisplayResult<'a> {
        DisplayResult(game_result, Some(new_state))
    }

    pub fn with_game_result(game_result: GameResult<'a>) -> DisplayResult<'a> {
        DisplayResult(game_result, None)
    }

    pub fn with_new_state(game: Game<'a>, new_state: DisplayState) -> DisplayResult<'a> {
        DisplayResult(GameResult::new_with_none(game), Some(new_state))
    }

    pub fn with_no_change(game: Game<'a>) -> DisplayResult<'a> {
        DisplayResult(GameResult::new_with_none(game), None)
    }
}
