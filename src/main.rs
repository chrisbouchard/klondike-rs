#[macro_use]
extern crate log;

use std::fs::File;
use std::io::Write;

use log::LevelFilter;
use rand::{seq::SliceRandom, thread_rng};
use simplelog::{Config, WriteLogger};
use termion::{clear, event::Key, input::TermRead};

use klondike_lib::{
    display::GamePainter,
    model::{Deck, KlondikeGame},
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

    let mut deck = Deck::new();
    deck.cards_mut().shuffle(&mut thread_rng());

    let mut game = KlondikeGame::new(&mut deck);
    clear_and_draw_game(&mut output, &mut game)?;

    'event_loop: for key in input.keys() {
        debug!("Read key: {:?}", key);
        game = match key? {
            Key::Char('q') => break 'event_loop,

            Key::Char('s') => game.move_to_stock(),
            Key::Char('t') => game.move_to_talon(),

            Key::Char('h') | Key::Left => game.move_left(),
            Key::Char('j') | Key::Down => game.move_down(),
            Key::Char('k') | Key::Up => game.move_up(),
            Key::Char('l') | Key::Right => game.move_right(),

            Key::Char(c @ '1'...'7') => {
                if let Some(index) = c.to_digit(10) {
                    game.move_to_tableaux(index as usize - 1)
                } else {
                    game
                }
            }

            Key::F(i @ 1...4) => game.move_to_foundation(i as usize - 1),

            Key::Char(' ') => game.activate(),

            _ => game,
        };

        clear_and_draw_game(&mut output, &mut game)?;
    }

    info!("QUITTING KLONDIKE");

    Ok(())
}

fn clear_and_draw_game<W>(output: &mut W, game: &mut KlondikeGame) -> Result
where
    W: Write,
{
    write!(output, "{}", clear::All)?;
    output.draw_game(&game)?;
    output.flush()?;

    Ok(())
}
