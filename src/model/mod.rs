pub use self::area::AreaId;
pub use self::card::{Card, Color, Rank, Suit};
pub use self::deck::Deck;
pub use self::game::{Game, GameResult};
pub use self::settings::Settings;

pub mod area;
pub mod card;
pub mod deck;
pub mod game;
pub mod settings;
pub mod stack;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "duplicate area ids: {:?}", area_ids)]
    DuplicateAreaIds { area_ids: Vec<AreaId> },

    #[fail(display = "area list must not be empty")]
    EmptyAreaList,

    #[fail(display = "invalid rank: {}", value)]
    InvalidRank { value: u8 },

    #[fail(display = "unknown area id: {:?}", area_id)]
    UnknownAreaId { area_id: AreaId },
}

pub type Result<T> = ::std::result::Result<T, Error>;
