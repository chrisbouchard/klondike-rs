use crate::game::card::Card;
use crate::game::game::area::{Area, AreaId, Focus, FocusedArea, UnfocusedArea};
use crate::game::stack::{Stack, StackDetails, StackSelection};

#[derive(Debug)]
pub struct Stock<F = ()> {
    cards: Vec<Card>,
    focus: F,
}

pub type FocusedStock = Stock<Focus>;

impl<F> Stock<F> {
    pub fn new(cards: Vec<Card>) -> Stock {
        Stock { cards, focus: () }
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

    fn take_focus_unsafe(self) -> (Stock, F) {
        (self.with_focus_unsafe(()), self.focus)
    }

    fn with_focus_unsafe<G>(self, focus: G) -> Stock<G> {
        Stock { cards: self.cards, focus }
    }
}

impl Area for Stock {
    fn id(&self) -> AreaId {
        AreaId::Stock
    }

    fn as_stack(&self) -> Stack {
        self.as_stack_helper(None)
    }
}

impl Area for FocusedStock {
    fn id(&self) -> AreaId {
        AreaId::Stock
    }

    fn as_stack(&self) -> Stack {
        self.as_stack_helper(Some(&self.focus))
    }
}

impl UnfocusedArea for Stock {
    type Focused = FocusedStock;

    fn accepts_focus(&self, focus: &Focus) -> bool {
        focus.held.is_none()
    }

    fn try_give_focus(self, focus: Focus) -> Result<Self::Focused, (Self, Focus)> {
        if self.accepts_focus(&focus) {
            Ok(self.with_focus_unsafe(focus))
        } else {
            Err((self, focus))
        }
    }
}

impl FocusedArea for FocusedStock {
    type Unfocused = Stock;

    fn try_move_focus<A>(self, other: A) -> Result<(Self::Unfocused, A::Focused), (Self, A)>
        where Self: Sized, Self::Unfocused: Sized, A: UnfocusedArea, A::Focused: Sized {
        let (stock, focus) = self.take_focus_unsafe();

        match other.try_give_focus(focus) {
            Ok(other) => {
                Ok((stock, other))
            }
            Err((other, focus)) => {
                let stock = stock.with_focus_unsafe(focus);
                Err((stock, other))
            }
        }
    }
}
