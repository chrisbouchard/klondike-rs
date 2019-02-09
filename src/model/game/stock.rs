use std::cmp::min;

use crate::model::card::Card;
use crate::model::stack::{Stack, StackDetails, StackSelection};

use super::area::{Action, Area, AreaId, SelectionMode};

#[derive(Debug)]
pub struct Stock {
    cards: Vec<Card>
}

impl Stock {
    pub fn new(cards: Vec<Card>) -> Stock {
        Stock { cards }
    }

    pub fn draw(&mut self, len: usize) -> Vec<Card> {
        let len = min(len, self.cards.len());

        if len > 0 {
            self.cards.split_off(self.cards.len() - len)
                .into_iter()
                .rev()
                .map(|card| card.face_up())
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn place(&mut self, mut cards: Vec<Card>) {
        self.cards.append(&mut cards);
    }
}

impl Area for Stock {
    fn id(&self) -> AreaId {
        AreaId::Stock
    }

    fn accepts_focus(&self, mode: &SelectionMode) -> bool {
        match mode {
            SelectionMode::Cards(len) => *len <= 1,
            _ => false,
        }
    }

    fn activate(&mut self, mode: &mut SelectionMode) -> Option<Action> {
        debug_assert!(self.accepts_focus(mode));

        match mode {
            SelectionMode::Cards(_) =>
                if self.cards.is_empty() {
                    Some(Action::Restock)
                } else {
                    Some(Action::Draw)
                },
            _ => None,
        }
    }

    fn as_stack<'a>(&'a self, mode: Option<&'a SelectionMode>) -> Stack<'a> {
        Stack::new(
            &self.cards,
            StackDetails {
                len: self.cards.len(),
                visible_len: 2,
                spread_len: 1,
                selection: mode.map(|_| StackSelection::Cards(1)),
            },
        )
    }
}
