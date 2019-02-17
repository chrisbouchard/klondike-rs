use crate::model::{
    card::Card,
    stack::{Stack, StackDetails, StackSelection},
};

use super::{
    area::{Action, Area, AreaId, Held, SelectionMode},
    settings::KlondikeGameSettings,
};

#[derive(Debug)]
pub struct Tableaux<'a> {
    index: usize,
    cards: Vec<Card>,
    revealed_len: usize,

    settings: &'a KlondikeGameSettings,
}

impl<'a> Tableaux<'a> {
    pub fn new(index: usize, cards: Vec<Card>, settings: &KlondikeGameSettings) -> Tableaux {
        let revealed_index = cards
            .iter()
            .position(|card| card.face_up)
            .unwrap_or_default();
        let revealed_len = cards.len() - revealed_index;

        Tableaux {
            index,
            cards,
            revealed_len,
            settings,
        }
    }
}

impl<'a> Area for Tableaux<'a> {
    fn id(&self) -> AreaId {
        AreaId::Tableaux(self.index)
    }

    fn accepts_focus(&self, mode: &SelectionMode) -> bool {
        match mode {
            SelectionMode::Held(held) => {
                if held.source == self.id() {
                    true
                } else if let Some(card) = held.cards.first() {
                    match self.cards.last() {
                        Some(tableaux_card) => {
                            tableaux_card.face_up
                                && card.rank.is_followed_by(tableaux_card.rank)
                                && card.color() != tableaux_card.color()
                        }
                        None => card.rank.is_king(),
                    }
                } else {
                    false
                }
            }
            SelectionMode::Cards(len) => {
                (*len <= self.revealed_len)
                    || (*len == 1 && self.revealed_len == 0 && !self.cards.is_empty())
            }
        }
    }

    fn activate(&mut self, mode: &mut SelectionMode) -> Option<Action> {
        debug_assert!(self.accepts_focus(mode));

        match mode {
            SelectionMode::Cards(len) => {
                if self.revealed_len > 0 {
                    self.revealed_len -= *len;

                    let cards = self.cards.split_off(self.cards.len() - *len);
                    let held = Held {
                        source: self.id(),
                        cards,
                    };
                    *mode = SelectionMode::Held(held);

                    None
                } else {
                    if let Some(card) = self.cards.last_mut() {
                        self.revealed_len = 1;
                        card.face_up = true;
                    }

                    None
                }
            }
            SelectionMode::Held(held) => {
                let len = held.cards.len();
                self.revealed_len += len;
                self.cards.append(&mut held.cards);

                let source = held.source;
                *mode = SelectionMode::new();

                Some(Action::MoveTo(source))
            }
        }
    }

    fn as_stack<'s>(&'s self, mode: Option<&'s SelectionMode>) -> Stack<'s> {
        let base_stack = Stack::new(
            &self.cards,
            StackDetails {
                len: self.cards.len(),
                visible_len: self.cards.len(),
                spread_len: self.revealed_len,
                selection: mode.map(|mode| match mode {
                    SelectionMode::Held(held) => StackSelection::Stack(held.cards.len()),
                    SelectionMode::Cards(len) => StackSelection::Cards(*len),
                }),
            },
        );

        if let Some(SelectionMode::Held(held)) = mode {
            base_stack.with_floating_cards_spread(&held.cards)
        } else {
            base_stack
        }
    }
}
