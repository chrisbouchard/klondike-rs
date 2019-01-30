use std::ops::Range;

use crate::game::card::*;
use crate::game::deck::*;

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
    stock: FocusBox<Stock>,
    talon: FocusBox<Talon>,
    foundation: Vec<FocusBox<Foundation>>,
    tableaux: Vec<FocusBox<Tableaux>>,

    areas: Vec<AreaId>,
}

impl KlondikeGame {
    pub fn new(deck: &mut Deck) -> KlondikeGame {
        let foundation_indices: Range<usize> = 0..3;
        let tableaux_indices: Range<usize> = 0..7;

        let areas =
            [AreaId::Stock, AreaId::Talon].iter()
                .chain(foundation_indices.map(AreaId::Foundation))
                .chain(tableaux_indices.map(AreaId::Tableaux))
                .cloned()
                .collect::<Vec<_>>();

        let tableaux_cards =
            tableaux_indices.map(|index| {
                deck.deal(index).into_iter()
                    .chain(deck.deal_one().map(Card::face_up))
                    .collect::<Vec<_>>()
            }).collect::<Vec<_>>();

        // TODO: Start with an empty talon.
        let talon_cards =
            deck.deal(3).into_iter()
                .map(Card::face_up)
                .collect();
        let stock_cards = deck.deal_rest();

        KlondikeGame {
            stock: FocusBox::Unfocused(Stock::new(stock_cards)),
            talon: FocusBox::Unfocused(Talon::new(talon_cards, 3)),
            foundation: foundation_indices.map(|index: usize| {
                FocusBox::Unfocused(Foundation::new(0, Vec::new()))
            }).collect(),
            tableaux: tableaux_cards.iter().enumerate().map(|(index, cards)| {
                FocusBox::Unfocused(Tableaux::new(index, cards))
            }),

            areas
        }
    }


    pub fn area(&self, area_id: AreaId) -> &Area {
        // TODO: Prevent panic! on indexing?
        match area_id {
            AreaId::Stock => &self.stock,
            AreaId::Talon => &self.talon,
            AreaId::Foundation(index) => &self.foundation[index],
            AreaId::Tableaux(index) => &self.tableaux[index]
        }
    }

    pub fn with_focused_area<F>(&self, visitor: F)
        where F: FnOnce(&FocusedArea) {
        for area_id in self.areas {
            match area_id {
                AreaId::Stock => {
                   if let Some(area) = self.stock.if_focused() {
                       visitor(area);
                       break;
                   }
                },
                AreaId::Talon => {
                    if let Some(area) = self.talon.if_focused() {
                        visitor(area);
                        break;
                    }
                },
                AreaId::Foundation(index) => {
                    if let Some(area) = self.foundation[index].if_focused() {
                        visitor(area);
                        break;
                    }
                },
                AreaId::Tableaux(index) => {
                    if let Some(area) = self.tableaux[index].if_focused() {
                        visitor(area);
                        break;
                    }
                }
            }
        }
    }


    /*
    pub fn move_to_stock(&mut self) {
        if let Some(selection) = self.first_valid_move(once(GameSelection::Stock)) {
            self.selection = selection;
        }
    }

    pub fn move_to_talon(&mut self) {
        if let Some(selection) = self.first_valid_move(once(GameSelection::Talon)) {
            self.selection = selection;
        }
    }

    pub fn move_to_tableaux(&mut self, index: usize) {
        let moves_iter =
            once(GameSelection::Tableaux(index, TableauxSelection::Cards(1)));

        if let Some(selection) = self.first_valid_move(moves_iter) {
            self.selection = selection;
        }
    }

    pub fn move_left(&mut self) {
        let moves_iter =
            successors(Some(self.selection), |selection| {
                self.next_selection_left(*selection)
            }).skip(1);  // Skip first result because it's just self.selection.

        if let Some(selection) = self.first_valid_move(moves_iter) {
            self.selection = selection;
        }
    }

    pub fn move_right(&mut self) {
        let moves_iter =
            successors(Some(self.selection), |selection| {
                self.next_selection_right(*selection)
            }).skip(1);  // Skip first result because it's just self.selection.

        if let Some(selection) = self.first_valid_move(moves_iter) {
            self.selection = selection;
        }
    }

    pub fn move_up(&mut self) {
        if let GameSelection::Tableaux(index, TableauxSelection::Cards(len)) = self.selection {
            let moves_iter =
                once(GameSelection::Tableaux(index, TableauxSelection::Cards(len + 1)));

            if let Some(selection) = self.first_valid_move(moves_iter) {
                self.selection = selection;
            }
        }
    }

    pub fn move_down(&mut self) {
        if let GameSelection::Tableaux(index, TableauxSelection::Cards(len)) = self.selection {
            let moves_iter =
                once(GameSelection::Tableaux(index, TableauxSelection::Cards(len - 1)));

            if let Some(selection) = self.first_valid_move(moves_iter) {
                self.selection = selection;
            }
        }
    }


    fn first_valid_move(&self, moves_iter: impl Iterator<Item=GameSelection>) -> Option<GameSelection> {
        moves_iter.filter(|selection| {
            debug!("first_valid_move: {:?}", selection);
            self.is_valid_selection(*selection)
        }).next()
    }

    fn next_selection_left(&self, selection: GameSelection) -> Option<GameSelection> {
        match selection {
            GameSelection::Stock => None,
            GameSelection::Talon => Some(GameSelection::Stock),
            GameSelection::Foundation(0) => Some(GameSelection::Talon),
            GameSelection::Foundation(index) => Some(GameSelection::Foundation(index - 1)),
            GameSelection::Tableaux(0, _) => Some(GameSelection::Foundation(3)),
            GameSelection::Tableaux(index, tableaux_selection) =>
                Some(GameSelection::Tableaux(index - 1, tableaux_selection)),
        }
    }

    fn next_selection_right(&self, selection: GameSelection) -> Option<GameSelection> {
        match selection {
            GameSelection::Stock => Some(GameSelection::Talon),
            GameSelection::Talon => Some(GameSelection::Foundation(0)),
            GameSelection::Foundation(index) if index < 3 =>
                Some(GameSelection::Foundation(index + 1)),
            GameSelection::Foundation(_) =>
                Some(GameSelection::Tableaux(0, TableauxSelection::Cards(1))),
            GameSelection::Tableaux(index, tableaux_selection) if index < 6 =>
                Some(GameSelection::Tableaux(index + 1, tableaux_selection)),
            GameSelection::Tableaux(_, _) => None
        }
    }

    fn is_valid_selection(&self, selection: GameSelection) -> bool {
        if let Some(ref holding) = self.holding {
            match selection {
                GameSelection::Stock => false,
                GameSelection::Talon => holding.source == GameArea::Talon,
                GameSelection::Foundation(index) =>
                    if let Some((card, &[])) = holding.cards.split_first() {
                        match self.foundation[index].last() {
                            Some(foundation_card) =>
                                index == card.suit.index()
                                    && card.rank.is_followed_by(foundation_card.rank),
                            None =>
                                index == card.suit.index()
                                    && card.rank.is_ace()
                        }
                    } else {
                        false
                    },
                GameSelection::Tableaux(index, _) =>
                    if let Some(card) = holding.cards.first() {
                        match self.tableaux[index].cards.last() {
                            Some(tableaux_card) =>
                                tableaux_card.face_up
                                    && tableaux_card.rank.is_followed_by(card.rank)
                                    && tableaux_card.color() != card.color(),
                            None =>
                                card.rank.is_king()
                        }
                    } else {
                        false
                    },
            }
        } else {
            match selection {
                GameSelection::Stock => true,
                GameSelection::Talon => !self.talon.is_empty(),
                GameSelection::Foundation(index) => !self.foundation[index].is_empty(),
                GameSelection::Tableaux(index, tableaux_spread) => {
                    let spread = &self.tableaux[index];
                    match tableaux_spread {
                        TableauxSelection::Cards(len) =>
                            !spread.cards.is_empty()
                                && 0 < len
                                && len <= spread.revealed_len,
                        TableauxSelection::Stack => !spread.cards.is_empty(),
                    }
                }
            }
        }
    }
    */
}
