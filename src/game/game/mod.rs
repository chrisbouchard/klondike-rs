use std::iter::once;
use std::ops::Range;

use crate::game::card::*;
use crate::game::deck::*;
use crate::game::stack::*;

pub use self::area::*;
pub use self::foundation::*;
pub use self::stock::*;
pub use self::tableaux::*;
pub use self::talon::*;

pub mod area;
pub mod foundation;
pub mod stock;
pub mod tableaux;
pub mod talon;


#[derive(Debug)]
pub struct KlondikeGame {
    stock: Stock,
    talon: Talon,
    foundation: Vec<Foundation>,
    tableaux: Vec<Tableaux>,

    selection: Selection,

    areas: Vec<AreaId>,
}

impl KlondikeGame {
    pub fn new(deck: &mut Deck) -> KlondikeGame {
        let foundation_indices: Range<usize> = 0..3;
        let tableaux_indices: Range<usize> = 0..7;

        let areas =
            once(AreaId::Stock)
                .chain(once(AreaId::Talon))
                .chain(foundation_indices.clone().map(AreaId::Foundation))
                .chain(tableaux_indices.clone().map(AreaId::Tableaux))
                .collect::<Vec<_>>();

        let tableaux =
            tableaux_indices.map(|index| {
                let cards = deck.deal(index).into_iter()
                    .chain(deck.deal_one().map(Card::face_up))
                    .collect::<Vec<_>>();
                Tableaux::new(index, cards)
            }).collect::<Vec<_>>();

        let talon = Talon::new(Vec::new(), 0);

        let stock_cards = deck.deal_rest();
        let stock = Stock::new(stock_cards);

        let foundation =
            foundation_indices.map(|index| {
                Foundation::new(index, Vec::new())
            }).collect();

        let selection = Selection::new();

        KlondikeGame {
            stock,
            talon,
            foundation,
            tableaux,
            selection,
            areas,
        }
    }


    pub fn area(&self, area_id: AreaId) -> &Area {
        match area_id {
            AreaId::Stock => &self.stock,
            AreaId::Talon => &self.talon,
            AreaId::Foundation(index) => &self.foundation[index],
            AreaId::Tableaux(index) => &self.tableaux[index]
        }
    }

    pub fn stack(&self, area_id: AreaId) -> Stack {
        let mode =
            if self.selection.matches(area_id) {
                Some(&self.selection.mode)
            } else {
                None
            };

        self.area(area_id).as_stack(mode)
    }


    pub fn move_to_stock(mut self) -> KlondikeGame {
        let mode = self.selection.mode.moved_ref();
        let moves_iter = once(AreaId::Stock);

        if let Some(area_id) = self.first_valid_move(mode, moves_iter) {
            self.selection = self.selection.move_to(area_id);
        }

        self
    }

    pub fn move_to_talon(mut self) -> KlondikeGame {
        let mode = self.selection.mode.moved_ref();
        let moves_iter = once(AreaId::Talon);

        if let Some(area_id) = self.first_valid_move(mode, moves_iter) {
            self.selection = self.selection.move_to(area_id);
        }

        self
    }

    pub fn move_to_tableaux(mut self, index: usize) -> KlondikeGame {
        let mode = self.selection.mode.moved_ref();
        let moves_iter = once(AreaId::Tableaux(index));

        if let Some(area_id) = self.first_valid_move(mode, moves_iter) {
            self.selection = self.selection.move_to(area_id);
        }

        self
    }

    pub fn move_left(mut self) -> KlondikeGame {
        let mode = self.selection.mode.moved_ref();

        let starting_area_id = self.selection.target;
        let moves_iter =
            self.areas.iter().rev()
                .cycle()
                .skip_while(|area_id| **area_id != starting_area_id)
                .skip(1)
                .take_while(|area_id| **area_id != starting_area_id)
                .cloned();

        if let Some(area_id) = self.first_valid_move(mode, moves_iter) {
            self.selection = self.selection.move_to(area_id);
        }

        self
    }

    pub fn move_right(mut self) -> KlondikeGame {
        let mode = self.selection.mode.moved_ref();

        let starting_area_id = self.selection.target;
        let moves_iter =
            self.areas.iter()
                .cycle()
                .skip_while(|area_id| **area_id != starting_area_id)
                .skip(1)
                .take_while(|area_id| **area_id != starting_area_id)
                .cloned();

        if let Some(area_id) = self.first_valid_move(mode, moves_iter) {
            self.selection = self.selection.move_to(area_id);
        }

        self
    }

    pub fn move_up(mut self) -> KlondikeGame {
        if let SelectionMode::Cards(len) = self.selection.mode {
            let mode = SelectionMode::Cards(len + 1);
            let moves_iter = once(self.selection.target);

            if let Some(area_id) = self.first_valid_move(&mode, moves_iter) {
                self.selection = self.selection.move_to(area_id);
            }
        }

        self
    }

    pub fn move_down(mut self) -> KlondikeGame {
        if let SelectionMode::Cards(len) = self.selection.mode {
            if len > 0 {
                let mode = SelectionMode::Cards(len - 1);
                let moves_iter = once(self.selection.target);

                if let Some(area_id) = self.first_valid_move(&mode, moves_iter) {
                    self.selection = self.selection.move_to(area_id);
                }
            }
        }

        self
    }


    fn first_valid_move<I>(&self, mode: &SelectionMode, moves_iter: I) -> Option<AreaId>
        where I: Iterator<Item=AreaId> {
        moves_iter.filter(|area_id| {
            self.area(*area_id).accepts_focus(mode)
        }).next()
    }
}
