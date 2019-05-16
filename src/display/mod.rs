//! Module for things related to displaying a Klondike game in the terminal.

pub use self::{coords::Coords, game::GameDisplay};

pub mod blank;
pub mod bounds;
pub mod card;
pub mod coords;
pub mod error;
pub mod frame;
pub mod game;
pub mod help;
pub mod selector;
pub mod stack;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum DisplayState {
    Playing,
    HelpMessageOpen,
    Quitting,
}
