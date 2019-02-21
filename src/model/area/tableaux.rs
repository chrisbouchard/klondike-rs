use crate::{
    model::{
        card::Card,
        settings::Settings,
        stack::{Stack, StackDetails, StackSelection},
    },
    utils::{usize::BoundedSub, vec::SplitOffBounded},
};

use super::{Action, Area, AreaId, SelectionMode};

#[derive(Debug)]
pub struct Tableaux<'a> {
    index: usize,
    cards: Vec<Card>,
    revealed_len: usize,

    settings: &'a Settings,
}

impl<'a> Tableaux<'a> {
    pub fn new(index: usize, revealed_len: usize, cards: Vec<Card>, settings: &Settings) -> Tableaux {
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

    fn accepts_cards(&self, cards: &Vec<Card>) -> bool {
        if let Some(card) = cards.first() {
            if let Some(tableaux_card) = self.cards.last() {
                self.revealed_len > 0
                    && card.rank.is_followed_by(tableaux_card.rank)
                    && card.color() != tableaux_card.color()
            } else {
                card.rank.is_king()
            }
        } else {
            false
        }
    }

    fn accepts_selection(&self, mode: &SelectionMode) -> bool {
        let len = mode.len();

        if self.revealed_len > 0 {
            mode.is_held() || (len > 0 && len <= self.revealed_len)
        } else {
            mode.is_free() && len == 1 && !self.cards.is_empty()
        }
    }

    fn place_cards(&mut self, mut cards: Vec<Card>) {
        self.revealed_len += cards.len();
        self.cards.append(&mut cards);
    }

    fn take_cards(&mut self, len: usize) -> Vec<Card> {
        let cards = self.cards.split_off_bounded(len);
        self.revealed_len.bounded_sub(cards.len());
        cards
    }

    fn activate(&self, mode: &SelectionMode) -> Action {
        if self.revealed_len > 0 {
            Action::Default
        } else {
            Action::FlipOver
        }
    }

    fn as_stack<'s>(&'s self, mode: Option<&'s SelectionMode>) -> Stack<'s> {
        Stack {
            cards: &self.cards,
            details: StackDetails {
                len: self.cards.len(),
                face_up_len: self.revealed_len,
                visible_len: self.cards.len(),
                spread_len: self.revealed_len,
                selection: mode.map(selection_to_stack_selection),
            },
        }
    }
}

fn selection_to_stack_selection(mode: &SelectionMode) -> StackSelection {
    match mode {
        SelectionMode::Held { len, .. } => StackSelection::Stack(*len),
        SelectionMode::Free { len, .. } => StackSelection::Cards(*len),
    }
}
