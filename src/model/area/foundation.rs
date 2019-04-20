use crate::{
    model::{
        card::Card,
        settings::Settings,
        stack::{Stack, StackDetails, StackSelection},
    },
    utils::vec::SplitOffBounded,
};

use super::{Action, Area, AreaId, Held, SelectedArea, UnselectedArea};

/// Selection of a foundation area. Only the top card of a foundation area can be selected, and that
/// card can either be held (picked up to move) or not. Additionally, depending on settings, it may
/// not be allowed to move cards out of a foundation area.
#[derive(Copy, Clone, Debug)]
pub struct Selection {
    /// Whether the selected card is picked up to move
    held: bool,
}

/// A foundation area in Klondike. The foundations are the areas where cards are moved to win,
/// creating piles by suit, starting with aces and ending with kings.
#[derive(Debug)]
pub struct Foundation<'a, S> {
    /// The index of this foundation area in the list of foundation areas. In Klondike there is one
    /// foundation for each suit.
    index: usize,
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
        AreaId::Foundation(self.index)
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
                self.index == card.suit.index() && foundation_card.rank.is_followed_by(card.rank)
            } else {
                // If there are no cards in this foundation yet, we have to start with the ace.
                self.index == card.suit.index() && card.rank.is_ace()
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

    fn take_cards(&mut self, len: usize) -> Held {
        let cards = self.cards.split_off_bounded(len);

        Held {
            source: self.id(),
            cards,
        }
    }

    fn as_stack(&self, selection: Option<Selection>) -> Stack {
        let cards_len = self.cards.len();

        Stack {
            cards: &self.cards,
            details: StackDetails {
                len: cards_len,
                face_up_len: cards_len,
                visible_len: 2,
                spread_len: 1,
                selection: selection.map(|_| StackSelection::Cards(1)),
            },
        }
    }

    fn with_selection<T>(self, selection: T) -> Foundation<'a, T> {
        Foundation {
            index: self.index,
            cards: self.cards,
            settings: self.settings,
            selection,
        }
    }
}

impl<'a> UnselectedFoundation<'a> {
    pub fn create<'b>(
        index: usize,
        cards: Vec<Card>,
        settings: &'a Settings,
    ) -> Box<dyn UnselectedArea<'a> + 'b>
    where
        'a: 'b,
    {
        Box::new(Foundation {
            index,
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
        Foundation::take_cards(self, len)
    }

    fn take_all_cards(&mut self) -> Held {
        Foundation::take_cards(self, self.cards.len())
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
        self.selection.held = false;
        Foundation::give_cards(self, held)
    }

    fn take_cards(&mut self, len: usize) -> Held {
        self.selection.held = false;
        Foundation::take_cards(self, len)
    }

    fn take_all_cards(&mut self) -> Held {
        self.selection.held = false;
        Foundation::take_cards(self, self.cards.len())
    }

    fn as_stack(&self) -> Stack {
        self.as_stack(Some(self.selection))
    }
}

impl<'a> UnselectedArea<'a> for UnselectedFoundation<'a> {
    fn select<'b>(
        self: Box<Self>,
    ) -> Result<Box<dyn SelectedArea<'a> + 'b>, Box<dyn UnselectedArea<'a> + 'b>>
    where
        'a: 'b,
    {
        if !self.cards.is_empty() {
            Ok(Box::new(self.with_selection(Selection { held: false })))
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
        match self.give_cards(held) {
            Ok(()) => Ok(Box::new(self.with_selection(Selection { held: true }))),
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
    fn deselect<'b>(mut self: Box<Self>) -> (Box<dyn UnselectedArea<'a> + 'b>, Option<Held>)
    where
        'a: 'b,
    {
        let held = if self.selection.held {
            // Our selection size is implicitly one
            Some(self.take_cards(1))
        } else {
            None
        };

        let unselected = Box::new(self.with_selection(()));

        (unselected, held)
    }

    fn activate(&mut self) -> Option<Action> {
        if self.selection.held {
            self.selection.held = false;
        } else if self.settings.take_from_foundation {
            self.selection.held = true;
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
