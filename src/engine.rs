use crate::{
    display::DisplayState,
    model::{
        area::AreaId,
        game::{Game, GameResult}
    }
};

pub type KeyMapperFn = 

pub struct GameEngine<'a> {
    game: Game<'a>,
    dirty_areas: Vec<AreaId>,
    state: DisplayState,
}

impl<'a> GameEngine<'a> {
    
}
