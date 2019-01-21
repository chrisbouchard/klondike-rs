#[macro_use]
extern crate failure_derive;

mod display;
mod game;

use display::*;
use game::*;

fn main() -> Result<()> {
    let mut display = KlondikeDisplay::init();

    let card1 = Card {
        face_up: true,
        rank: Rank::new(3)?,
        suit: Suit::DIAMONDS
    };

    let card2 = Card {
        face_up: true,
        rank: Rank::new(10)?,
        suit: Suit::CLUBS
    };

    let card3 = Card {
        face_up: false,
        rank: Rank::new(1)?,
        suit: Suit::SPADES
    };

    display.display_full_card(0, 0, &card1);
    display.display_full_card(0, 12, &card2);
    display.display_full_card(0, 24, &card3);

    Ok(())
}
