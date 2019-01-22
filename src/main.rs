#[macro_use]
extern crate failure_derive;

mod display;
mod game;

use display::*;
use game::*;

fn main() -> Result<()> {
    let mut display = KlondikeDisplay::init();

    let deck = CardStack {
        pile: vec![
            Card {
                face_up: false,
                rank: Rank::new(1)?,
                suit: Suit::HEARTS,
            },
            Card {
                face_up: false,
                rank: Rank::new(1)?,
                suit: Suit::HEARTS,
            },
            Card {
                face_up: false,
                rank: Rank::new(1)?,
                suit: Suit::HEARTS,
            },
        ],
        fanned: vec![],
    };

    let stack = CardStack {
        pile: vec![
            Card {
                face_up: true,
                rank: Rank::new(1)?,
                suit: Suit::HEARTS,
            },
        ],
        fanned: vec![
            Card {
                face_up: true,
                rank: Rank::new(1)?,
                suit: Suit::SPADES,
            },
            Card {
                face_up: true,
                rank: Rank::new(3)?,
                suit: Suit::DIAMONDS,
            },
            Card {
                face_up: true,
                rank: Rank::new(10)?,
                suit: Suit::CLUBS,
            }
        ],
    };

    display.draw_horizontal_card_stack(Coords { x: 1, y: 0 }, &deck);
    display.draw_horizontal_card_stack(Coords { x: 14, y: 0 }, &stack);
    display.refresh();

    Ok(())
}
