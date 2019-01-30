use crate::game::card::*;
use crate::game::stack::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AreaId {
    Stock,
    Talon,
    Foundation(usize),
    Tableaux(usize),
}


#[derive(Debug)]
pub struct Held {
    pub source: AreaId,
    pub cards: Vec<Card>,
}

#[derive(Debug)]
pub struct Focus {
    pub held: Option<Held>
}


pub trait Area {
    fn id(&self) -> AreaId;
    fn as_stack(&self) -> Stack;
}

pub trait UnfocusedArea: Area {
    type Focused: Area + ?Sized;
    fn accepts_focus(&self, focus: &Focus) -> bool;
    fn try_give_focus(self, focus: Focus) -> Result<Self::Focused, (Self, Focus)>
        where Self: Sized, Self::Focused: Sized;
}

pub trait FocusedArea: Area {
    type Unfocused: Area + ?Sized;
    fn try_move_focus<A>(self, other: A) -> Result<(Self::Unfocused, A::Focused), (Self, A)>
        where Self: Sized, Self::Unfocused: Sized, A: UnfocusedArea, A::Focused: Sized;
}


#[derive(Debug)]
pub enum FocusBox<A> where A: UnfocusedArea, A::Focused: FocusedArea<Unfocused=A> + Sized {
    Unfocused(A),
    Focused(A::Focused),
}

impl<A> FocusBox<A> where A: UnfocusedArea, A::Focused: FocusedArea<Unfocused=A> + Sized {
    pub fn if_focused(&self) -> Option<&FocusedArea<Unfocused=A>> {
        match self {
            FocusBox::Unfocused(_) => None,
            FocusBox::Focused(area) => Some(area)
        }
    }
}

impl<A> Area for FocusBox<A> where A: UnfocusedArea, A::Focused: FocusedArea<Unfocused=A> + Sized {
    fn id(&self) -> AreaId {
        match self {
            FocusBox::Unfocused(area) => area.id(),
            FocusBox::Focused(area) => area.id(),
        }
    }

    fn as_stack(&self) -> Stack {
        match self {
            FocusBox::Unfocused(area) => area.as_stack(),
            FocusBox::Focused(area) => area.as_stack(),
        }
    }
}

impl<A1> FocusBox<A1> where A1: UnfocusedArea, A1::Focused: FocusedArea<Unfocused=A1> + Sized {
    fn try_move_focus<A2>(&mut self, target: &mut FocusBox<A2>) -> bool
        where A2: UnfocusedArea, A2::Focused: FocusedArea<Unfocused=A2> + Sized {
        if let (FocusBox::Focused(focused_area), FocusBox::Unfocused(unfocused_area)) = (self, target) {
            match focused_area.try_move_focus(*unfocused_area) {
                Ok((unfocused_area, focused_area)) => {
                    *self = FocusBox::Unfocused(unfocused_area);
                    *target = FocusBox::Focused(focused_area);
                    true
                }
                Err((focused_area, unfocused_area)) => {
                    *self = FocusBox::Focused(focused_area);
                    *target = FocusBox::Unfocused(unfocused_area);
                    false
                }
            }
        } else {
            false
        }
    }
}
