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

    fn is_focused(&self) -> bool;
    fn accepts_focus(&self, focus: &Focus) -> bool;

    fn try_give_focus(&mut self, focus: Focus) -> Result<(), Focus>;
    fn try_move_focus(&mut self, other: &mut Area) -> bool;

    fn as_stack(&self) -> Stack;

    fn if_focused(&self) -> Option<&Area> where Self: Sized {
        if self.is_focused() {
            Some(self)
        } else {
            None
        }
    }

    fn if_focused_mut(&mut self) -> Option<&mut Area> where Self: Sized {
        if self.is_focused() {
            Some(self)
        } else {
            None
        }
    }
}
