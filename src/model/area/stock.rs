use crate::{
    model::{
        card::Card,
        settings::GameSettings,
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
pub struct Stock<S> {
    cards: Vec<Card>,
    draw_from_stock_len: usize,
    selection: S,
}

pub type UnselectedStock = Stock<()>;
pub type SelectedStock = Stock<Selection>;

impl<S> Stock<S> {
    fn id(&self) -> AreaId {
        AreaId::Stock
    }

    fn validate_cards(&self, held: &Held) -> Result {
        if held.source == self.id() || held.source == AreaId::Talon {
            // We'll always take back our own cards, and we'll allow cards from the talon to be
            // replaced on us.
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

    fn with_selection<T>(self, selection: T) -> Stock<T> {
        Stock {
            cards: self.cards,
            draw_from_stock_len: self.draw_from_stock_len,
            selection,
        }
    }
}

impl UnselectedStock {
    pub fn create(cards: Vec<Card>, settings: &GameSettings) -> Box<dyn UnselectedArea> {
        Box::new(Stock {
            cards,
            draw_from_stock_len: settings.draw_from_stock_len,
            selection: (),
        })
    }
}

impl Area for UnselectedStock {
    fn id(&self) -> AreaId {
        Stock::id(self)
    }

    fn is_selected(&self) -> bool {
        false
    }

    fn is_held(&self) -> bool {
        false
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

impl Area for SelectedStock {
    fn id(&self) -> AreaId {
        Stock::id(self)
    }

    fn is_selected(&self) -> bool {
        true
    }

    fn is_held(&self) -> bool {
        false
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

impl UnselectedArea for UnselectedStock {
    fn select(self: Box<Self>) -> MoveResult<Box<dyn SelectedArea>, Box<dyn UnselectedArea>> {
        MoveResult::Moved(Box::new(self.with_selection(Selection)))
    }

    fn select_with_held(
        self: Box<Self>,
        held: Held,
    ) -> MoveResult<Box<dyn SelectedArea>, (Box<dyn UnselectedArea>, Held)> {
        NotSupported {
            message: "Cards in this area cannot be held",
        }
        .fail_move((self, held))
    }
}

impl SelectedArea for SelectedStock {
    fn deselect(self: Box<Self>) -> (Box<dyn UnselectedArea>, Option<Held>) {
        let unselected = Box::new(self.with_selection(()));
        (unselected, None)
    }

    fn activate(&mut self) -> Result<Option<Action>> {
        if self.cards.is_empty() {
            Ok(Some(Action::Restock))
        } else {
            Ok(Some(Action::Draw(self.draw_from_stock_len)))
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
}
