use std::borrow::Borrow;

use log::debug;

use super::{
    area::{Area, AreaId},
    area_list::AreaList,
    card::{Rank, Suit},
    stack::Stack,
};

#[derive(Debug)]
pub struct Game {
    pub areas: AreaList,
    pub last_area: AreaId,
}

impl Game {
    pub fn new(areas: AreaList) -> Game {
        let last_area = areas.selected().id();

        Game { areas, last_area }
    }

    pub fn is_win(&self) -> bool {
        Suit::values()
            .flat_map(|suit| self.areas.get_by_area_id(AreaId::Foundation(suit)))
            .all(|foundation| {
                let held = foundation.is_held();
                let complete = foundation
                    .peek_top_card()
                    .map(|suit| suit.rank == Rank::King)
                    .unwrap_or_default();

                !held && complete
            })
    }

    pub fn area_ids(&self) -> Vec<AreaId> {
        self.areas.area_ids()
    }

    pub fn stack(&self, area_id: AreaId) -> Option<Stack<'_>> {
        self.areas.get_by_area_id(area_id).ok().map(Area::as_stack)
    }

    pub fn apply_action(&mut self, action: Action) -> Vec<AreaId> {
        action.borrow().apply(self)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Action {
    MoveTo(AreaId),
    MoveBack,
    MoveToFoundation,
    MoveLeft,
    MoveRight,
    SelectMore,
    SelectLess,
    Activate,
    ReturnHeld,
}

impl Action {
    fn apply(self, game: &mut Game) -> Vec<AreaId> {
        match self {
            Action::MoveTo(area_id) => {
                let moves = vec![area_id];
                self.make_first_valid_move(game, moves)
            }
            Action::MoveBack => {
                let moves = vec![game.last_area];
                self.make_first_valid_move(game, moves)
            }
            Action::MoveToFoundation => {
                let moves = Suit::values().map(AreaId::Foundation);
                self.make_first_valid_move(game, moves)
            }
            Action::MoveLeft => {
                let moves = game
                    .areas
                    .iter_left_from_selection()
                    .map(Area::id)
                    .collect::<Vec<_>>();
                self.make_first_valid_move(game, moves)
            }
            Action::MoveRight => {
                let moves = game
                    .areas
                    .iter_right_from_selection()
                    .map(Area::id)
                    .collect::<Vec<_>>();
                self.make_first_valid_move(game, moves)
            }
            Action::SelectMore => game.areas.select_more().unwrap_or_else(|error| {
                debug!("Unable to select more: {}", error);
                vec![]
            }),
            Action::SelectLess => game.areas.select_less().unwrap_or_else(|error| {
                debug!("Unable to select less: {}", error);
                vec![]
            }),
            Action::Activate => game.areas.activate_selected().unwrap_or_else(|error| {
                debug!("Unable to activate: {}", error);
                vec![]
            }),
            Action::ReturnHeld => game.areas.return_held().unwrap_or_else(|error| {
                debug!("Unable to return held: {}", error);
                vec![]
            }),
        }
    }

    fn make_first_valid_move<I>(self, game: &mut Game, moves: I) -> Vec<AreaId>
    where
        I: IntoIterator<Item = AreaId>,
    {
        let new_last_area = game.areas.selected().id();

        for new_area_id in moves {
            debug!("Attempting to move selection to {:?}", new_area_id);

            match game.areas.move_selection(new_area_id) {
                Ok(area_ids) => {
                    game.last_area = new_last_area;
                    return area_ids;
                }
                Err(error) => {
                    debug!("Unable to move to {:?}: {}", new_area_id, error);
                }
            }
        }

        vec![]
    }
}
