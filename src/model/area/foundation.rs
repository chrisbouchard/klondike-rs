use crate::{
    model::{
        card::{Card, Suit},
        settings::Settings,
        stack::{Orientation, Stack, StackDetails, StackSelection},
    },
    utils::vec::SplitOffBounded,
};

use super::{Action, Area, AreaId, Held, SelectedArea, UnselectedArea};

/// Selection of a foundation area. Only the top card of a foundation area can be selected, and that
/// card can either be held (picked up to move) or not. Additionally, depending on settings, it may
/// not be allowed to move cards out of a foundation area.
#[derive(Copy, Clone, Debug)]
pub struct Selection {
    held_from: Option<AreaId>,
}

/// A foundation area in Klondike. The foundations are the areas where cards are moved to win,
/// creating piles by suit, starting with aces and ending with kings.
#[derive(Debug)]
pub struct Foundation<'a, S> {
    /// The suit of this foundation. In Klondike there is one foundation for each suit.
    suit: Suit,
    /// The cards in this area
    cards: Vec<Card>,
    /// The game settings
    settings: &'a Settings,
    /// The current selection state of this foundation area. Expected values are either `()` for
    /// unselected, or a [`Selection`](Selection) instance for selected.
    selection: S,
}

/// A foundation area in Klondike that is currently unselected.
pub type UnselectedFoundation<'a> = Foundation<'a, ()>;

/// A foundation area in Klondike that is currently selected. Only the top card of a foundation area
/// can be selected, and that card can either be held (picked up to move) or not. Additionally,
/// depending on settings, it may not be allowed to move cards out of a foundation area.
pub type SelectedFoundation<'a> = Foundation<'a, Selection>;

impl<'a, S> Foundation<'a, S> {
    fn id(&self) -> AreaId {
        AreaId::Foundation(self.suit)
    }

    fn accepts_cards(&self, held: &Held) -> bool {
        if held.source == self.id() {
            true
        }
        // We only accept one card at a time.
        else if let [card] = held.cards.as_slice() {
            if let Some(foundation_card) = self.cards.last() {
                // If there are already cards in this foundation, only accept the next card in
                // sequence.
                self.suit.index() == card.suit.index()
                    && foundation_card.rank.is_followed_by(card.rank)
            } else {
                // If there are no cards in this foundation yet, we have to start with the ace.
                self.suit.index() == card.suit.index() && card.rank.is_ace()
            }
        } else {
            // Reject too many or too few cards.
            false
        }
    }

    fn give_cards(&mut self, mut held: Held) -> Result<(), Held> {
        if self.accepts_cards(&held) {
            self.cards.append(&mut held.cards);
            Ok(())
        } else {
            Err(held)
        }
    }

    fn take_cards(&mut self, len: usize, source: AreaId) -> Held {
        let cards = self.cards.split_off_bounded(len);

        Held { source, cards }
    }

    fn as_stack(&self, selection: Option<Selection>) -> Stack {
        let cards_len = self.cards.len();

        Stack {
            cards: &self.cards,
            details: StackDetails {
                orientation: Orientation::Horizontal,
                len: cards_len,
                face_up_len: cards_len,
                visible_len: 2,
                spread_len: 1,
                selection: selection.map(|selection| StackSelection {
                    len: 1,
                    held: selection.held_from.is_some(),
                }),
            },
        }
    }

    fn with_selection<T>(self, selection: T) -> Foundation<'a, T> {
        Foundation {
            suit: self.suit,
            cards: self.cards,
            settings: self.settings,
            selection,
        }
    }
}

impl<'a> UnselectedFoundation<'a> {
    pub fn create(
        suit: Suit,
        cards: Vec<Card>,
        settings: &'a Settings,
    ) -> Box<dyn UnselectedArea<'a> + 'a> {
        Box::new(Foundation {
            suit,
            cards,
            settings,
            selection: (),
        })
    }
}

impl<'a> Area<'a> for UnselectedFoundation<'a> {
    fn id(&self) -> AreaId {
        Foundation::id(self)
    }

    fn give_cards(&mut self, held: Held) -> Result<(), Held> {
        Foundation::give_cards(self, held)
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

impl<'a> Area<'a> for SelectedFoundation<'a> {
    fn id(&self) -> AreaId {
        Foundation::id(self)
    }

    fn give_cards(&mut self, held: Held) -> Result<(), Held> {
        self.selection.held_from = None;
        Foundation::give_cards(self, held)
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

impl<'a> UnselectedArea<'a> for UnselectedFoundation<'a> {
    fn select(
        self: Box<Self>,
    ) -> Result<Box<dyn SelectedArea<'a> + 'a>, Box<dyn UnselectedArea<'a> + 'a>> {
        if !self.cards.is_empty() {
            Ok(Box::new(self.with_selection(Selection { held_from: None })))
        } else {
            Err(self)
        }
    }

    fn select_with_held(
        mut self: Box<Self>,
        held: Held,
    ) -> Result<Box<dyn SelectedArea<'a> + 'a>, (Box<dyn UnselectedArea<'a> + 'a>, Held)> {
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

impl<'a> SelectedArea<'a> for SelectedFoundation<'a> {
    fn deselect(mut self: Box<Self>) -> (Box<dyn UnselectedArea<'a> + 'a>, Option<Held>) {
        let held = if let Some(source) = self.selection.held_from {
            // Our selection size is implicitly one
            Some(self.take_cards(1, source))
        } else {
            None
        };

        let unselected = Box::new(self.with_selection(()));

        (unselected, held)
    }

    fn activate(&mut self) -> Option<Action> {
        if self.selection.held_from.is_some() {
            self.put_down();
        } else {
            self.pick_up();
        }

        None
    }

    fn pick_up(&mut self) {
        if self.settings.take_from_foundation {
            self.selection.held_from = Some(self.id());
        }
    }

    fn put_down(&mut self) {
        self.selection.held_from = None;
    }

    fn select_more(&mut self) {}
    fn select_less(&mut self) {}

    fn held_from(&self) -> Option<AreaId> {
        self.selection.held_from
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
