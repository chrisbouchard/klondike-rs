use crate::model::{
    card::Card,
    settings::Settings,
    stack::{Stack, StackDetails, StackSelection},
};

use super::{Action, Area, AreaId, Held, SelectionMode};

#[derive(Debug)]
pub struct Talon<'a> {
    cards: Vec<Card>,
    fanned_len: usize,

    settings: &'a Settings,
}

impl<'a> Talon<'a> {
    pub fn new(cards: Vec<Card>, fanned_len: usize, settings: &Settings) -> Talon {
        Talon {
            cards,
            fanned_len,
            settings,
        }
    }

    pub fn flip(&mut self) -> Vec<Card> {
        self.fanned_len = 1;
        self.cards
            .split_off(0)
            .into_iter()
            .rev()
            .map(|card| card.face_down())
            .collect()
    }

    pub fn place(&mut self, mut cards: Vec<Card>) {
        if !cards.is_empty() {
            self.fanned_len = cards.len();
            self.cards.append(&mut cards);
        }
    }
}

impl<'a> Area for Talon<'a> {
    fn id(&self) -> AreaId {
        AreaId::Talon
    }

    fn accepts_focus(&self, mode: &SelectionMode) -> bool {
        match mode {
            SelectionMode::Held(held) => held.source == AreaId::Talon,
            SelectionMode::Cards(len) => *len == 1 && !self.cards.is_empty(),
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

                if self.fanned_len > 1 {
                    self.fanned_len -= 1;
                }

                None
            }
            SelectionMode::Held(held) => {
                self.fanned_len += held.cards.len();
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
                visible_len: self.fanned_len + 1,
                spread_len: self.fanned_len,
                selection: mode.map(|_| StackSelection::Cards(1)),
            },
        );

        if let Some(SelectionMode::Held(held)) = mode {
            base_stack.with_floating_cards_spread(&held.cards)
        } else {
            base_stack
        }
    }
}
