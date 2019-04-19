use crate::{
    model::{
        card::Card,
        settings::Settings,
        stack::{Stack, StackDetails, StackSelection},
    },
    utils::vec::SplitOffBounded,
};

use super::{Action, Area, AreaId, Held, SelectedArea, UnselectedArea};

#[derive(Copy, Clone, Debug)]
pub struct Selection {
    held: bool,
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
    pub fn new(cards: Vec<Card>, fanned_len: usize, settings: &Settings) -> UnselectedTalon {
        Talon {
            cards,
            fanned_len,
            settings,
            selection: (),
        }
    }

    fn as_stack(&self, mode: Option<Selection>) -> Stack {
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

impl<'a> Area for UnselectedTalon<'a> {
    fn id(&self) -> AreaId {
        AreaId::Talon
    }

    fn as_stack(&self) -> Stack {
        self.as_stack(None)
    }
}

impl<'a> Area for SelectedTalon<'a> {
    fn id(&self) -> AreaId {
        AreaId::Talon
    }

    fn as_stack(&self) -> Stack {
        self.as_stack(Some(self.selection))
    }
}

impl<'a> UnselectedArea for UnselectedTalon<'a> {
    fn select(self: Box<Self>) -> Result<Box<dyn SelectedArea>, Box<dyn UnselectedArea>> {
        if !self.cards.is_empty() {
            Ok(Box::new(Talon {
                cards: self.cards,
                fanned_len: self.fanned_len,
                settings: self.settings,
                selection: Selection { held: false },
            }))
        } else {
            Err(self)
        }
    }

    fn select_with_held(
        self: Box<Self>,
        mut held: Held,
    ) -> Result<Box<dyn SelectedArea>, (Box<dyn UnselectedArea>, Held)> {
        if self.id() == held.source {
            self.fanned_len += held.cards.len();
            self.cards.append(&mut held.cards);
            Ok(Box::new(Talon {
                cards: self.cards,
                fanned_len: self.fanned_len,
                settings: self.settings,
                selection: Selection { held: true },
            }))
        } else {
            Err((self, held))
        }
    }

    fn as_area(&self) -> &dyn Area {
        self
    }
}

impl<'a> SelectedArea for SelectedTalon<'a> {
    fn deselect(self: Box<Self>) -> (Box<dyn UnselectedArea>, Option<Held>) {
        let held = if self.selection.held {
            let cards = self.cards.split_off_bounded(1);
            self.fanned_len -= 1;

            Some(Held {
                source: self.id(),
                cards,
            })
        } else {
            None
        };

        let unselected = Box::new(Talon {
            cards: self.cards,
            fanned_len: self.fanned_len,
            settings: self.settings,
            selection: (),
        });

        (unselected, held)
    }

    fn activate(&mut self) -> Option<Action> {
        self.selection.held = !self.selection.held;
        None
    }

    fn select_more(&mut self) {}
    fn select_less(&mut self) {}

    fn as_area(&self) -> &dyn Area {
        self
    }
}
