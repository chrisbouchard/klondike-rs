use crate::game::card::Card;
use crate::game::game::area::{Area, AreaId, Focus, FocusedArea, UnfocusedArea};
use crate::game::stack::{Stack, StackDetails, StackSelection};

#[derive(Debug)]
pub struct Foundation<F = ()> {
    index: usize,
    cards: Vec<Card>,
    focus: F,
}

pub type FocusedFoundation = Foundation<Focus>;

impl<F> Foundation<F> {
    pub fn new(index: usize, cards: Vec<Card>) -> Foundation {
        Foundation { index, cards, focus: () }
    }

    fn as_stack_helper<'a>(&'a self, focus: Option<&'a Focus>) -> Stack<'a> {
        Stack::new(
            &self.cards,
            StackDetails {
                len: self.cards.len(),
                visible_len: 2,
                spread_len: 1,
                selection: focus.map(|_| StackSelection::Cards(1)),
            },
        )
    }

    fn take_focus_unsafe(self) -> (Foundation, F) {
        (self.with_focus_unsafe(()), self.focus)
    }

    fn with_focus_unsafe<G>(self, focus: G) -> Foundation<G> {
        Foundation { index: self.index, cards: self.cards, focus }
    }
}

impl Area for Foundation {
    fn id(&self) -> AreaId {
        AreaId::Foundation(self.index)
    }

    fn as_stack(&self) -> Stack {
        self.as_stack_helper(None)
    }
}

impl Area for FocusedFoundation {
    fn id(&self) -> AreaId {
        AreaId::Foundation(self.index)
    }

    fn as_stack(&self) -> Stack {
        self.as_stack_helper(Some(&self.focus))
    }
}

impl UnfocusedArea for Foundation {
    type Focused = FocusedFoundation;

    fn accepts_focus(&self, focus: &Focus) -> bool {
        if let Some(ref held) = focus.held {
            if let Some((card, &[])) = held.cards.split_first() {
                if let Some(foundation_card) = self.base.cards.last() {
                    self.base.index == card.suit.index()
                        && card.rank.is_followed_by(foundation_card.rank)
                } else {
                    self.base.index == card.suit.index()
                        && card.rank.is_ace()
                }
            } else {
                false
            }
        } else {
            !self.base.cards.is_empty()
        }
    }

    fn try_give_focus(self, focus: Focus) -> Result<Self::Focused, (Self, Focus)> {
        if self.accepts_focus(&focus) {
            Ok(self.with_focus_unsafe(focus))
        } else {
            Err((self, focus))
        }
    }
}

impl FocusedArea for FocusedFoundation {
    type Unfocused = Foundation;

    fn try_move_focus<A>(self, other: A) -> Result<(Self::Unfocused, A::Focused), (Self, A)>
        where Self: Sized, Self::Unfocused: Sized, A: UnfocusedArea, A::Focused: Sized {
        let (foundation, focus) = self.take_focus_unsafe();

        match other.try_give_focus(focus) {
            Ok(other) => {
                Ok((foundation, other))
            }
            Err((other, focus)) => {
                let foundation = foundation.with_focus_unsafe(focus);
                Err((foundation, other))
            }
        }
    }
}
