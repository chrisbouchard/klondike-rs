use super::{
    area::{
        area_list::AreaList, foundation::UnselectedFoundation, stock::UnselectedStock,
        tableaux::UnselectedTableaux, talon::UnselectedTalon, Area, AreaId,
    },
    deck::Deck,
    settings::Settings,
    stack::Stack,
};
use crate::model::area::UnselectedArea;

#[derive(Debug)]
pub struct Game<'a> {
    areas: AreaList<'a>,
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

        Game { areas, settings }
    }

    pub fn stack(&self, area_id: AreaId) -> Stack {
        self.areas.get_by_area_id(area_id).as_stack()
    }

    pub fn move_to(self, area_id: AreaId) -> Game<'a> {
        let moves = vec![area_id];
        self.make_first_valid_move(moves)
    }

    pub fn move_to_foundation(self) -> Game<'a> {
        let moves = self.settings.foundation_indices().map(AreaId::Foundation);
        self.make_first_valid_move(moves)
    }

    pub fn move_left(self) -> Game<'a> {
        // Skip the first (selected) area id, then iterate the remainder in reverse order (right-to-
        // left).
        let moves = self
            .areas
            .iter_left_from_selection()
            .map(Area::id)
            .collect::<Vec<_>>();

        self.make_first_valid_move(moves)
    }

    pub fn move_right(self) -> Game<'a> {
        // Skip the first (selected) area id.
        let moves = self
            .areas
            .iter_right_from_selection()
            .map(Area::id)
            .collect::<Vec<_>>();

        self.make_first_valid_move(moves)
    }

    pub fn move_up(mut self) -> Game<'a> {
        self.areas.selected_mut().select_more();
        self
    }

    pub fn move_down(mut self) -> Game<'a> {
        self.areas.selected_mut().select_less();
        self
    }

    fn make_first_valid_move<I>(mut self, moves: I) -> Self
    where
        I: IntoIterator<Item = AreaId>,
    {
        for area_id in moves {
            debug!("Attempting to move selection to {:?}", area_id);

            let (areas, success) = self.areas.move_selection(area_id);
            self.areas = areas;

            if success {
                break;
            }
        }

        self
    }

    pub fn activate(mut self) -> Game<'a> {
        self.areas = self.areas.activate_selected();
        self
    }
}
