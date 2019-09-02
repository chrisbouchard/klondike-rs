#[macro_use]
extern crate log;

use std::{convert::TryFrom, error::Error, fs::File};

use log::LevelFilter;
use rand::{seq::SliceRandom, thread_rng};
use simplelog::{Config, WriteLogger};
use termion::{event::Key, input::TermRead};

use klondike_lib::{
    display::DisplayState,
    engine::{GameEngineBuilder, Update},
    model::{game::Action, AreaId, Deck, Game, Settings, Suit},
    terminal::Terminal,
};

static LOG_FILE: &'static str = "klondike.log";

fn main() -> Result<(), Box<dyn Error>> {
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create(LOG_FILE)?,
    )?;
    log_panics::init();

    info!("STARTING KLONDIKE");

    let terminal = Terminal::new()?;
    let input = terminal.input()?;
    let output = terminal.output()?;

    let settings = Settings::default();

    let game = {
        let mut deck = Deck::new();
        deck.cards_mut().shuffle(&mut thread_rng());
        Game::new(&mut deck, &settings)
    };

    let mut game_engine = GameEngineBuilder::playing(game)
        .input_mapper(DisplayState::Playing, handle_playing_input)
        .input_mapper(DisplayState::HelpMessageOpen, handle_help_input)
        .output(output)
        .start()?;

    for key in input.keys() {
        let key = key?;
        debug!("Read key: {:?}", key);
        game_engine.handle_input(key)?;

        if game_engine.state() == DisplayState::Quitting {
            break;
        }
    }

    info!("QUITTING KLONDIKE");

    Ok(())
}

fn handle_playing_input(key: Key) -> Option<Update> {
    match key {
        Key::Char('q') => Some(Update::State(DisplayState::Quitting)),
        Key::Char('?') => Some(Update::State(DisplayState::HelpMessageOpen)),

        Key::Char('s') => Some(Update::Action(Action::MoveTo(AreaId::Stock))),
        Key::Char('t') => Some(Update::Action(Action::MoveTo(AreaId::Talon))),

        Key::Char('f') => Some(Update::Action(Action::MoveToFoundation)),

        Key::Char('h') | Key::Left => Some(Update::Action(Action::MoveLeft)),
        Key::Char('j') | Key::Down => Some(Update::Action(Action::SelectLess)),
        Key::Char('k') | Key::Up => Some(Update::Action(Action::SelectMore)),
        Key::Char('l') | Key::Right => Some(Update::Action(Action::MoveRight)),

        Key::Char(c @ '1'..='7') => {
            if let Some(index) = c.to_digit(10) {
                let area_id = AreaId::Tableaux(index as u8 - 1);
                Some(Update::Action(Action::MoveTo(area_id)))
            } else {
                None
            }
        }

        Key::F(i @ 1..=4) => {
            let area_id = AreaId::Foundation(Suit::try_from(i as u8 - 1).unwrap());
            Some(Update::Action(Action::MoveTo(area_id)))
        }

        Key::Char('-') => Some(Update::Action(Action::MoveBack)),
        Key::Esc => Some(Update::Action(Action::ReturnHeld)),

        Key::Char(' ') | Key::Char('\n') => Some(Update::Action(Action::Activate)),

        _ => None,
    }
}

fn handle_help_input(key: Key) -> Option<Update> {
    match key {
        _ => Some(Update::State(DisplayState::Playing)),
    }
}
