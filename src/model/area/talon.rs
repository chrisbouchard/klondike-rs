use crate::{
    model::{
        card::Card,
        settings::Settings,
        stack::{Orientation, Stack, StackDetails, StackSelection},
    },
    utils::{usize::BoundedSub, vec::SplitOffBounded},
};

use super::{Action, Area, AreaId, Held, SelectedArea, UnselectedArea};

#[derive(Copy, Clone, Debug)]
pub struct Selection {
    held_from: Option<AreaId>,
}

#[derive(Debug)]
pub struct Talon<'a, S> {
    cards: Vec<Card>,
    fanned_len: usize,
    settings: &'a Settings,
    selection: S,
}

pub type UnselectedTalon<'a> = Talon<'a, ()>;
pub type SelectedTalon<'a> = Talon<'a, Selection>;

impl<'a, S> Talon<'a, S> {
    fn give_cards(&mut self, mut held: Held) -> Result<(), Held> {
        if held.source == AreaId::Talon {
            self.fanned_len += held.cards.len();
            self.cards.append(&mut held.cards);
            Ok(())
        } else if held.source == AreaId::Stock {
            self.fanned_len = held.cards.len();
            self.cards.append(&mut held.cards);
            Ok(())
        } else {
            Err(held)
        }
    }

    fn take_cards(&mut self, len: usize, source: AreaId) -> Held {
        let cards = self.cards.split_off_bounded(len);
        self.fanned_len = self.fanned_len.bounded_sub(len);

        Held { source, cards }
    }

    fn as_stack(&self, mode: Option<Selection>) -> Stack {
        let cards_len = self.cards.len();

        Stack {
            cards: &self.cards,
            details: StackDetails {
                orientation: Orientation::Horizontal,
                len: cards_len,
                face_up_len: cards_len,
                visible_len: self.fanned_len + 1,
                spread_len: self.fanned_len,
                selection: mode.map(|_| StackSelection::Cards(1)),
            },
        }
    }

    fn with_selection<T>(self, selection: T) -> Talon<'a, T> {
        Talon {
            cards: self.cards,
            fanned_len: self.fanned_len,
            settings: self.settings,
            selection,
        }
    }
}

impl<'a> UnselectedTalon<'a> {
    pub fn create(
        cards: Vec<Card>,
        fanned_len: usize,
        settings: &'a Settings,
    ) -> Box<dyn UnselectedArea<'a> + 'a> {
        Box::new(Talon {
            cards,
            fanned_len,
            settings,
            selection: (),
        })
    }
}

impl<'a> Area<'a> for UnselectedTalon<'a> {
    fn id(&self) -> AreaId {
        AreaId::Talon
    }

    fn give_cards(&mut self, held: Held) -> Result<(), Held> {
        Talon::give_cards(self, held)
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

impl<'a> Area<'a> for SelectedTalon<'a> {
    fn id(&self) -> AreaId {
        AreaId::Talon
    }

    fn give_cards(&mut self, held: Held) -> Result<(), Held> {
        self.selection.held_from = None;
        Talon::give_cards(self, held)
    }

    fn take_cards(&mut self, len: usize) -> Held {
        let source = self.selection.held_from.take().unwrap_or_else(|| self.id());
        self.take_cards(len, source)
    }

    fn take_all_cards(&mut self) -> Held {
        let source = self.selection.held_from.take().unwrap_or_else(|| self.id());
        self.take_cards(self.cards.len(), source)
    }

    fn peek_top_card(&self) -> Option<&Card> {
        self.cards.first()
    }

    fn as_stack(&self) -> Stack {
        self.as_stack(Some(self.selection))
    }
}

impl<'a> UnselectedArea<'a> for UnselectedTalon<'a> {
    fn select<'b>(
        self: Box<Self>,
    ) -> Result<Box<dyn SelectedArea<'a> + 'b>, Box<dyn UnselectedArea<'a> + 'b>>
    where
        'a: 'b,
    {
        if !self.cards.is_empty() {
            Ok(Box::new(self.with_selection(Selection { held_from: None })))
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

        match self.give_cards(held) {
            Ok(()) => Ok(Box::new(self.with_selection(Selection {
                held_from: Some(source),
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

impl<'a> SelectedArea<'a> for SelectedTalon<'a> {
    fn deselect<'b>(mut self: Box<Self>) -> (Box<dyn UnselectedArea<'a> + 'b>, Option<Held>)
    where
        'a: 'b,
    {
        let held = if let Some(source) = self.selection.held_from {
            Some(self.take_cards(1, source))
        } else {
            None
        };

        let unselected = Box::new(self.with_selection(()));

        (unselected, held)
    }

    fn activate(&mut self) -> Option<Action> {
        if self.selection.held_from.is_some() {
            self.selection.held_from = None;
        } else {
            self.selection.held_from = Some(self.id());
        }

        None
    }

    fn select_more(&mut self) {}
    fn select_less(&mut self) {}

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
