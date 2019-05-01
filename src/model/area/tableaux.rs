use crate::{
    model::{
        card::Card,
        settings::Settings,
        stack::{Orientation, Stack, StackDetails, StackSelection},
    },
    utils::vec::SplitOffBounded,
};

use super::{Action, Area, AreaId, Held, SelectedArea, UnselectedArea};

#[derive(Copy, Clone, Debug)]
pub struct Selection {
    held_from: Option<AreaId>,
    len: usize,
}

#[derive(Debug)]
pub struct Tableaux<'a, S> {
    index: u8,
    cards: Vec<Card>,
    revealed_len: usize,
    settings: &'a Settings,
    selection: S,
}

pub type UnselectedTableaux<'a> = Tableaux<'a, ()>;
pub type SelectedTableaux<'a> = Tableaux<'a, Selection>;

impl<'a, S> Tableaux<'a, S> {
    fn id(&self) -> AreaId {
        AreaId::Tableaux(self.index)
    }

    fn accepts_cards(&self, held: &Held) -> bool {
        if held.source == self.id() {
            true
        } else if let Some(card) = held.cards.first() {
            if let Some(tableaux_card) = self.cards.last() {
                // TODO: Check that the pile itself is legit.
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

    fn give_cards(&mut self, mut held: Held) -> Result<(), Held> {
        if self.accepts_cards(&held) {
            let held_len = held.cards.len();
            self.revealed_len += held_len;
            self.cards.append(&mut held.cards);
            Ok(())
        } else {
            Err(held)
        }
    }

    fn take_cards(&mut self, len: usize, source: AreaId) -> Held {
        let cards = self.cards.split_off_bounded(len);
        self.revealed_len -= cards.len();

        Held { source, cards }
    }

    fn as_stack(&self, mode: Option<Selection>) -> Stack {
        Stack {
            cards: &self.cards,
            details: StackDetails {
                orientation: Orientation::Vertical,
                len: self.cards.len(),
                face_up_len: self.revealed_len,
                visible_len: self.cards.len(),
                spread_len: self.revealed_len,
                selection: mode.map(|selection| StackSelection {
                    len: selection.len,
                    held: selection.held_from.is_some(),
                }),
            },
        }
    }

    fn with_selection<T>(self, selection: T) -> Tableaux<'a, T> {
        Tableaux {
            index: self.index,
            cards: self.cards,
            revealed_len: self.revealed_len,
            settings: self.settings,
            selection,
        }
    }
}

impl<'a> UnselectedTableaux<'a> {
    pub fn create(
        index: u8,
        revealed_len: usize,
        cards: Vec<Card>,
        settings: &'a Settings,
    ) -> Box<dyn UnselectedArea<'a> + 'a> {
        Box::new(Tableaux {
            index,
            cards,
            revealed_len,
            settings,
            selection: (),
        })
    }
}

impl<'a> Area<'a> for UnselectedTableaux<'a> {
    fn id(&self) -> AreaId {
        Tableaux::id(self)
    }

    fn give_cards(&mut self, held: Held) -> Result<(), Held> {
        Tableaux::give_cards(self, held)
    }

    fn take_cards(&mut self, len: usize) -> Held {
        self.take_cards(len, self.id())
    }

    fn take_all_cards(&mut self) -> Held {
        self.take_cards(self.cards.len(), self.id())
    }

    fn peek_top_card(&self) -> Option<&Card> {
        self.cards.first()
    }

    fn as_stack(&self) -> Stack {
        self.as_stack(None)
    }
}

impl<'a> Area<'a> for SelectedTableaux<'a> {
    fn id(&self) -> AreaId {
        Tableaux::id(self)
    }

    fn give_cards(&mut self, held: Held) -> Result<(), Held> {
        self.selection.held_from = None;
        self.selection.len = 1;

        Tableaux::give_cards(self, held)
    }

    fn take_cards(&mut self, len: usize) -> Held {
        let source = self.selection.held_from.take().unwrap_or_else(|| self.id());
        self.selection.len = 1;

        self.take_cards(len, source)
    }

    fn take_all_cards(&mut self) -> Held {
        let source = self.selection.held_from.take().unwrap_or_else(|| self.id());
        self.selection.len = 1;

        self.take_cards(self.cards.len(), source)
    }

    fn peek_top_card(&self) -> Option<&Card> {
        self.cards.first()
    }

    fn as_stack(&self) -> Stack {
        self.as_stack(Some(self.selection))
    }
}

impl<'a> UnselectedArea<'a> for UnselectedTableaux<'a> {
    fn select<'b>(
        self: Box<Self>,
    ) -> Result<Box<dyn SelectedArea<'a> + 'b>, Box<dyn UnselectedArea<'a> + 'b>>
    where
        'a: 'b,
    {
        if !self.cards.is_empty() {
            Ok(Box::new(self.with_selection(Selection {
                held_from: None,
                len: 1,
            })))
        } else {
            Err(self)
        }
    }

    fn select_with_held<'b>(
        mut self: Box<Self>,
        held: Held,
    ) -> Result<Box<dyn SelectedArea<'a> + 'b>, (Box<dyn UnselectedArea<'a> + 'b>, Held)>
    where
        'a: 'b,
    {
        let source = held.source;
        let len = held.cards.len();

        match self.give_cards(held) {
            Ok(()) => Ok(Box::new(self.with_selection(Selection {
                held_from: Some(source),
                len,
            }))),
            Err(held) => Err((self, held)),
        }
    }

    fn as_area<'b>(&'b self) -> &'b dyn Area<'a>
    where
        'a: 'b,
    {
        self
    }

    fn as_area_mut<'b>(&'b mut self) -> &'b mut Area<'a>
    where
        'a: 'b,
    {
        self
    }
}

impl<'a> SelectedArea<'a> for SelectedTableaux<'a> {
    fn deselect<'b>(mut self: Box<Self>) -> (Box<dyn UnselectedArea<'a> + 'b>, Option<Held>)
    where
        'a: 'b,
    {
        let held = if let Some(source) = self.selection.held_from {
            Some(self.take_cards(self.selection.len, source))
        } else {
            None
        };

        let unselected = Box::new(self.with_selection(()));

        (unselected, held)
    }

    fn activate(&mut self) -> Option<Action> {
        if self.revealed_len > 0 {
            if self.selection.held_from.is_some() {
                self.selection.held_from = None;
            } else {
                self.selection.held_from = Some(self.id());
            }
        } else if !self.cards.is_empty() {
            self.revealed_len += 1;
        }

        None
    }

    fn select_more(&mut self) {
        if self.selection.len < self.revealed_len {
            self.selection.len += 1;
        }
    }

    fn select_less(&mut self) {
        if self.selection.len > 1 {
            self.selection.len -= 1;
        }
    }

    fn as_area<'b>(&'b self) -> &'b dyn Area<'a>
    where
        'a: 'b,
    {
        self
    }

    fn as_area_mut<'b>(&'b mut self) -> &'b mut Area<'a>
    where
        'a: 'b,
    {
        self
    }
}
