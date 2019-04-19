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
    pub fn new(index: usize, cards: Vec<Card>, settings: &Settings) -> UnselectedFoundation {
        Foundation {
            index,
            cards,
            settings,
            selection: (),
        }
    }

    fn accepts_cards(&self, cards: &[Card]) -> bool {
        // We only accept one card at a time.
        if let [card] = cards {
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
}

impl<'a> Area for UnselectedFoundation<'a> {
    fn id(&self) -> AreaId {
        AreaId::Foundation(self.index)
    }

    fn as_stack(&self) -> Stack {
        return self.as_stack(None);
    }
}

impl<'a> Area for SelectedFoundation<'a> {
    fn id(&self) -> AreaId {
        AreaId::Foundation(self.index)
    }

    fn as_stack(&self) -> Stack {
        return self.as_stack(Some(self.selection));
    }
}

impl<'a> UnselectedArea for UnselectedFoundation<'a> {
    fn select(self: Box<Self>) -> Result<Box<dyn SelectedArea>, Box<dyn UnselectedArea>> {
        if !self.cards.is_empty() {
            Ok(Box::new(Foundation {
                index: self.index,
                cards: self.cards,
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
        if self.id() == held.source || self.accepts_cards(&held.cards) {
            self.cards.append(&mut held.cards);
            Ok(Box::new(Foundation {
                index: self.index,
                cards: self.cards,
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

impl<'a> SelectedArea for SelectedFoundation<'a> {
    fn deselect(self: Box<Self>) -> (Box<dyn UnselectedArea>, Option<Held>) {
        let held = if self.selection.held {
            // Our selection size is implicitly one, so we split off the last card as a new Vec.
            let cards = self.cards.split_off_bounded(1);

            Some(Held {
                source: self.id(),
                cards,
            })
        } else {
            None
        };

        let unselected = Box::new(Foundation {
            index: self.index,
            cards: self.cards,
            settings: self.settings,
            selection: (),
        });

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

    fn as_area(&self) -> &dyn Area {
        self
    }
}
