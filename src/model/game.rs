use std::borrow::Borrow;

use super::{
    area::{
        foundation::UnselectedFoundation, stock::UnselectedStock, tableaux::UnselectedTableaux,
        talon::UnselectedTalon, Area, AreaId, UnselectedArea,
    },
    area_list::AreaList,
    card::{Rank, Suit},
    deck::Deck,
    settings::Settings,
    stack::Stack,
};

#[derive(Debug)]
pub struct Game<'a> {
    areas: AreaList<'a>,
    last_area: AreaId,
    settings: &'a Settings,
}

impl<'a> Game<'a> {
    pub fn new<'d>(deck: &'d mut Deck, settings: &'a Settings) -> Game<'a> {
        let mut tableaux = settings
            .tableaux_indices()
            .map(|index| {
                let cards = deck.deal(index as usize + 1);
                UnselectedTableaux::create(index, 1, cards, settings)
            })
            .collect::<Vec<_>>();

        let stock = {
            let cards = deck.deal_rest();
            UnselectedStock::create(cards, settings)
        };

        let talon = UnselectedTalon::create(vec![], 0, settings);

        let mut foundation = Suit::values()
            .map(|index| UnselectedFoundation::create(index, vec![], settings))
            .collect::<Vec<_>>();

        let mut areas: Vec<Box<dyn UnselectedArea>> = vec![stock, talon];
        areas.append(&mut foundation);
        areas.append(&mut tableaux);

        let areas = AreaList::new(areas).expect("Unable to create AreaList");
        let last_area = areas.selected().id();

        Game {
            areas,
            last_area,
            settings,
        }
    }

    pub fn is_win(&self) -> bool {
        Suit::values().all(|suit| {
            self.areas
                .get_by_area_id(AreaId::Foundation(suit))
                .peek_top_card()
                .map_or(false, |suit| suit.rank == Rank::King)
        })
    }

    pub fn area_ids(&self) -> Vec<AreaId> {
        self.areas.area_ids()
    }

    pub fn stack(&self, area_id: AreaId) -> Stack {
        self.areas.get_by_area_id(area_id).as_stack()
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
