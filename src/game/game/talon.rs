use crate::game::card::*;
use crate::game::game::*;
use crate::game::stack::*;


#[derive(Debug)]
pub struct BaseTalon {
    cards: Vec<Card>,
    fanned_len: usize,
}

impl Area for BaseTalon {
    fn id(&self) -> AreaId {
        AreaId::Talon
    }
}


#[derive(Debug)]
pub struct UnfocusedTalon {
    base: BaseTalon
}

impl UnfocusedTalon {
    pub fn new(cards: Vec<Card>, fanned_len: usize) -> UnfocusedTalon {
        UnfocusedTalon {
            base: BaseTalon { cards, fanned_len }
        }
    }
}

impl Area for UnfocusedTalon {
    fn id(&self) -> AreaId {
        self.base.id()
    }
}

impl<'a> From<&'a UnfocusedTalon> for Stack<'a> {
    fn from(area: &'a UnfocusedTalon) -> Self {
        as_stack_helper(&area.base, None)
    }
}

impl UnfocusedArea for UnfocusedTalon {
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
            Ok(FocusedTalon { base: self.base, focus })
        } else {
            Err((self, focus))
        }
    }
}


#[derive(Debug)]
pub struct FocusedTalon {
    base: BaseTalon,
    focus: Focus,
}

impl Area for FocusedTalon {
    fn id(&self) -> AreaId {
        self.base.id()
    }
}

impl<'a> From<&'a FocusedTalon> for Stack<'a> {
    fn from(area: &'a FocusedTalon) -> Self {
        let base_stack = as_stack_helper(&area.base, Some(&area.focus));

        if let Some(ref held) = area.focus.held {
            base_stack.with_floating_cards_spread(&held.cards)
        } else {
            base_stack
        }
    }
}

impl FocusedArea for FocusedTalon {
    type Unfocused = UnfocusedTalon;

    fn take_focus(self) -> (Self::Unfocused, Focus) {
        (UnfocusedTalon { base: self.base }, self.focus)
    }
}


fn as_stack_helper<'a>(base: &'a BaseTalon, focus: Option<&'a Focus>) -> Stack<'a> {
    Stack::new(
        &base.cards,
        StackDetails {
            len: base.cards.len(),
            visible_len: base.fanned_len + 1,
            spread_len: base.fanned_len,
            selection: focus.map(|_| StackSelection::Cards(1)),
        },
    )
}
