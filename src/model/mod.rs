pub use self::card::{
    Card,
    Color,
    Rank,
    Suit
};
pub use self::deck::Deck;
pub use self::game::{
    area::AreaId,
    KlondikeGame
};

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
