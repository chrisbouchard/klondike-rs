use std::{convert::TryFrom, error::Error, fs};

use log::{info, LevelFilter};
use num_traits::ToPrimitive;
use simplelog::{Config, WriteLogger};
use termion::{event::Key, input::TermRead};

use klondike_lib::{
    display::DisplayState,
    engine::{GameEngineBuilder, Update},
    model::{game::Action, AreaId, Settings, Suit},
    terminal::Terminal,
};

static LOG_FILE: &str = "klondike.log";

fn main() -> Result<(), Box<dyn Error>> {
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        fs::File::create(LOG_FILE)?,
    )?;
    log_panics::init();

    info!("STARTING KLONDIKE");

    let terminal = Terminal::new()?;
    let input = terminal.input()?;
    let output = terminal.output()?;

    let settings = Settings::read_from_system()?;

    let mut engine = GameEngineBuilder::builder(&settings.game, input.keys(), output)
        .input_mapper(DisplayState::Playing, handle_playing_input)
        .input_mapper(DisplayState::HelpMessageOpen, handle_help_input)
        .input_mapper(DisplayState::WinMessageOpen, handle_win_input)
        .start()?;

    while engine.tick()? {}

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

        Key::Char(c @ '1'..='9') => {
            if let Some(index) = c.to_digit(10) {
                let area_id = AreaId::Tableaux(index.to_u8()? - 1);
                Some(Update::Action(Action::MoveTo(area_id)))
            } else {
                None
            }
        }

        Key::F(i @ 1..=4) => {
            let area_id = AreaId::Foundation(Suit::try_from(i.to_u8()? - 1).unwrap());
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

fn handle_win_input(key: Key) -> Option<Update> {
    match key {
        Key::Char('y') => Some(Update::NewGame),
        Key::Char('n') => Some(Update::State(DisplayState::Quitting)),
        _ => None,
    }
}
