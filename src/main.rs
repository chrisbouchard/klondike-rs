#![feature(const_fn)]

extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate log;
extern crate rand;
extern crate simplelog;

use std::fs::File;
use std::result::Result;

use failure::Error;
use rand::seq::SliceRandom;
use rand::thread_rng;
use simplelog::*;

use crate::display::*;
use crate::game::*;

mod display;
mod game;
mod utils;

static LOG_FILE: &'static str = "klondike.log";

fn main() -> Result<(), Error> {
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create(LOG_FILE)?,
    )?;

    info!("STARTING KLONDIKE");

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
