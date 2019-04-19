use std::iter::once;

use super::{
    area::{
        area_list::AreaList, foundation::Foundation, stock::Stock, tableaux::Tableaux,
        talon::Talon, Action, Area, AreaId,
    },
    card::Card,
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
        let tableaux = settings.tableaux_indices().map(|index| {
            let cards = deck.deal(index + 1);
            Box::new(Tableaux::new(index, index, cards, settings)) as Box<dyn UnselectedArea>
        });

        let talon = Box::new(Talon::new(Vec::new(), 0, settings));

        let foundation = settings.foundation_indices().map(|index| {
            Box::new(Foundation::new(index, Vec::new(), settings)) as Box<dyn UnselectedArea>
        });

        let stock_cards = deck.deal_rest();
        let stock = Box::new(Stock::new(stock_cards, settings));

        let mut areas: Vec<Box<dyn UnselectedArea>> = vec![stock, talon];
        areas.extend(foundation);
        areas.extend(tableaux);

        let areas = AreaList::new(areas);

        Game { areas, settings }
    }

    pub fn stack(&self, area_id: AreaId) -> Stack {
        self.areas.get_by_area_id(area_id).as_stack()
    }

    pub fn move_to(mut self, area_id: AreaId) -> Game<'a> {
        let moves = once(area_id);
        self.make_first_valid_move(moves);

        self
    }

    pub fn move_to_foundation(mut self) -> Game<'a> {
        let moves = self.settings.foundation_indices().map(AreaId::Foundation);
        self.make_first_valid_move(moves);

        self
    }

    pub fn move_left(mut self) -> Game<'a> {
        // Skip the first (selected) area id, then iterate the remainder in reverse order (right-to-
        // left).
        let moves = self.areas.area_ids_selected_first().skip(1).rev();
        self.make_first_valid_move(moves);

        self
    }

    pub fn move_right(mut self) -> Game<'a> {
        // Skip the first (selected) area id.
        let moves = self.areas.area_ids_selected_first().skip(1);
        self.make_first_valid_move(moves);

        self
    }

    pub fn move_up(mut self) -> Game<'a> {
        // if let SelectionMode::Cards(len) = self.selection.mode {
        //     let mode = SelectionMode::Cards(len + 1);
        //     let moves_iter = once(self.selection.target);

        //     if self.make_first_valid_move(&mode, moves_iter).is_some() {
        //         self.selection = self.selection.select(mode);
        //     }
        // }

        self
    }

    pub fn move_down(mut self) -> Game<'a> {
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

    fn make_first_valid_move<I>(&mut self, mut moves: I)
    where
        I: IntoIterator<Item = AreaId>,
    {
        for area_id in moves {
            debug!("Attempting to move selection to {:?}", area_id);
            if self.areas.move_selection(area_id) {
                break;
            }
        }
    }

    pub fn activate(mut self) -> Game<'a> {
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
    }
}
