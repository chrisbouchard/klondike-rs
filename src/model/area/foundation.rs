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
pub struct Foundation<'a> {
    index: usize,
    cards: Vec<Card>,

    settings: &'a Settings,
}

impl<'a> Foundation<'a> {
    pub fn new(index: usize, cards: Vec<Card>, settings: &Settings) -> Foundation {
        Foundation {
            index,
            cards,
            settings,
        }
    }
}

impl<'a> Area for Foundation<'a> {
    fn id(&self) -> AreaId {
        AreaId::Foundation(self.index)
    }

    fn accepts_cards(&self, cards: &Vec<Card>) -> bool {
        if let Some((card, &[])) = cards.split_first() {
            if let Some(foundation_card) = self.cards.last() {
                self.index == card.suit.index() && foundation_card.rank.is_followed_by(card.rank)
            } else {
                self.index == card.suit.index() && card.rank.is_ace()
            }
        } else {
            false
        }
    }

    fn accepts_selection(&self, mode: &SelectionMode) -> bool {
        mode.is_held()
            || (self.settings.take_from_foundation && mode.len() == 1 && !self.cards.is_empty())
    }

    fn place_cards(&mut self, mut cards: Vec<Card>) {
        self.cards.append(&mut cards);
    }

    fn take_cards(&mut self, len: usize) -> Vec<Card> {
        self.cards.split_off_bounded(len)
    }

    fn as_stack<'s>(&'s self, mode: Option<&'s SelectionMode>) -> Stack<'s> {
        let cards_len = self.cards.len();

        Stack {
            cards: &self.cards,
            details: StackDetails {
                len: cards_len,
                face_up_len: cards_len,
                visible_len: 2,
                spread_len: 1,
                selection: mode.map(|_| StackSelection::Cards(1)),
            },
        }
    }
}
