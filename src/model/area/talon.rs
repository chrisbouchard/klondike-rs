use crate::{
    model::{
        card::Card,
        stack::{Orientation, Stack, StackDetails, StackSelection},
    },
    utils::vec::SplitOffBounded,
};

use super::{
    Action, Area, AreaId, Held, MoveResult, NotSupported, NothingToSelect, Result, SelectedArea,
    SnafuSelectorExt, UnselectedArea,
};

#[derive(Copy, Clone, Debug)]
pub struct Selection {
    held_from: Option<AreaId>,
}

#[derive(Debug)]
pub struct Talon<S> {
    cards: Vec<Card>,
    fanned_len: usize,
    selection: S,
}

pub type UnselectedTalon = Talon<()>;
pub type SelectedTalon = Talon<Selection>;

impl<S> Talon<S> {
    fn id(&self) -> AreaId {
        AreaId::Talon
    }

    fn validate_cards(&self, held: &Held) -> Result {
        if held.source == self.id() || held.source == AreaId::Stock {
            // We'll always take back our own cards, and we'll allow cards from the stock to be
            // replaced onto us.
            Ok(())
        } else {
            // But no cards from anywhere else.
            NotSupported {
                message: format!("Cannot place cards from area: {:?}", held.source),
            }
            .fail()
        }
    }

    fn give_cards(&mut self, mut held: Held) -> MoveResult<(), Held> {
        match self.validate_cards(&held) {
            Ok(_) => {
                if held.source == AreaId::Stock {
                    self.fanned_len = held.cards.len();
                } else {
                    self.fanned_len += held.cards.len();
                }

                self.cards.append(&mut held.cards);
                MoveResult::Moved(())
            }
            Err(error) => MoveResult::Unmoved(held, error),
        }
    }

    fn take_cards(&mut self, len: usize, source: AreaId) -> Held {
        let cards = self.cards.split_off_bounded(len);
        self.fanned_len = self.fanned_len.saturating_sub(cards.len());

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
                selection: mode.map(|selection| StackSelection {
                    len: 1,
                    held: selection.held_from.is_some(),
                }),
            },
        }
    }

    fn with_selection<T>(self, selection: T) -> Talon<T> {
        Talon {
            cards: self.cards,
            fanned_len: self.fanned_len,
            selection,
        }
    }
}

impl UnselectedTalon {
    pub fn create(cards: Vec<Card>, fanned_len: usize) -> Box<dyn UnselectedArea> {
        Box::new(Talon {
            cards,
            fanned_len,
            selection: (),
        })
    }
}

impl<'a> Area for UnselectedTalon {
    fn id(&self) -> AreaId {
        Talon::id(self)
    }

    fn is_selected(&self) -> bool {
        false
    }

    fn is_held(&self) -> bool {
        false
    }

    fn give_cards(&mut self, held: Held) -> MoveResult<(), Held> {
        Talon::give_cards(self, held)
    }

    fn take_cards(&mut self, len: usize) -> Held {
        self.take_cards(len, self.id())
    }

    fn take_all_cards(&mut self) -> Held {
        self.take_cards(self.cards.len(), self.id())
    }

    fn peek_top_card(&self) -> Option<&Card> {
        self.cards.last()
    }

    fn as_stack(&self) -> Stack {
        self.as_stack(None)
    }

    fn as_area(&self) -> &dyn Area {
        self
    }

    fn as_area_mut(&mut self) -> &mut dyn Area {
        self
    }
}

impl Area for SelectedTalon {
    fn id(&self) -> AreaId {
        Talon::id(self)
    }

    fn is_selected(&self) -> bool {
        true
    }

    fn is_held(&self) -> bool {
        self.selection.held_from.is_some()
    }

    fn give_cards(&mut self, held: Held) -> MoveResult<(), Held> {
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
        self.cards.last()
    }

    fn as_stack(&self) -> Stack {
        self.as_stack(Some(self.selection))
    }

    fn as_area(&self) -> &dyn Area {
        self
    }

    fn as_area_mut(&mut self) -> &mut dyn Area {
        self
    }
}

impl UnselectedArea for UnselectedTalon {
    fn select(self: Box<Self>) -> MoveResult<Box<dyn SelectedArea>, Box<dyn UnselectedArea>> {
        if !self.cards.is_empty() {
            MoveResult::Moved(Box::new(self.with_selection(Selection { held_from: None })))
        } else {
            NothingToSelect {
                message: "Empty area",
            }
            .fail_move(self)
        }
    }

    fn select_with_held(
        mut self: Box<Self>,
        held: Held,
    ) -> MoveResult<Box<dyn SelectedArea>, (Box<dyn UnselectedArea>, Held)> {
        let source = held.source;

        match self.give_cards(held) {
            MoveResult::Moved(()) => MoveResult::Moved(Box::new(self.with_selection(Selection {
                held_from: Some(source),
            }))),
            MoveResult::Unmoved(held, error) => MoveResult::Unmoved((self, held), error),
        }
    }
}

impl SelectedArea for SelectedTalon {
    fn deselect(mut self: Box<Self>) -> (Box<dyn UnselectedArea>, Option<Held>) {
        let held = if let Some(source) = self.selection.held_from {
            Some(self.take_cards(1, source))
        } else {
            None
        };

        let unselected = Box::new(self.with_selection(()));

        (unselected, held)
    }

    fn activate(&mut self) -> Result<Option<Action>> {
        if self.selection.held_from.is_some() {
            self.put_down()?;
        } else {
            self.pick_up()?;
        }

        Ok(None)
    }

    fn pick_up(&mut self) -> Result {
        self.selection.held_from = Some(self.id());
        Ok(())
    }

    fn put_down(&mut self) -> Result {
        self.selection.held_from = None;
        Ok(())
    }

    fn select_more(&mut self) -> Result {
        NotSupported {
            message: "Selection cannot be changed",
        }
        .fail()
    }
    fn select_less(&mut self) -> Result {
        NotSupported {
            message: "Selection cannot be changed",
        }
        .fail()
    }

    fn held_from(&self) -> Option<AreaId> {
        self.selection.held_from
    }
}
