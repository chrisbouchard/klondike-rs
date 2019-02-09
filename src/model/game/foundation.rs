use crate::model::{
    card::Card,
    stack::{Stack, StackDetails, StackSelection},
};

use super::area::{Action, Area, AreaId, Held, SelectionMode};

#[derive(Debug)]
pub struct Foundation {
    index: usize,
    cards: Vec<Card>,
}

impl Foundation {
    pub fn new(index: usize, cards: Vec<Card>) -> Foundation {
        Foundation { index, cards }
    }
}

impl Area for Foundation {
    fn id(&self) -> AreaId {
        AreaId::Foundation(self.index)
    }

    fn accepts_focus(&self, mode: &SelectionMode) -> bool {
        match mode {
            SelectionMode::Cards(len) => *len == 1 && !self.cards.is_empty(),
            SelectionMode::Held(held) => {
                if held.source == self.id() {
                    true
                } else if let Some((card, &[])) = held.cards.split_first() {
                    if let Some(foundation_card) = self.cards.last() {
                        self.index == card.suit.index()
                            && foundation_card.rank.is_followed_by(card.rank)
                    } else {
                        self.index == card.suit.index() && card.rank.is_ace()
                    }
                } else {
                    false
                }
            }
        }
    }

    fn activate(&mut self, mode: &mut SelectionMode) -> Option<Action> {
        debug_assert!(self.accepts_focus(mode));

        match mode {
            SelectionMode::Cards(_) => {
                let cards = self.cards.split_off(self.cards.len() - 1);
                let held = Held {
                    source: self.id(),
                    cards,
                };
                *mode = SelectionMode::Held(held);

                None
            }
            SelectionMode::Held(held) => {
                self.cards.append(&mut held.cards);
                *mode = SelectionMode::Cards(1);

                None
            }
        }
    }

    fn as_stack<'a>(&'a self, mode: Option<&'a SelectionMode>) -> Stack<'a> {
        let base_stack = Stack::new(
            &self.cards,
            StackDetails {
                len: self.cards.len(),
                visible_len: 2,
                spread_len: 1,
                selection: mode.map(|_| StackSelection::Cards(1)),
            },
        );

        if let Some(SelectionMode::Held(held)) = mode {
            base_stack.with_floating_cards(&held.cards)
        } else {
            base_stack
        }
    }
}
