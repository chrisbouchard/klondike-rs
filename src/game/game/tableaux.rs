use crate::game::card::Card;
use crate::game::game::area::{Area, AreaId, SelectionMode};
use crate::game::stack::{Stack, StackDetails, StackSelection};


#[derive(Debug)]
pub struct Tableaux {
    index: usize,
    cards: Vec<Card>,
    revealed_len: usize,
}

impl Tableaux {
    pub fn new(index: usize, cards: Vec<Card>) -> Tableaux {
        let revealed_index = cards.iter()
            .position(|card| card.face_up)
            .unwrap_or_default();
        let revealed_len = cards.len() - revealed_index;

        Tableaux { index, cards, revealed_len }
    }
}

impl Area for Tableaux {
    fn id(&self) -> AreaId {
        AreaId::Tableaux(self.index)
    }

    fn accepts_focus(&self, mode: &SelectionMode) -> bool {
        match mode {
            SelectionMode::Held(held) =>
                if let Some(card) = held.cards.first() {
                    match self.cards.last() {
                        Some(tableaux_card) =>
                            tableaux_card.face_up
                                && tableaux_card.rank.is_followed_by(card.rank)
                                && tableaux_card.color() != card.color(),
                        None =>
                            card.rank.is_king(),
                    }
                } else {
                    false
                },
            SelectionMode::Cards(len) => *len <= self.revealed_len
        }
    }

    fn as_stack(&self, mode: Option<&SelectionMode>) -> Stack {
        Stack::new(
            &self.cards,
            StackDetails {
                len: self.cards.len(),
                visible_len: self.cards.len(),
                spread_len: self.revealed_len,
                selection: mode.map(|mode| {
                    match mode {
                        SelectionMode::Held(held) => {
                            StackSelection::Stack(held.cards.len())
                        }
                        SelectionMode::Cards(len) => {
                            StackSelection::Cards(*len)
                        }
                    }
                }),
            },
        )
    }
}
