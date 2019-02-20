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
pub enum SelectionMode {
    Free { len: usize },
    Held { len: usize, source: AreaId },
}

impl SelectionMode {
    pub fn is_free(&self) -> bool {
        match self {
            SelectionMode::Free { .. } => true,
            _ => false,
        }
    }

    pub fn is_held(&self) -> bool {
        match self {
            SelectionMode::Held { .. } => true,
            _ => false,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            SelectionMode::Free { len, .. } => *len,
            SelectionMode::Held { len, .. } => *len,
        }
    }

    pub fn moved(self) -> SelectionMode {
        match self {
            SelectionMode::Free { .. } => Self::default(),
            SelectionMode::Held { .. } => self,
        }
    }
}

impl Default for SelectionMode {
    fn default() -> Self {
        SelectionMode::Free { len: 1 }
    }
}

#[derive(Debug)]
pub struct Selection {
    pub target: AreaId,
    pub mode: SelectionMode,
}

impl Selection {
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

impl Default for Selection {
    fn default() -> Self {
        Selection {
            target: AreaId::Stock,
            mode: SelectionMode::default(),
        }
    }
}

pub enum Action {
    Default,
    Draw,
    FlipOver,
    Refresh,
}

pub trait Area {
    fn id(&self) -> AreaId;

    fn accepts_cards(&self, cards: &Vec<Card>) -> bool;
    fn accepts_selection(&self, mode: &SelectionMode) -> bool;

    fn offer_cards(&mut self, cards: Vec<Card>) -> Result<(), Vec<Card>> {
        if self.accepts_cards(&cards) {
            self.place_cards(cards);
            Ok(())
        } else {
            Err(cards)
        }
    }

    fn place_cards(&mut self, cards: Vec<Card>);

    fn replace_cards(&mut self, cards: Vec<Card>) {
        self.place_cards(cards);
    }

    fn take_cards(&mut self, len: usize) -> Vec<Card>;

    fn activate(&self, mode: &SelectionMode) -> Action {
        Action::Default
    }

    fn as_stack<'a>(&'a self, mode: Option<&'a SelectionMode>) -> Stack<'a>;
}
