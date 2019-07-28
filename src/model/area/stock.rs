use crate::{
    model::{
        card::Card,
        settings::Settings,
        stack::{Orientation, Stack, StackDetails, StackSelection},
    },
    utils::vec::SplitOffBounded,
};

use super::{
    Action, Area, AreaId, Held, MoveResult, NotSupported, Result, SelectedArea, SnafuSelectorExt,
    UnselectedArea,
};

#[derive(Copy, Clone, Debug)]
pub struct Selection;

#[derive(Debug)]
pub struct Stock<'a, S> {
    cards: Vec<Card>,
    settings: &'a Settings,
    selection: S,
}

pub type UnselectedStock<'a> = Stock<'a, ()>;
pub type SelectedStock<'a> = Stock<'a, Selection>;

impl<'a, S> Stock<'a, S> {
    fn id(&self) -> AreaId {
        AreaId::Stock
    }

    fn validate_cards(&self, held: &Held) -> Result {
        if held.source == self.id() {
            // We'll always take back our own cards.
            Ok(())
        } else if held.source == AreaId::Talon {
            // We'll allow cards from the talon to be replaced onto us.
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
                self.cards.append(&mut held.cards);
                MoveResult::Moved(())
            }
            Err(error) => MoveResult::Unmoved(held, error),
        }
    }

    fn take_cards(&mut self, len: usize) -> Held {
        let cards = self.cards.split_off_bounded(len);

        Held {
            source: AreaId::Stock,
            cards,
        }
    }

    fn as_stack(&self, mode: Option<Selection>) -> Stack {
        Stack {
            cards: &self.cards,
            details: StackDetails {
                orientation: Orientation::Horizontal,
                len: self.cards.len(),
                face_up_len: 0,
                visible_len: 2,
                spread_len: 1,
                selection: mode.map(|_| StackSelection {
                    len: 1,
                    held: false,
                }),
            },
        }
    }

    fn with_selection<T>(self, selection: T) -> Stock<'a, T> {
        Stock {
            cards: self.cards,
            settings: self.settings,
            selection,
        }
    }
}

impl<'a> UnselectedStock<'a> {
    pub fn create(cards: Vec<Card>, settings: &'a Settings) -> Box<dyn UnselectedArea<'a> + 'a> {
        Box::new(Stock {
            cards,
            settings,
            selection: (),
        })
    }
}

impl<'a> Area<'a> for UnselectedStock<'a> {
    fn id(&self) -> AreaId {
        Stock::id(self)
    }

    fn give_cards(&mut self, held: Held) -> MoveResult<(), Held> {
        Stock::give_cards(self, held)
    }

    fn take_cards(&mut self, len: usize) -> Held {
        Stock::take_cards(self, len)
    }

    fn take_all_cards(&mut self) -> Held {
        Stock::take_cards(self, self.cards.len())
    }

    fn peek_top_card(&self) -> Option<&Card> {
        self.cards.first()
    }

    fn as_stack(&self) -> Stack {
        self.as_stack(None)
    }
}

impl<'a> Area<'a> for SelectedStock<'a> {
    fn id(&self) -> AreaId {
        Stock::id(self)
    }

    fn give_cards(&mut self, held: Held) -> MoveResult<(), Held> {
        Stock::give_cards(self, held)
    }

    fn take_cards(&mut self, len: usize) -> Held {
        Stock::take_cards(self, len)
    }

    fn take_all_cards(&mut self) -> Held {
        Stock::take_cards(self, self.cards.len())
    }

    fn peek_top_card(&self) -> Option<&Card> {
        self.cards.first()
    }

    fn as_stack(&self) -> Stack {
        self.as_stack(Some(self.selection))
    }
}

impl<'a> UnselectedArea<'a> for UnselectedStock<'a> {
    fn select(
        self: Box<Self>,
    ) -> MoveResult<Box<dyn SelectedArea<'a> + 'a>, Box<dyn UnselectedArea<'a> + 'a>> {
        MoveResult::Moved(Box::new(self.with_selection(Selection)))
    }

    fn select_with_held(
        self: Box<Self>,
        held: Held,
    ) -> MoveResult<Box<dyn SelectedArea<'a> + 'a>, (Box<dyn UnselectedArea<'a> + 'a>, Held)> {
        NotSupported {
            message: "Cards in this area cannot be held",
        }
        .fail_move((self, held))
    }

    fn as_area<'b>(&'b self) -> &'b dyn Area<'a>
    where
        'a: 'b,
    {
        self
    }

    fn as_area_mut<'b>(&'b mut self) -> &'b mut dyn Area<'a>
    where
        'a: 'b,
    {
        self
    }
}

impl<'a> SelectedArea<'a> for SelectedStock<'a> {
    fn deselect(self: Box<Self>) -> (Box<dyn UnselectedArea<'a> + 'a>, Option<Held>) {
        let unselected = Box::new(self.with_selection(()));
        (unselected, None)
    }

    fn activate(&mut self) -> Result<Option<Action>> {
        if self.cards.is_empty() {
            Ok(Some(Action::Restock))
        } else {
            Ok(Some(Action::Draw(self.settings.draw_from_stock_len)))
        }
    }

    fn pick_up(&mut self) -> Result {
        NotSupported {
            message: "Cards in this area cannot be held",
        }
        .fail()
    }
    fn put_down(&mut self) -> Result {
        NotSupported {
            message: "Cards in this area cannot be held",
        }
        .fail()
    }
    fn select_more(&mut self) -> Result {
        NotSupported {
            message: "Cards in this area cannot be held",
        }
        .fail()
    }
    fn select_less(&mut self) -> Result {
        NotSupported {
            message: "Cards in this area cannot be held",
        }
        .fail()
    }

    fn held_from(&self) -> Option<AreaId> {
        None
    }

    fn as_area<'b>(&'b self) -> &'b dyn Area<'a>
    where
        'a: 'b,
    {
        self
    }

    fn as_area_mut<'b>(&'b mut self) -> &'b mut dyn Area<'a>
    where
        'a: 'b,
    {
        self
    }
}
