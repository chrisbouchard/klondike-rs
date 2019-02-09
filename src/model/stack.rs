use std::iter::Chain;
use std::slice::Iter;

use crate::utils::usize::BoundedSub;

use super::card::Card;

#[derive(Copy, Clone, Debug)]
pub enum StackSelection {
    Cards(usize),
    Stack(usize),
    FullStack,
}

#[derive(Copy, Clone, Debug)]
pub struct StackDetails {
    pub len: usize,
    pub visible_len: usize,
    pub spread_len: usize,
    pub selection: Option<StackSelection>,
}

impl StackDetails {
    pub fn visible_index(&self) -> usize {
        self.len.bounded_sub(self.visible_len)
    }

    pub fn spread_index(&self) -> usize {
        self.len.bounded_sub(self.spread_len)
    }

    pub fn selection_index(&self) -> Option<usize> {
        self.selection.and_then(|selection|
            match selection {
                StackSelection::Cards(len) => Some(self.len.bounded_sub(len)),
                StackSelection::Stack(len) => Some(self.len.bounded_sub(len)),
                StackSelection::FullStack => None,
            }
        )
    }
}


pub struct Stack<'a> {
    fixed_cards: &'a [Card],
    floating_cards: &'a [Card],
    details: StackDetails,
}

impl<'a> Stack<'a> {
    pub fn new(fixed_cards: &[Card], details: StackDetails) -> Stack {
        Stack {
            fixed_cards,
            floating_cards: &[],
            details,
        }
    }

    pub fn with_floating_cards(self, floating_cards: &'a [Card]) -> Stack<'a> {
        Stack {
            fixed_cards: self.fixed_cards,
            floating_cards,
            details: StackDetails {
                len: self.details.len + floating_cards.len(),
                visible_len: self.details.visible_len,
                spread_len: self.details.spread_len,
                selection: self.details.selection,
            },
        }
    }

    pub fn with_floating_cards_spread(self, floating_cards: &'a [Card]) -> Stack<'a> {
        Stack {
            fixed_cards: self.fixed_cards,
            floating_cards,
            details: StackDetails {
                len: self.details.len + floating_cards.len(),
                visible_len: self.details.visible_len + floating_cards.len(),
                spread_len: self.details.spread_len + floating_cards.len(),
                selection: self.details.selection.map(|selection| {
                    match selection {
                        StackSelection::Cards(_) => StackSelection::Cards(floating_cards.len()),
                        StackSelection::Stack(_) => StackSelection::Stack(floating_cards.len()),
                        StackSelection::FullStack => StackSelection::FullStack,
                    }
                }),
            },
        }
    }

    pub fn details(&self) -> &StackDetails {
        &self.details
    }
}

impl<'a, 'b> IntoIterator for &'b Stack<'a> {
    type Item = &'a Card;
    type IntoIter = Chain<Iter<'a, Card>, Iter<'a, Card>>;

    fn into_iter(self) -> Self::IntoIter {
        self.fixed_cards.iter()
            .chain(self.floating_cards.iter())
    }
}

