#![feature(const_fn)]

extern crate failure;
extern crate klondike_lib;
#[macro_use]
extern crate log;
extern crate rand;
extern crate simplelog;
extern crate termion;

use std::fs::File;
use std::result::Result;

use failure::Error;
use rand::seq::SliceRandom;
use rand::thread_rng;
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

    let mut tty = get_tty();

    let mut deck = Deck::new();
    deck.cards_mut().shuffle(&mut thread_rng());

    let mut game = KlondikeGame::new(&mut deck);

    'event_loop: loop {
        term.clear()?;
        term.draw_game(&game);
        term.swap_buffers()?;

        if let Some(input) = get_input(&mut term)? {
            debug!("Received input: {:?}", input);
            game = match input {
                Input::Key('q') => break 'event_loop,

                Input::Key('s') => game.move_to_stock(),
                Input::Key('t') => game.move_to_talon(),

                Input::Key('h') | Input::Left => game.move_left(),
                Input::Key('j') | Input::Down => game.move_down(),
                Input::Key('k') | Input::Up => game.move_up(),
                Input::Key('l') | Input::Right => game.move_right(),

                Input::Key(c @ '1'...'7') =>
                    if let Some(index) = c.to_digit(10) {
                        game.move_to_tableaux(index as usize - 1)
                    } else {
                        game
                    },

                Input::F(i @ 1...4) => game.move_to_foundation(i as usize),

                Input::Key(' ') => game.activate(),

                _ => game,
            }
        }
    }

    Ok(())
}
