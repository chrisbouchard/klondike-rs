#[macro_use]
extern crate log;

use std::fs::File;

use log::LevelFilter;
use rand::{seq::SliceRandom, thread_rng};
use simplelog::{Config, WriteLogger};
use termion::{event::Key, input::TermRead, terminal_size};

use klondike_lib::{
    display::GameDisplay,
    model::{AreaId, Deck, Game, GameResult, Settings, Suit},
    terminal::Terminal,
};

type Result = ::std::result::Result<(), failure::Error>;

static LOG_FILE: &'static str = "klondike.log";

fn main() -> Result {
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create(LOG_FILE)?,
    )?;
    log_panics::init();

    info!("STARTING KLONDIKE");

    let terminal = Terminal::new()?;
    let input = terminal.input()?;
    let mut output = terminal.output()?;

    let settings = Settings::default();

    let mut deck = Deck::new();
    deck.cards_mut().shuffle(&mut thread_rng());

    let mut game_display = GameDisplay::new(&mut output);

    let mut game = Game::new(&mut deck, &settings);
    game_display.draw_all_areas(&game)?;
    game_display.flush()?;

    let mut current_terminal_size = terminal_size()?;

    'event_loop: for key in input.keys() {
        debug!("Read key: {:?}", key);
        let GameResult(new_game, area_ids) = match key? {
            Key::Char('q') => break 'event_loop,

            Key::Char('s') => game.move_to(AreaId::Stock),
            Key::Char('t') => game.move_to(AreaId::Talon),

            Key::Char('f') => game.move_to_foundation(),

            Key::Char('h') | Key::Left => game.move_left(),
            Key::Char('j') | Key::Down => game.move_down(),
            Key::Char('k') | Key::Up => game.move_up(),
            Key::Char('l') | Key::Right => game.move_right(),

            Key::Char(c @ '1'...'7') => {
                if let Some(index) = c.to_digit(10) {
                    game.move_to(AreaId::Tableaux(index as u8 - 1))
                } else {
                    GameResult::new_with_none(game)
                }
            }

            Key::F(i @ 1...4) => game.move_to(AreaId::Foundation(Suit::from_index(i as u8 - 1))),

            Key::Char('-') => game.move_back(),

            Key::Char(' ') => game.activate(),

            _ => GameResult::new_with_none(game),
        };

        game = new_game;

        let new_terminal_size = terminal_size()?;

        if current_terminal_size == new_terminal_size {
            for area_id in area_ids {
                game_display.draw_area(&game, area_id)?;
            }
        } else {
            current_terminal_size = new_terminal_size;
            game_display.draw_all_areas(&game)?;
        }

        game_display.flush()?;
    }

    info!("QUITTING KLONDIKE");

    Ok(())
}
