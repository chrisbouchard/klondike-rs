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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Input {
    Key(char),
    F(u8),
    Up,
    Down,
    Left,
    Right,
    Unknown,
}


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

fn get_input(term: &mut Terminal) -> Result<Option<Input>, ::std::io::Error> {
    static ESCAPE: char = '\u{1b}';

    if let Some(Event::Key(c)) = term.get_event(None)? {
        if c == ESCAPE {
            debug!("Read Escape");
            parse_escape(term)
        } else {
            Ok(Some(Input::Key(c)))
        }
    } else {
        Ok(None)
    }
}

fn parse_escape(term: &mut Terminal) -> Result<Option<Input>, ::std::io::Error> {
    let event = term.get_event(None)?;
    debug!("Read {:?}", event);

    match event {
        /* For now we can only handle escapes like ^[OX. */
        Some(Event::Key('O')) => {
            let event = term.get_event(None)?;
            debug!("Read {:?}", event);

            match event {
                Some(Event::Key('A')) => Ok(Some(Input::Up)),
                Some(Event::Key('B')) => Ok(Some(Input::Down)),
                Some(Event::Key('C')) => Ok(Some(Input::Right)),
                Some(Event::Key('D')) => Ok(Some(Input::Left)),

                Some(Event::Key('P')) => Ok(Some(Input::F(1))),
                Some(Event::Key('Q')) => Ok(Some(Input::F(2))),
                Some(Event::Key('R')) => Ok(Some(Input::F(3))),
                Some(Event::Key('S')) => Ok(Some(Input::F(4))),

                _ => Ok(Some(Input::Unknown)),
            }
        }

        _ => Ok(Some(Input::Unknown)),
    }
}
