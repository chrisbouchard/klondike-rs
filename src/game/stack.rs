use std::ops;

use crate::game::card::*;
use crate::utils;

#[derive(Copy, Clone, Debug)]
pub enum StackSelection {
    Cards(usize),
    Stack(usize),
    FullStack,
}

#[derive(Copy, Clone, Debug)]
pub struct Stack<'a> {
    pub cards: &'a [Card],
    pub visible_len: usize,
    pub spread_len: usize,
    pub selection: Option<StackSelection>,
}

impl<'a> Stack<'a> {
    pub fn visible_index(&self) -> usize {
        utils::index_of_last_n(self.visible_len, self.cards)
    }

    pub fn spread_index(&self) -> usize {
        utils::index_of_last_n(self.spread_len, self.cards)
    }

    pub fn selection_index(&self) -> Option<usize> {
        self.selection.and_then(|selection|
            match selection {
                StackSelection::Cards(len) => Some(utils::index_of_last_n(len, self.cards)),
                StackSelection::Stack(len) => Some(utils::index_of_last_n(len, self.cards)),
                StackSelection::FullStack => None,
            }
        )
    }
}

impl<'a> ops::Deref for Stack<'a> {
    type Target = [Card];

    fn deref(&self) -> &Self::Target {
        self.cards
    }
}

impl<'a> IntoIterator for &'a Stack<'a> {
    type Item = &'a Card;
    type IntoIter = ::std::slice::Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.iter()
    }
}
