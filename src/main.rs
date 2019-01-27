#![feature(const_fn)]

extern crate failure;
extern crate klondike_lib;
#[macro_use]
extern crate log;
extern crate rand;
extern crate simplelog;

use std::fs::File;
use std::result::Result;

use failure::Error;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rustty::{Event, Terminal};
use simplelog::*;

use klondike_lib::*;

static LOG_FILE: &'static str = "klondike.log";

fn main() -> Result<(), Error> {
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create(LOG_FILE)?,
    )?;

    info!("STARTING KLONDIKE");

    let mut term = Terminal::new()?;

    let mut deck = Deck::new();
    deck.cards_mut().shuffle(&mut thread_rng());

    let game = KlondikeGame::new(&mut deck);
    term.draw_game(&game);

    term.swap_buffers()?;

    'event_loop: loop {
        if let Some(Event::Key(c)) = term.get_event(None)? {
            match c {
                'q' => break 'event_loop,
                _ => {}
            }
        }
    }

    Ok(())
}
