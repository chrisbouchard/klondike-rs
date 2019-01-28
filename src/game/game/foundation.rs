use crate::game::card::*;
use crate::game::game::*;
use crate::game::stack::*;


#[derive(Debug)]
pub struct BaseFoundation {
    index: usize,
    cards: Vec<Card>,
}

impl Area for BaseFoundation {
    fn id(&self) -> AreaId {
        AreaId::Foundation(self.index)
    }
}


#[derive(Debug)]
pub struct UnfocusedFoundation {
    base: BaseFoundation
}

impl UnfocusedFoundation {
    pub fn new(index: usize, cards: Vec<Card>) -> UnfocusedFoundation {
        UnfocusedFoundation {
            base: BaseFoundation { index, cards }
        }
    }
}

impl Area for UnfocusedFoundation {
    fn id(&self) -> AreaId {
        self.base.id()
    }
}

impl<'a> From<&'a UnfocusedFoundation> for Stack<'a> {
    fn from(area: &'a UnfocusedFoundation) -> Self {
        as_stack_helper(&area.base, None)
    }
}

impl UnfocusedArea for UnfocusedFoundation {
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
            Ok(FocusedFoundation { base: self.base, focus })
        } else {
            Err((self, focus))
        }
    }
}


#[derive(Debug)]
pub struct FocusedFoundation {
    base: BaseFoundation,
    focus: Focus,
}

impl Area for FocusedFoundation {
    fn id(&self) -> AreaId {
        self.base.id()
    }
}

impl<'a> From<&'a FocusedFoundation> for Stack<'a> {
    fn from(area: &'a FocusedFoundation) -> Self {
        let base_stack = as_stack_helper(&area.base, Some(&area.focus));

        if let Some(ref held) = area.focus.held {
            base_stack.with_floating_cards(&held.cards)
        } else {
            base_stack
        }
    }
}

impl FocusedArea for FocusedFoundation {
    type Unfocused = UnfocusedFoundation;

    fn take_focus(self) -> (Self::Unfocused, Focus) {
        (UnfocusedFoundation { base: self.base }, self.focus)
    }
}


fn as_stack_helper<'a>(base: &'a BaseFoundation, focus: Option<&'a Focus>) -> Stack<'a> {
    Stack::new(
        &base.cards,
        StackDetails {
            len: base.cards.len(),
            visible_len: 2,
            spread_len: 1,
            selection: focus.map(|_| StackSelection::Cards(1)),
        },
    )
}
