pub mod foundation;
pub mod stock;
pub mod tableaux;
pub mod talon;

use super::{card::Card, stack::Stack};

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
pub enum SelectionMode {
    Cards(usize),
    Held(Held),
}

impl SelectionMode {
    pub const fn new() -> SelectionMode {
        SelectionMode::Cards(1)
    }

    pub fn moved_ref(&self) -> &SelectionMode {
        static DEFAULT: SelectionMode = SelectionMode::new();

        match self {
            SelectionMode::Cards(_) => &DEFAULT,
            SelectionMode::Held(_) => self,
        }
    }

    pub fn moved(self) -> SelectionMode {
        match self {
            SelectionMode::Cards(_) => SelectionMode::new(),
            SelectionMode::Held(_) => self,
        }
    }
}

#[derive(Debug)]
pub struct Selection {
    pub target: AreaId,
    pub mode: SelectionMode,
}

impl Selection {
    pub const fn new() -> Selection {
        Selection {
            target: AreaId::Stock,
            mode: SelectionMode::new(),
        }
    }

    pub fn move_to(mut self, area_id: AreaId) -> Selection {
        self.target = area_id;
        self.mode = self.mode.moved();
        self
    }

    pub fn select(mut self, mode: SelectionMode) -> Selection {
        self.mode = mode;
        self
    }

    pub fn matches(&self, area_id: AreaId) -> bool {
        self.target == area_id
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Action {
    Draw,
    MoveTo(AreaId),
    Restock,
}

pub trait Area {
    fn id(&self) -> AreaId;

    fn accepts_focus(&self, mode: &SelectionMode) -> bool;
    fn activate(&mut self, mode: &mut SelectionMode) -> Option<Action>;

    fn as_stack<'a>(&'a self, mode: Option<&'a SelectionMode>) -> Stack<'a>;
}
