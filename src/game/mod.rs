extern crate failure;

pub mod card;
pub mod game;

pub use self::card::*;
pub use self::game::*;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid rank: {}", value)]
    InvalidRank {
        value: u8
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
