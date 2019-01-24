#![feature(const_fn)]

#[macro_use]
extern crate failure_derive;
extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::display::*;
use crate::game::*;

mod display;
mod game;
mod utils;

fn main() -> Result<()> {
    let mut display = KlondikeDisplay::init();

    let mut deck = Deck::new();
    deck.cards_mut().shuffle(&mut thread_rng());

    let game = KlondikeGame::new(&mut deck);
    draw_game(&mut display, &game);

    display.refresh();
    display.getch();
    display.clear();

    Ok(())
}
