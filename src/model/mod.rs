//! Module for managing the state of a Klondike game.

pub use self::area::AreaId;
pub use self::card::{Card, Color, Rank, Suit};
pub use self::deck::Deck;
pub use self::game::Game;
pub use self::settings::Settings;

pub mod area;
pub mod area_list;
pub mod card;
pub mod dealer;
pub mod deck;
pub mod game;
pub mod settings;
pub mod stack;
