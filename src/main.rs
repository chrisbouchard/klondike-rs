#[macro_use]
extern crate log;

use std::{convert::TryFrom, error::Error, fs, path};

use directories::ProjectDirs;
use log::LevelFilter;
use num_traits::ToPrimitive;
use rand::{seq::SliceRandom, thread_rng};
use simplelog::{Config, WriteLogger};
use termion::{event::Key, input::TermRead};
use toml;

use klondike_lib::{
    display::DisplayState,
    engine::{GameEngineBuilder, Update},
    model::{game::Action, AreaId, Deck, Game, Settings, Suit},
    terminal::Terminal,
};

static QUALIFIER: &'static str = "net";
static ORGANIZATION: &'static str = "upflitinglemma";
static APPLICATION: &'static str = "klondike-rs";

static LOG_FILE: &'static str = "klondike.log";
static CONFIG_FILE: &'static str = "config.toml";

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

    let project_dirs: Option<ProjectDirs> = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION);
    let config_path: Option<path::PathBuf> = project_dirs.map(|project_dirs| {
        let mut path = project_dirs.config_dir().to_path_buf();
        path.push(CONFIG_FILE);
        info!("Looking for config file: {}", path.display());
        path
    });

    // Use File to handle missing/unreadable/etc. better
    let settings: Settings = {
        config_path
            .and_then(|config_path| fs::read_to_string(config_path).ok())
            .map(|contents| toml::from_str(&contents).unwrap())
            .unwrap_or_default()
    };

    let game = {
        let mut deck = Deck::new();
        deck.cards_mut().shuffle(&mut thread_rng());
        Game::new(&mut deck, &settings.game)
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
