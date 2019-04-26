#[macro_use]
extern crate log;

use std::fs::File;
use std::io::Write;

use log::LevelFilter;
use rand::{seq::SliceRandom, thread_rng};
use simplelog::{Config, WriteLogger};
use termion::{clear, event::Key, input::TermRead};

use klondike_lib::{
    display::GameDisplay,
    model::{AreaId, Deck, Game, Settings},
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

    let mut game = Game::new(&mut deck, &settings);
    clear_and_draw_game(&mut output, &mut game)?;

    'event_loop: for key in input.keys() {
        debug!("Read key: {:?}", key);
        game = match key? {
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
                    game.move_to(AreaId::Tableaux(index as usize - 1))
                } else {
                    game
                }
            }

            Key::F(i @ 1...4) => game.move_to(AreaId::Foundation(i as usize - 1)),

            Key::Char(' ') => game.activate(),

            _ => game,
        };

        clear_and_draw_game(&mut output, &mut game)?;
    }

    info!("QUITTING KLONDIKE");

    Ok(())
}

fn clear_and_draw_game<W>(display: &mut GameDisplay<W>, game: &mut Game) -> Result
where
    W: Write,
{
    write!(display, "{}", clear::All)?;
    // TODO: Get the areas to display
    display.draw_area(&game, AreaId::Stock)?;
    display.flush()?;

    Ok(())
}
