use crate::game::card::Card;
use crate::game::game::area::{Area, AreaId, SelectionMode};
use crate::game::stack::{Stack, StackDetails, StackSelection};

#[derive(Debug)]
pub struct Talon {
    cards: Vec<Card>,
    fanned_len: usize
}

impl Talon {
    pub fn new(cards: Vec<Card>, fanned_len: usize) -> Talon {
        Talon { cards, fanned_len }
    }
}

impl Area for Talon {
    fn id(&self) -> AreaId {
        AreaId::Talon
    }

    fn accepts_focus(&self, mode: &SelectionMode) -> bool {
        match mode {
            SelectionMode::Held(held) => held.source == AreaId::Talon,
            SelectionMode::Cards(len) => *len == 1 && !self.cards.is_empty()
        }
    }

    fn as_stack(&self, mode: Option<&SelectionMode>) -> Stack {
        Stack::new(
            &self.cards,
            StackDetails {
                len: self.cards.len(),
                visible_len: self.fanned_len + 1,
                spread_len: self.fanned_len,
                selection: mode.map(|_| StackSelection::Cards(1)),
            },
        )
    }
}
