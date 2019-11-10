//! Module tying together the Klondike model and display.

use snafu::ResultExt;
use std::{collections::HashMap, fmt, io};
use termion::event::Key;

use crate::{
    display::{
        game::{GameWidget, GameWidgetState},
        geometry, terminal_bounds, DisplayState,
    },
    model::{
        area::AreaId,
        dealer::{create_dealer, Dealer},
        game::{Action, Game},
        settings::GameSettings,
    },
    utils::tuple::both,
};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("IO error: {}", source))]
    IoError { source: io::Error },

    #[snafu(display("GameEngineBuilder: {}", message))]
    GameEngineBuilderError { message: String },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Update {
    Action(Action),
    NewGame,
    State(DisplayState),
}

pub trait InputMapper {
    fn map_input(&mut self, input: Key) -> Option<Update>;
}

impl<F> InputMapper for F
where
    F: FnMut(Key) -> Option<Update>,
{
    fn map_input(&mut self, input: Key) -> Option<Update> {
        self(input)
    }
}

pub struct GameEngine<'a, I, O>
where
    I: Iterator<Item = Result<Key, io::Error>> + 'a,
    O: io::Write + 'a,
{
    settings: &'a GameSettings,
    dealer: Box<dyn Dealer>,
    game: Option<Game>,
    state: DisplayState,
    input_mappers: HashMap<DisplayState, Box<dyn InputMapper + 'a>>,
    input: I,
    output: O,
    game_widget_state: GameWidgetState,
}

impl<'a, I, O> GameEngine<'a, I, O>
where
    I: Iterator<Item = Result<Key, io::Error>> + 'a,
    O: io::Write + 'a,
{
    pub fn tick(&mut self) -> Result<bool> {
        if self.game.is_none() {
            self.game = Some(self.dealer.deal_game(self.settings));
            // Refresh to display the initial game state before getting input.
            self.refresh(&[])?;
        }

        let update = both(
            self.input.next().transpose().context(IoError)?,
            self.input_mappers.get_mut(&self.state),
        )
        .and_then(|(input, input_mapper)| input_mapper.map_input(input));

        if let Some(update) = update {
            let area_ids = match update {
                Update::Action(action) => {
                    if let Some(ref mut game) = self.game {
                        let area_ids = game.apply_action(action);

                        if game.is_win() {
                            // Refresh first to display the winning game state.
                            self.refresh(&area_ids)?;
                            self.state = DisplayState::WinMessageOpen;
                        }

                        area_ids
                    } else {
                        vec![]
                    }
                }
                Update::NewGame => {
                    self.game = None;
                    self.state = DisplayState::Playing;
                    vec![]
                }
                Update::State(state) => {
                    self.state = state;
                    vec![]
                }
            };

            self.refresh(&area_ids)?;
        }

        Ok(self.state != DisplayState::Quitting)
    }

    fn refresh(&mut self, area_ids: &[AreaId]) -> Result<()> {
        if let Some(ref game) = self.game {
            let terminal_size = terminal_bounds().context(IoError)?;

            let widget = GameWidget {
                area_ids,
                bounds: geometry::Rect::from_size(terminal_size),
                game,
                display_state: self.state,
                widget_state: &self.game_widget_state,
            };
            write!(self.output, "{}", widget).context(IoError)?;
            self.output.flush().context(IoError)?;
        }

        Ok(())
    }
}

impl<'a, I, O> fmt::Debug for GameEngine<'a, I, O>
where
    I: Iterator<Item = Result<Key, io::Error>> + 'a,
    O: io::Write + 'a,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("GameEngine")
            .field("game", &self.game)
            .field("state", &self.state)
            .finish()
    }
}

pub struct GameEngineBuilder<'a, I, O>
where
    I: IntoIterator<Item = Result<Key, io::Error>> + 'a,
    O: io::Write + 'a,
{
    settings: &'a GameSettings,
    state: DisplayState,
    input_mappers: HashMap<DisplayState, Box<dyn InputMapper + 'a>>,
    input: I,
    output: O,
}

impl<'a, I, O> GameEngineBuilder<'a, I, O>
where
    I: IntoIterator<Item = Result<Key, io::Error>> + 'a,
    O: io::Write + 'a,
{
    pub fn builder(settings: &'a GameSettings, input: I, output: O) -> Self {
        GameEngineBuilder {
            settings,
            state: DisplayState::Playing,
            input_mappers: HashMap::new(),
            input,
            output,
        }
    }

    pub fn input_mapper<M>(mut self, state: DisplayState, input_mapper: M) -> Self
    where
        M: InputMapper + 'a,
    {
        let _ = self.input_mappers.insert(state, Box::new(input_mapper));
        self
    }

    pub fn start(self) -> Result<GameEngine<'a, I::IntoIter, O>> {
        let dealer = create_dealer(self.settings.dealer);
        let game_widget_state = GameWidgetState::default();

        Ok(GameEngine {
            settings: self.settings,
            dealer,
            game: None,
            state: self.state,
            input_mappers: self.input_mappers,
            input: self.input.into_iter(),
            output: self.output,
            game_widget_state,
        })
    }
}

impl<'a, I, O> fmt::Debug for GameEngineBuilder<'a, I, O>
where
    I: IntoIterator<Item = Result<Key, io::Error>> + 'a,
    O: io::Write + 'a,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("GameEngineBuilder")
            .field("settings", &self.settings)
            .field("state", &self.state)
            .finish()
    }
}
