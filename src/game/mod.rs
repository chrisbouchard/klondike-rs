extern crate failure;

pub use self::card::*;
pub use self::deck::*;
pub use self::game::*;
pub use self::stack::*;

pub mod card;
pub mod deck;
pub mod game;
pub mod stack;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid rank: {}", value)]
    InvalidRank {
        value: u8
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
