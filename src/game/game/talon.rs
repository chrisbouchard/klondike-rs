use crate::game::card::Card;
use crate::game::game::area::{Area, AreaId, Focus, FocusedArea, UnfocusedArea};
use crate::game::stack::{Stack, StackDetails, StackSelection};

#[derive(Debug)]
pub struct Talon<F = ()> {
    cards: Vec<Card>,
    fanned_len: usize,
    focus: F,
}

pub type FocusedTalon = Talon<Focus>;

impl<F> Talon<F> {
    pub fn new(cards: Vec<Card>, fanned_len: usize) -> Talon {
        Talon { cards, fanned_len, focus: () }
    }

    fn as_stack_helper<'a>(&'a self, focus: Option<&'a Focus>) -> Stack<'a> {
        Stack::new(
            &self.cards,
            StackDetails {
                len: self.cards.len(),
                visible_len: self.fanned_len + 1,
                spread_len: self.fanned_len,
                selection: focus.map(|_| StackSelection::Cards(1)),
            },
        )
    }

    fn take_focus_unsafe(self) -> (Talon, F) {
        (self.with_focus_unsafe(()), self.focus)
    }

    fn with_focus_unsafe<G>(self, focus: G) -> Talon<G> {
        Talon { cards: self.cards, fanned_len: self.fanned_len, focus }
    }
}

impl Area for Talon {
    fn id(&self) -> AreaId {
        AreaId::Talon
    }

    fn as_stack(&self) -> Stack {
        self.as_stack_helper(None)
    }
}

impl Area for FocusedTalon {
    fn id(&self) -> AreaId {
        AreaId::Talon
    }

    fn as_stack(&self) -> Stack {
        let base_stack = self.as_stack_helper(Some(&self.focus));

        if let Some(ref held) = self.focus.held {
            base_stack.with_floating_cards_spread(&held.cards)
        } else {
            base_stack
        }
    }
}

impl UnfocusedArea for Talon {
    type Focused = FocusedTalon;

    fn accepts_focus(&self, focus: &Focus) -> bool {
        if let Some(ref held) = focus.held {
            held.source == AreaId::Talon
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

impl FocusedArea for FocusedTalon {
    type Unfocused = Talon;

    fn try_move_focus<A>(self, other: A) -> Result<(Self::Unfocused, A::Focused), (Self, A)>
        where Self: Sized, Self::Unfocused: Sized, A: UnfocusedArea, A::Focused: Sized {
        let (talon, focus) = self.take_focus_unsafe();

        match other.try_give_focus(focus) {
            Ok(other) => {
                Ok((talon, other))
            }
            Err((other, focus)) => {
                let talon = talon.with_focus_unsafe(focus);
                Err((talon, other))
            }
        }
    }
}
