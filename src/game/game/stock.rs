use crate::game::card::*;
use crate::game::game::*;
use crate::game::stack::*;


#[derive(Debug)]
pub struct BaseStock {
    cards: Vec<Card>
}

impl Area for BaseStock {
    fn id(&self) -> AreaId {
        AreaId::Stock
    }
}


#[derive(Debug)]
pub struct UnfocusedStock {
    base: BaseStock
}

impl UnfocusedStock {
    pub fn new(cards: Vec<Card>) -> UnfocusedStock {
        UnfocusedStock { base: BaseStock { cards } }
    }
}

impl Area for UnfocusedStock {
    fn id(&self) -> AreaId {
        self.base.id()
    }
}

impl<'a> From<&'a UnfocusedStock> for Stack<'a> {
    fn from(area: &'a UnfocusedStock) -> Self {
        as_stack_helper(&area.base, None)
    }
}

impl UnfocusedArea for UnfocusedStock {
    type Focused = FocusedStock;

    fn accepts_focus(&self, focus: &Focus) -> bool {
        focus.held.is_none()
    }

    fn try_give_focus(self, focus: Focus) -> Result<Self::Focused, (Self, Focus)> {
        if self.accepts_focus(&focus) {
            Ok(FocusedStock { base: self.base, focus })
        } else {
            Err((self, focus))
        }
    }
}


#[derive(Debug)]
pub struct FocusedStock {
    base: BaseStock,
    focus: Focus,
}

impl Area for FocusedStock {
    fn id(&self) -> AreaId {
        self.base.id()
    }
}

impl<'a> From<&'a FocusedStock> for Stack<'a> {
    fn from(area: &'a FocusedStock) -> Self {
        as_stack_helper(&area.base, None)
    }
}

impl FocusedArea for FocusedStock {
    type Unfocused = UnfocusedStock;

    fn take_focus(self) -> (Self::Unfocused, Focus) {
        (UnfocusedStock { base: self.base }, self.focus)
    }
}


fn as_stack_helper<'a>(base: &'a BaseStock, focus: Option<&'a Focus>) -> Stack<'a> {
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
