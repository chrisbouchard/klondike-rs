use std::borrow::Borrow;

use super::{
    area::{
        area_list::AreaList, foundation::UnselectedFoundation, stock::UnselectedStock,
        tableaux::UnselectedTableaux, talon::UnselectedTalon, Area, AreaId, UnselectedArea,
    },
    card::Suit,
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

        let areas = AreaList::new(areas);
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
                .map_or(false, |suit| suit.rank.is_king())
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
                make_first_valid_move(game, moves)
            }
            Action::MoveBack => {
                let moves = vec![game.last_area];
                make_first_valid_move(game, moves)
            }
            Action::MoveToFoundation => {
                let moves = Suit::values().map(AreaId::Foundation);
                make_first_valid_move(game, moves)
            }
            Action::MoveLeft => {
                let moves = game
                    .areas
                    .iter_left_from_selection()
                    .map(Area::id)
                    .collect::<Vec<_>>();
                make_first_valid_move(game, moves)
            }
            Action::MoveRight => {
                let moves = game
                    .areas
                    .iter_right_from_selection()
                    .map(Area::id)
                    .collect::<Vec<_>>();
                make_first_valid_move(game, moves)
            }
            Action::SelectMore => {
                game.areas.selected_mut().select_more();
                vec![game.areas.selected().id()]
            }
            Action::SelectLess => {
                game.areas.selected_mut().select_less();
                vec![game.areas.selected().id()]
            }
            Action::Activate => game.areas.activate_selected(),
            Action::ReturnHeld => game.areas.return_held(),
        }
    }
}

fn make_first_valid_move<I>(game: &mut Game, moves: I) -> Vec<AreaId>
where
    I: IntoIterator<Item = AreaId>,
{
    let new_last_area = game.areas.selected().id();

    for new_area_id in moves {
        debug!("Attempting to move selection to {:?}", new_area_id);

        let area_ids = game.areas.move_selection(new_area_id);

        if !area_ids.is_empty() {
            game.last_area = new_last_area;
            return area_ids;
        }
    }

    vec![]
}
