use crate::game::card::Card;
use crate::game::game::area::{Area, AreaId, SelectionMode};
use crate::game::stack::{Stack, StackDetails, StackSelection};

#[derive(Debug)]
pub struct Stock {
    cards: Vec<Card>
}

impl Stock {
    pub fn new(cards: Vec<Card>) -> Stock {
        Stock { cards }
    }
}

impl Area for Stock {
    fn id(&self) -> AreaId {
        AreaId::Stock
    }

    fn accepts_focus(&self, mode: &SelectionMode) -> bool {
        match mode {
            SelectionMode::Cards(len) => *len == 1 && !self.cards.is_empty(),
            _ => false,
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
