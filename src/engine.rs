//! Module tying together the Klondike model and display.

use snafu::{OptionExt, ResultExt};
use std::{collections::HashMap, fmt, io};

use crate::{
    display::{
        game::{GameWidget, GameWidgetState},
        geometry, terminal_bounds, DisplayState,
    },
    model::game::{Action, Game},
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
    State(DisplayState),
}

pub trait InputMapper<I> {
    fn map_input(&mut self, input: I) -> Option<Update>;
}

impl<F, I> InputMapper<I> for F
where
    F: FnMut(I) -> Option<Update>,
{
    fn map_input(&mut self, input: I) -> Option<Update> {
        self(input)
    }
}

pub struct GameEngine<'a, I, O>
where
    I: 'a,
    O: io::Write + 'a,
{
    game: Game<'a>,
    state: DisplayState,
    input_mappers: HashMap<DisplayState, Box<dyn InputMapper<I> + 'a>>,
    output: O,
    game_widget_state: GameWidgetState,
}

impl<'a, I, O> GameEngine<'a, I, O>
where
    I: 'a,
    O: io::Write + 'a,
{
    pub fn game(&self) -> &Game<'a> {
        &self.game
    }

    pub fn state(&self) -> DisplayState {
        self.state
    }

    pub fn handle_input(&mut self, input: I) -> Result<()> {
        if let Some(input_mapper) = self.input_mappers.get_mut(&self.state) {
            let update = input_mapper.map_input(input);

            let area_ids = update.map(|update| match update {
                Update::Action(action) => {
                    let area_ids = self.game.apply_action(action);

                    if self.game.is_win() {
                        self.state = DisplayState::WinMessageOpen;
                    }

                    area_ids
                }
                Update::State(state) => {
                    self.state = state;
                    vec![]
                }
            });

            if let Some(area_ids) = area_ids {
                let terminal_size = terminal_bounds().context(IoError)?;

                let widget = GameWidget {
                    area_ids,
                    bounds: geometry::Rect::from_size(terminal_size),
                    game: &self.game,
                    display_state: self.state,
                    widget_state: &self.game_widget_state,
                };
                write!(self.output, "{}", widget).context(IoError)?;
                self.output.flush().context(IoError)?;
            }
        }

        Ok(())
    }
}

impl<'a, I, O> fmt::Debug for GameEngine<'a, I, O>
where
    I: 'a,
    O: io::Write + 'a,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("GameEngine")
            .field("game", &self.game)
            .field("state", &self.state)
            .finish()
    }
}

pub struct GameEngineBuilder<'a, I, O> {
    game: Game<'a>,
    state: DisplayState,
    input_mappers: HashMap<DisplayState, Box<dyn InputMapper<I> + 'a>>,
    output: Option<O>,
}

impl<'a, I, O> GameEngineBuilder<'a, I, O>
where
    I: 'a,
    O: io::Write + 'a,
{
    pub fn playing(game: Game<'a>) -> Self {
        GameEngineBuilder {
            game,
            state: DisplayState::Playing,
            input_mappers: HashMap::new(),
            output: None,
        }
    }

    pub fn input_mapper<M>(mut self, state: DisplayState, input_mapper: M) -> Self
    where
        M: InputMapper<I> + 'a,
    {
        let _ = self.input_mappers.insert(state, Box::new(input_mapper));
        self
    }

    pub fn output(mut self, output: O) -> Self {
        self.output = Some(output);
        self
    }

    pub fn start(self) -> Result<GameEngine<'a, I, O>> {
        let game = self.game;
        let mut output = self.output.context(GameEngineBuilderError {
            message: "Required field output undefined",
        })?;
        let game_widget_state = GameWidgetState::default();

        {
            let terminal_size = terminal_bounds().context(IoError)?;

            let widget = GameWidget {
                area_ids: vec![],
                bounds: geometry::Rect::from_size(terminal_size),
                game: &game,
                display_state: self.state,
                widget_state: &game_widget_state,
            };
            write!(output, "{}", widget).context(IoError)?;
            output.flush().context(IoError)?;
        }

        Ok(GameEngine {
            game,
            state: self.state,
            input_mappers: self.input_mappers,
            output,
            game_widget_state,
        })
    }
}

impl<'a, I, O> fmt::Debug for GameEngineBuilder<'a, I, O>
where
    I: 'a,
    O: io::Write + 'a,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("GameEngineBuilder")
            .field("game", &self.game)
            .field("state", &self.state)
            .finish()
    }
}
