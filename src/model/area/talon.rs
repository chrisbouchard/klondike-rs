use crate::{
    model::{
        card::Card,
        settings::Settings,
        stack::{Stack, StackDetails, StackSelection},
    },
    utils::{usize::BoundedSub, vec::SplitOffBounded},
};

use super::{Area, AreaId, SelectionMode};

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
}

impl<'a> Area for Talon<'a> {
    fn id(&self) -> AreaId {
        AreaId::Talon
    }

    fn accepts_cards(&self, cards: &Vec<Card>) -> bool {
        false
    }

    fn accepts_selection(&self, mode: &SelectionMode) -> bool {
        mode.is_held() || (mode.len() == 1 && !self.cards.is_empty())
    }

    fn place_cards(&mut self, mut cards: Vec<Card>) {
        self.fanned_len = cards.len();
        self.cards.append(&mut cards);
    }

    fn replace_cards(&mut self, mut cards: Vec<Card>) {
        self.fanned_len += cards.len();
        self.cards.append(&mut cards);
    }

    fn take_cards(&mut self, len: usize) -> Vec<Card> {
        let cards = self.cards.split_off_bounded(len);
        self.fanned_len.bounded_sub_with_min(cards.len(), 1);
        cards
    }

    fn as_stack<'s>(&'s self, mode: Option<&'s SelectionMode>) -> Stack<'s> {
        let cards_len = self.cards.len();

        Stack {
            cards: &self.cards,
            details: StackDetails {
                len: cards_len,
                face_up_len: cards_len,
                visible_len: self.fanned_len + 1,
                spread_len: self.fanned_len,
                selection: mode.map(|_| StackSelection::Cards(1)),
            },
        }
    }
}
