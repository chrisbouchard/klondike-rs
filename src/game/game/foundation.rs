use crate::game::card::Card;
use crate::game::game::area::{Area, AreaId, SelectionMode};
use crate::game::stack::{Stack, StackDetails, StackSelection};

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
            SelectionMode::Held(held) =>
                if let Some((card, &[])) = held.cards.split_first() {
                    if let Some(foundation_card) = self.cards.last() {
                        self.index == card.suit.index()
                            && card.rank.is_followed_by(foundation_card.rank)
                    } else {
                        self.index == card.suit.index()
                            && card.rank.is_ace()
                    }
                } else {
                    false
                },
            SelectionMode::Cards(len) => *len == 1 && !self.cards.is_empty(),
        }
    }

    fn as_stack(&self, mode: Option<&SelectionMode>) -> Stack {
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
