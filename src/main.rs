#[macro_use]
extern crate failure_derive;

mod display;
mod game;

use display::*;
use display::card::*;
use display::stack::*;
use game::*;

fn main() -> Result<()> {
    let mut display = KlondikeDisplay::init();

    let deck = CardStack {
        pile: &[
            Card {
                face_up: false,
                rank: Rank::new(1)?,
                suit: Suit::Hearts,
            },
            Card {
                face_up: false,
                rank: Rank::new(1)?,
                suit: Suit::Hearts,
            },
            Card {
                face_up: false,
                rank: Rank::new(1)?,
                suit: Suit::Hearts,
            },
        ],
        fanned: &[],
    };

    let stack = CardStack {
        pile: &[
            Card {
                face_up: true,
                rank: Rank::new(1)?,
                suit: Suit::Hearts,
            },
        ],
        fanned: &[
            Card {
                face_up: true,
                rank: Rank::new(1)?,
                suit: Suit::Spades,
            },
            Card {
                face_up: true,
                rank: Rank::new(3)?,
                suit: Suit::Diamonds,
            },
            Card {
                face_up: true,
                rank: Rank::new(10)?,
                suit: Suit::Clubs,
            }
        ],
    };

    draw_horizontal_card_stack(&mut display, Coords { x: 1, y: 0 }, &deck);
    draw_horizontal_card_stack(&mut display, Coords { x: 12, y: 0 }, &stack);

    display.refresh();
    display.getch();
    display.clear();

    let mut y = 0;

    for &suit in Suit::values() {
        let mut x = 1;

        for rank in 1..13 {
            draw_card(
                &mut display,
                Coords { x, y },
                &Card {
                    face_up: true,
                    rank: Rank::new(rank)?,
                    suit,
                },
            );

            x += 10;
        }

        y += 2;
    }

    display.refresh();
    display.getch();
    display.clear();

    Ok(())
}
