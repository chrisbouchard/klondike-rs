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

    pub fn move_to(&mut self, area_id: AreaId) -> Vec<AreaId> {
        let moves = vec![area_id];
        self.make_first_valid_move(moves)
    }

    pub fn move_back(&mut self) -> Vec<AreaId> {
        let moves = vec![self.last_area];
        self.make_first_valid_move(moves)
    }

    pub fn move_to_foundation(&mut self) -> Vec<AreaId> {
        let moves = Suit::values().map(AreaId::Foundation);
        self.make_first_valid_move(moves)
    }

    pub fn move_left(&mut self) -> Vec<AreaId> {
        // Skip the first (selected) area id, then iterate the remainder in reverse order (right-to-
        // left).
        let moves = self
            .areas
            .iter_left_from_selection()
            .map(Area::id)
            .collect::<Vec<_>>();

        self.make_first_valid_move(moves)
    }

    pub fn move_right(&mut self) -> Vec<AreaId> {
        // Skip the first (selected) area id.
        let moves = self
            .areas
            .iter_right_from_selection()
            .map(Area::id)
            .collect::<Vec<_>>();

        self.make_first_valid_move(moves)
    }

    pub fn move_up(&mut self) -> Vec<AreaId> {
        self.areas.selected_mut().select_more();
        vec![self.areas.selected().id()]
    }

    pub fn move_down(&mut self) -> Vec<AreaId> {
        self.areas.selected_mut().select_less();
        vec![self.areas.selected().id()]
    }

    fn make_first_valid_move<I>(&mut self, moves: I) -> Vec<AreaId>
    where
        I: IntoIterator<Item = AreaId>,
    {
        let new_last_area = self.areas.selected().id();

        for new_area_id in moves {
            debug!("Attempting to move selection to {:?}", new_area_id);

            let area_ids = self.areas.move_selection(new_area_id);

            if !area_ids.is_empty() {
                self.last_area = new_last_area;
                return area_ids;
            }
        }

        vec![]
    }

    pub fn activate(&mut self) -> Vec<AreaId> {
        self.areas.activate_selected()
    }
}
