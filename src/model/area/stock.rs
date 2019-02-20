use crate::{
    model::{
        card::Card,
        settings::Settings,
        stack::{Stack, StackDetails, StackSelection},
    },
    utils::vec::SplitOffBounded,
};

use super::{Action, Area, AreaId, SelectionMode};

#[derive(Debug)]
pub struct Stock<'a> {
    cards: Vec<Card>,
    settings: &'a Settings,
}

impl<'a> Stock<'a> {
    pub fn new(cards: Vec<Card>, settings: &Settings) -> Stock {
        Stock { cards, settings }
    }
}

impl<'a> Area for Stock<'a> {
    fn id(&self) -> AreaId {
        AreaId::Stock
    }

    fn accepts_cards(&self, cards: &Vec<Card>) -> bool {
        false
    }

    fn accepts_selection(&self, mode: &SelectionMode) -> bool {
        mode.is_free() && mode.len() == 1
    }

    fn place_cards(&mut self, mut cards: Vec<Card>) {
        self.cards.append(&mut cards)
    }

    fn take_cards(&mut self, len: usize) -> Vec<Card> {
        self.cards.split_off_bounded(len)
    }

    fn activate(&self, mode: &SelectionMode) -> Action {
        if self.cards.is_empty() {
            Action::Draw
        } else {
            Action::Refresh
        }
    }

    fn as_stack<'s>(&'s self, mode: Option<&'s SelectionMode>) -> Stack<'s> {
        Stack {
            cards: &self.cards,
            details: StackDetails {
                len: self.cards.len(),
                face_up_len: 0,
                visible_len: 2,
                spread_len: 1,
                selection: mode.map(|_| StackSelection::Cards(1)),
            },
        }
    }
}
