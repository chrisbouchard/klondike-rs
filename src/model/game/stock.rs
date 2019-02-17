use std::cmp::min;

use crate::model::{
    card::Card,
    stack::{Stack, StackDetails, StackSelection},
};

use super::{
    area::{Action, Area, AreaId, SelectionMode},
    settings::KlondikeGameSettings,
};

#[derive(Debug)]
pub struct Stock<'a> {
    cards: Vec<Card>,
    settings: &'a KlondikeGameSettings,
}

impl<'a> Stock<'a> {
    pub fn new(cards: Vec<Card>, settings: &KlondikeGameSettings) -> Stock {
        Stock { cards, settings }
    }

    pub fn draw(&mut self) -> Vec<Card> {
        let len = min(self.settings.draw_from_stock_len, self.cards.len());

        if len > 0 {
            self.cards
                .split_off(self.cards.len() - len)
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

impl<'a> Area for Stock<'a> {
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
            SelectionMode::Cards(_) => {
                if self.cards.is_empty() {
                    Some(Action::Restock)
                } else {
                    Some(Action::Draw)
                }
            }
            _ => None,
        }
    }

    fn as_stack<'s>(&'s self, mode: Option<&'s SelectionMode>) -> Stack<'s> {
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
