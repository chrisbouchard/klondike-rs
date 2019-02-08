#![feature(const_fn)]

extern crate failure;
extern crate klondike_lib;
#[macro_use]
extern crate log;
extern crate rand;
extern crate simplelog;
extern crate termion;

use std::fs::File;
use std::io::{stdin, stdout, Write};

use failure::Error;
use rand::seq::SliceRandom;
use rand::thread_rng;
use simplelog::*;
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use klondike_lib::*;

type Result = ::std::result::Result<(), Error>;

static LOG_FILE: &'static str = "klondike.log";

fn main() -> Result {
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create(LOG_FILE)?,
    )?;

    info!("STARTING KLONDIKE");

    let input = stdin();
    let mut output =
        AlternateScreen::from(stdout().into_raw_mode()?);

    let mut deck = Deck::new();
    deck.cards_mut().shuffle(&mut thread_rng());

    let mut game = KlondikeGame::new(&mut deck);
    clear_and_draw_game(&mut output, &mut game)?;

    'event_loop: for key in input.keys() {
        game = match key? {
            Key::Char('q') => break 'event_loop,

            Key::Char('s') => game.move_to_stock(),
            Key::Char('t') => game.move_to_talon(),

            Key::Char('h') | Key::Left => game.move_left(),
            Key::Char('j') | Key::Down => game.move_down(),
            Key::Char('k') | Key::Up => game.move_up(),
            Key::Char('l') | Key::Right => game.move_right(),

            Key::Char(c @ '1'...'7') =>
                if let Some(index) = c.to_digit(10) {
                    game.move_to_tableaux(index as usize - 1)
                } else {
                    game
                },

            Key::F(i @ 1...4) => game.move_to_foundation(i as usize),

            Key::Char(' ') => game.activate(),

            _ => game,
        };

        clear_and_draw_game(&mut output, &mut game)?;
    }

    Ok(())
}

fn clear_and_draw_game<W>(output: &mut W, game: &mut KlondikeGame)  -> Result where W: Write {
    write!(output, "{}", clear::All)?;
    output.draw_game(&game)?;
    write!(output, "{}", cursor::Hide)?;
    output.flush()?;

    Ok(())
}
