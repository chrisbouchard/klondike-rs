use super::{
    area::{
        area_list::AreaList, foundation::UnselectedFoundation, stock::UnselectedStock,
        tableaux::UnselectedTableaux, talon::UnselectedTalon, AreaId,
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
        let mut tableaux = settings.tableaux_indices().map(|index| {
            let cards = deck.deal(index + 1);
            UnselectedTableaux::new(index, index, cards, settings)
        }).collect::<Vec<_>>();

        let stock_cards = deck.deal_rest();
        let stock = UnselectedStock::new(stock_cards, settings);

        let talon = UnselectedTalon::new(vec![], 0, settings);

        let mut foundation = settings
            .foundation_indices()
            .map(|index| UnselectedFoundation::new(index, vec![], settings))
            .collect::<Vec<_>>();

        let mut areas: Vec<Box<dyn UnselectedArea>> = vec![stock, talon];
        areas.append(&mut foundation);
        areas.append(&mut tableaux);

        let areas = AreaList::new(areas);

        Game { areas, settings }
    }

    pub fn stack<'b>(&'b self, area_id: AreaId) -> Stack<'b> {
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
            .iter_from_selected()
            .skip(1)
            .rev()
            .map(|area| area.id())
            .collect::<Vec<_>>();

        self.make_first_valid_move(moves)
    }

    pub fn move_right(self) -> Game<'a> {
        // Skip the first (selected) area id.
        let moves = self
            .areas
            .iter_from_selected()
            .skip(1)
            .map(|area| area.id())
            .collect::<Vec<_>>();

        self.make_first_valid_move(moves)
    }

    pub fn move_up(self) -> Game<'a> {
        // if let SelectionMode::Cards(len) = self.selection.mode {
        //     let mode = SelectionMode::Cards(len + 1);
        //     let moves_iter = once(self.selection.target);

        //     if self.make_first_valid_move(&mode, moves_iter).is_some() {
        //         self.selection = self.selection.select(mode);
        //     }
        // }

        self
    }

    pub fn move_down(self) -> Game<'a> {
        // if let SelectionMode::Cards(len) = self.selection.mode {
        //     if len > 1 {
        //         let mode = SelectionMode::Cards(len - 1);
        //         let moves_iter = once(self.selection.target);

        //         if self.make_first_valid_move(&mode, moves_iter).is_some() {
        //             self.selection = self.selection.select(mode);
        //         }
        //     }
        // }

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

    pub fn activate(self) -> Game<'a> {
        // let selected_area = self.areas.area_mut(self.selection.target);

        // match selected_area.activate(&mut self.selection.mode) {
        //     Some(Action::Draw) => {
        //         let cards = self.areas.stock.draw();
        //         self.areas.talon.place(cards);
        //         self
        //     }
        //     Some(Action::MoveTo(area_id)) => self.move_to(area_id),
        //     Some(Action::Restock) => {
        //         let cards = self.areas.talon.flip();
        //         self.areas.stock.place(cards);
        //         self
        //     }
        //     None => self,
        // }

        self
    }
}
