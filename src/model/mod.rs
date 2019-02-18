pub use self::area::AreaId;
pub use self::card::{Card, Color, Rank, Suit};
pub use self::deck::Deck;
pub use self::game::Game;
pub use self::settings::Settings;

pub mod area;
pub mod card;
pub mod deck;
pub mod game;
pub mod settings;
pub mod stack;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid rank: {}", value)]
    InvalidRank { value: u8 },
}

pub type Result<T> = ::std::result::Result<T, Error>;
