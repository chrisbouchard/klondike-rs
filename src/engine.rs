//! Module tying together the Klondike model and display.

use std::{collections::HashMap, fmt};

use crate::{
    display::{DisplayState, Result},
    model::{
        area::AreaId,
        game::{Action, Game},
    },
};

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

pub trait RepaintWatcher {
    fn full_repaint_required(&mut self) -> Result<bool>;
}

impl<F> RepaintWatcher for F
where
    F: FnMut() -> Result<bool>,
{
    fn full_repaint_required(&mut self) -> Result<bool> {
        self()
    }
}

pub trait Repainter {
    fn repaint_full(&mut self, game: &Game, state: DisplayState) -> Result;
    fn repaint_areas(&mut self, game: &Game, area_ids: &[AreaId]) -> Result;
}

pub struct GameEngine<'a, I, R> {
    game: Game<'a>,
    state: DisplayState,
    input_mappers: HashMap<DisplayState, Box<dyn InputMapper<I> + 'a>>,
    repainter: R,
    repaint_watchers: Vec<Box<dyn RepaintWatcher + 'a>>,
}

impl<'a, I, R> GameEngine<'a, I, R>
where
    R: Repainter,
{
    pub fn game(&self) -> &Game<'a> {
        &self.game
    }

    pub fn state(&self) -> DisplayState {
        self.state
    }

    pub fn handle_input(&mut self, input: I) -> Result {
        if let Some(input_mapper) = self.input_mappers.get_mut(&self.state) {
            match input_mapper.map_input(input) {
                Some(Update::Action(action)) => {
                    let area_ids = self.game.apply_action(action);
                    if self.repaint_full()? {
                        self.repainter.repaint_full(&self.game, self.state)?;
                    } else {
                        self.repainter.repaint_areas(&self.game, &area_ids)?;
                    }
                }
                Some(Update::State(state)) => {
                    self.state = state;
                    self.repainter.repaint_full(&self.game, state)?;
                }
                None => {}
            }
        }

        Ok(())
    }

    fn repaint_full(&mut self) -> Result<bool> {
        #[allow(clippy::redundant_closure)]
        self.repaint_watchers
            .iter_mut()
            .map(|repaint_watcher| repaint_watcher.full_repaint_required())
            .collect::<Result<Vec<bool>>>()
            .map(|v| v.into_iter().all(|b| b))
    }
}

impl<'a, I, R> fmt::Debug for GameEngine<'a, I, R> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("GameEngine")
            .field("game", &self.game)
            .field("state", &self.state)
            .finish()
    }
}

pub struct GameEngineBuilder<'a, I, R = ()> {
    game: Game<'a>,
    state: DisplayState,
    input_mappers: HashMap<DisplayState, Box<dyn InputMapper<I> + 'a>>,
    repainter: R,
    repaint_watchers: Vec<Box<dyn RepaintWatcher + 'a>>,
}

impl<'a, I> GameEngineBuilder<'a, I, ()> {
    pub fn playing(game: Game<'a>) -> Self {
        GameEngineBuilder {
            game,
            state: DisplayState::Playing,
            input_mappers: HashMap::new(),
            repainter: (),
            repaint_watchers: vec![],
        }
    }
}

impl<'a, I, R> GameEngineBuilder<'a, I, R> {
    pub fn input_mapper<M>(mut self, state: DisplayState, input_mapper: M) -> Self
    where
        M: InputMapper<I> + 'a,
    {
        let _ = self.input_mappers.insert(state, Box::new(input_mapper));
        self
    }

    pub fn repainter<R2>(self, repainter: R2) -> GameEngineBuilder<'a, I, R2>
    where
        R2: Repainter,
    {
        GameEngineBuilder {
            game: self.game,
            state: self.state,
            input_mappers: self.input_mappers,
            repainter,
            repaint_watchers: self.repaint_watchers,
        }
    }

    pub fn repaint_watcher<W>(mut self, repaint_watcher: W) -> Self
    where
        W: RepaintWatcher + 'a,
    {
        self.repaint_watchers.push(Box::new(repaint_watcher));
        self
    }

    pub fn start(mut self) -> Result<GameEngine<'a, I, R>>
    where
        R: Repainter,
    {
        self.repainter.repaint_full(&self.game, self.state)?;

        Ok(GameEngine {
            game: self.game,
            state: self.state,
            input_mappers: self.input_mappers,
            repainter: self.repainter,
            repaint_watchers: self.repaint_watchers,
        })
    }
}

impl<'a, I, R> fmt::Debug for GameEngineBuilder<'a, I, R> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("GameEngineBuilder")
            .field("game", &self.game)
            .field("state", &self.state)
            .finish()
    }
}
