use super::{
    area::{
        area_list::{AreaList, AreaListResult},
        foundation::UnselectedFoundation,
        stock::UnselectedStock,
        tableaux::UnselectedTableaux,
        talon::UnselectedTalon,
        Area, AreaId,
    },
    deck::Deck,
    settings::Settings,
    stack::Stack,
};
use crate::model::area::UnselectedArea;

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
                let cards = deck.deal(index + 1);
                UnselectedTableaux::create(index, 1, cards, settings)
            })
            .collect::<Vec<_>>();

        let stock = {
            let cards = deck.deal_rest();
            UnselectedStock::create(cards, settings)
        };

        let talon = UnselectedTalon::create(vec![], 0, settings);

        let mut foundation = settings
            .foundation_indices()
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

    pub fn area_ids(&self) -> Vec<AreaId> {
        self.areas.area_ids()
    }

    pub fn stack(&self, area_id: AreaId) -> Stack {
        self.areas.get_by_area_id(area_id).as_stack()
    }

    pub fn move_to(self, area_id: AreaId) -> GameResult<'a> {
        let moves = vec![area_id];
        self.make_first_valid_move(moves)
    }

    pub fn move_back(self) -> GameResult<'a> {
        let moves = vec![self.last_area];
        self.make_first_valid_move(moves)
    }

    pub fn move_to_foundation(self) -> GameResult<'a> {
        let moves = self.settings.foundation_indices().map(AreaId::Foundation);
        self.make_first_valid_move(moves)
    }

    pub fn move_left(self) -> GameResult<'a> {
        // Skip the first (selected) area id, then iterate the remainder in reverse order (right-to-
        // left).
        let moves = self
            .areas
            .iter_left_from_selection()
            .map(Area::id)
            .collect::<Vec<_>>();

        self.make_first_valid_move(moves)
    }

    pub fn move_right(self) -> GameResult<'a> {
        // Skip the first (selected) area id.
        let moves = self
            .areas
            .iter_right_from_selection()
            .map(Area::id)
            .collect::<Vec<_>>();

        self.make_first_valid_move(moves)
    }

    pub fn move_up(mut self) -> GameResult<'a> {
        self.areas.selected_mut().select_more();
        GameResult::new_with_selected(self)
    }

    pub fn move_down(mut self) -> GameResult<'a> {
        self.areas.selected_mut().select_less();
        GameResult::new_with_selected(self)
    }

    fn make_first_valid_move<I>(mut self, moves: I) -> GameResult<'a>
    where
        I: IntoIterator<Item = AreaId>,
    {
        let new_last_area = self.areas.selected().id();

        for new_area_id in moves {
            debug!("Attempting to move selection to {:?}", new_area_id);

            let AreaListResult(new_areas, area_ids) = self.areas.move_selection(new_area_id);
            self.areas = new_areas;

            if !area_ids.is_empty() {
                self.last_area = new_last_area;
                return GameResult::new(self, area_ids);
            }
        }

        GameResult::new_with_none(self)
    }

    pub fn activate(mut self) -> GameResult<'a> {
        let AreaListResult(new_areas, area_ids) = self.areas.activate_selected();
        self.areas = new_areas;
        GameResult::new(self, area_ids)
    }
}

#[derive(Debug)]
pub struct GameResult<'a>(pub Game<'a>, pub Vec<AreaId>);

impl<'a> GameResult<'a> {
    pub fn new(game: Game<'a>, area_ids: impl IntoIterator<Item = AreaId>) -> GameResult<'a> {
        GameResult(game, area_ids.into_iter().collect())
    }

    pub fn new_with_none(game: Game<'a>) -> GameResult<'a> {
        GameResult(game, vec![])
    }

    pub fn new_with_one(game: Game<'a>, area_id: AreaId) -> GameResult<'a> {
        GameResult(game, vec![area_id])
    }

    pub fn new_with_selected(game: Game<'a>) -> GameResult<'a> {
        let selected_area_id = game.areas.selected().id();
        GameResult(game, vec![selected_area_id])
    }
}
