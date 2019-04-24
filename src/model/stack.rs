use std::slice::Iter;

use crate::utils::usize::BoundedSub;

use super::card::Card;

#[derive(Copy, Clone, Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, Debug)]
pub enum StackSelection {
    Cards(usize),
    Stack(usize),
    FullStack,
}

#[derive(Copy, Clone, Debug)]
pub struct StackDetails {
    pub orientation: Orientation,
    pub len: usize,
    pub face_up_len: usize,
    pub visible_len: usize,
    pub spread_len: usize,
    pub selection: Option<StackSelection>,
}

impl StackDetails {
    pub fn face_up_index(&self) -> usize {
        self.len.bounded_sub(self.face_up_len)
    }

    pub fn visible_index(&self) -> usize {
        self.len.bounded_sub(self.visible_len)
    }

    pub fn spread_index(&self) -> usize {
        self.len.bounded_sub(self.spread_len)
    }

    pub fn selection_index(&self) -> Option<usize> {
        self.selection.and_then(|selection| match selection {
            StackSelection::Cards(len) => Some(self.len.bounded_sub(len)),
            StackSelection::Stack(len) => Some(self.len.bounded_sub(len)),
            StackSelection::FullStack => None,
        })
    }
}

#[derive(Debug)]
pub struct Stack<'a> {
    pub cards: &'a [Card],
    pub details: StackDetails,
}

impl<'a, 'b> IntoIterator for &'b Stack<'a> {
    type Item = &'a Card;
    type IntoIter = Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.iter()
    }
}
