use crate::{
    model::{
        area::{AlreadyHeld, MaxSelection, MinSelection, NothingHeld},
        card::{Card, Rank},
        stack::{Orientation, Stack, StackDetails, StackSelection},
    },
    utils::vec::SplitOffBounded,
};

use super::{
    Action, Area, AreaId, Held, InvalidCard, MoveResult, NothingToSelect, Result, SelectedArea,
    SnafuSelectorExt, UnselectedArea,
};

#[derive(Copy, Clone, Debug)]
pub struct Selection {
    held_from: Option<AreaId>,
    len: usize,
}

#[derive(Debug)]
pub struct Tableaux<S> {
    index: u8,
    cards: Vec<Card>,
    revealed_len: usize,
    selection: S,
}

pub type UnselectedTableaux = Tableaux<()>;
pub type SelectedTableaux = Tableaux<Selection>;

impl<S> Tableaux<S> {
    fn id(&self) -> AreaId {
        AreaId::Tableaux(self.index)
    }

    fn accepts_cards(&self, held: &Held) -> Result {
        if held.source == self.id() {
            // We'll always take back our own cards.
            Ok(())
        } else if let Some(card) = held.cards.first() {
            if let Some(tableaux_card) = self.cards.last() {
                // TODO: Check that the pile itself is legit.
                ensure!(
                    self.revealed_len > 0
                        && card.rank.is_followed_by(tableaux_card.rank)
                        && card.color() != tableaux_card.color(),
                    InvalidCard {
                        message: format!(
                            "Card does not follow: card: {:?}, top: {:?}",
                            card, tableaux_card
                        )
                    }
                );
                Ok(())
            } else {
                ensure!(
                    card.rank == Rank::King,
                    InvalidCard {
                        message: format!("Card does not follow: card: {:?}, top: empty", card)
                    }
                );
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    fn give_cards(&mut self, mut held: Held) -> MoveResult<(), Held> {
        match self.accepts_cards(&held) {
            Ok(_) => {
                self.revealed_len += held.cards.len();
                self.cards.append(&mut held.cards);
                MoveResult::Moved(())
            }
            Err(error) => MoveResult::Unmoved(held, error),
        }
    }

    fn take_cards(&mut self, len: usize, source: AreaId) -> Held {
        let cards = self.cards.split_off_bounded(len);
        self.revealed_len -= cards.len();

        Held { source, cards }
    }

    fn as_stack(&self, mode: Option<Selection>) -> Stack<'_> {
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

    fn with_selection<T>(self, selection: T) -> Tableaux<T> {
        Tableaux {
            index: self.index,
            cards: self.cards,
            revealed_len: self.revealed_len,
            selection,
        }
    }
}

impl UnselectedTableaux {
    pub fn create(index: u8, revealed_len: usize, cards: Vec<Card>) -> Box<dyn UnselectedArea> {
        Box::new(Tableaux {
            index,
            cards,
            revealed_len,
            selection: (),
        })
    }
}

impl Area for UnselectedTableaux {
    fn id(&self) -> AreaId {
        Tableaux::id(self)
    }

    fn is_selected(&self) -> bool {
        false
    }

    fn is_held(&self) -> bool {
        false
    }

    fn give_cards(&mut self, held: Held) -> MoveResult<(), Held> {
        Tableaux::give_cards(self, held)
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

    fn as_stack(&self) -> Stack<'_> {
        self.as_stack(None)
    }

    fn as_area(&self) -> &dyn Area {
        self
    }

    fn as_area_mut(&mut self) -> &mut dyn Area {
        self
    }
}

impl Area for SelectedTableaux {
    fn id(&self) -> AreaId {
        Tableaux::id(self)
    }

    fn is_selected(&self) -> bool {
        true
    }

    fn is_held(&self) -> bool {
        self.selection.held_from.is_some()
    }

    fn give_cards(&mut self, held: Held) -> MoveResult<(), Held> {
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
        self.cards.last()
    }

    fn as_stack(&self) -> Stack<'_> {
        self.as_stack(Some(self.selection))
    }

    fn as_area(&self) -> &dyn Area {
        self
    }

    fn as_area_mut(&mut self) -> &mut dyn Area {
        self
    }
}

impl UnselectedArea for UnselectedTableaux {
    fn select(self: Box<Self>) -> MoveResult<Box<dyn SelectedArea>, Box<dyn UnselectedArea>> {
        if !self.cards.is_empty() {
            MoveResult::Moved(Box::new(self.with_selection(Selection {
                held_from: None,
                len: 1,
            })))
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
        let len = held.cards.len();

        match self.give_cards(held) {
            MoveResult::Moved(()) => MoveResult::Moved(Box::new(self.with_selection(Selection {
                held_from: Some(source),
                len,
            }))),
            MoveResult::Unmoved(held, error) => MoveResult::Unmoved((self, held), error),
        }
    }
}

impl SelectedArea for SelectedTableaux {
    fn deselect(mut self: Box<Self>) -> (Box<dyn UnselectedArea>, Option<Held>) {
        let held = if let Some(source) = self.selection.held_from {
            Some(self.take_cards(self.selection.len, source))
        } else {
            None
        };

        let unselected = Box::new(self.with_selection(()));

        (unselected, held)
    }

    fn activate(&mut self) -> Result<Option<Action>> {
        if self.selection.held_from.is_some() {
            self.put_down()?;
            Ok(None)
        } else if self.revealed_len > 0 {
            self.pick_up()?;
            Ok(None)
        } else if !self.cards.is_empty() {
            self.revealed_len += 1;
            Ok(None)
        } else {
            NothingToSelect {
                message: "Empty area",
            }
            .fail()
        }
    }

    fn pick_up(&mut self) -> Result {
        ensure!(self.selection.held_from.is_none(), AlreadyHeld);

        ensure!(
            !self.cards.is_empty(),
            NothingToSelect {
                message: "Empty area",
            }
        );

        ensure!(
            self.revealed_len > 0,
            NothingToSelect {
                message: "No visible cards",
            }
        );

        self.selection.held_from = Some(self.id());
        Ok(())
    }

    fn put_down(&mut self) -> Result {
        ensure!(self.selection.held_from.is_some(), NothingHeld);
        self.selection.held_from = None;
        Ok(())
    }

    fn select_more(&mut self) -> Result {
        ensure!(self.selection.len < self.revealed_len, MaxSelection);
        self.selection.len += 1;
        Ok(())
    }

    fn select_less(&mut self) -> Result {
        ensure!(self.selection.len > 1, MinSelection);
        self.selection.len -= 1;
        Ok(())
    }

    fn held_from(&self) -> Option<AreaId> {
        self.selection.held_from
    }
}
