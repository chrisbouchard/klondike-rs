use crate::game::card::Card;
use crate::game::game::area::{Area, AreaId, Focus, FocusedArea, Held, UnfocusedArea};
use crate::game::stack::{Stack, StackDetails, StackSelection};

#[derive(Debug)]
pub enum TableauxFocus {
    Held(Held),
    NoHeld(usize),
}

impl TableauxFocus {
    fn from_focus(focus: Focus) -> TableauxFocus {
        TableauxFocus::from_focus_with_len(focus, 1)
    }

    fn from_focus_with_len(focus: Focus, len: usize) -> TableauxFocus {
        focus.held
            .map(|held| TableauxFocus::Held(held))
            .unwrap_or(TableauxFocus::NoHeld(len))
    }

    fn to_focus(self) -> (Focus, usize) {
        match self {
            TableauxFocus::Held(held) => (Focus { held: Some(held) }, 1),
            TableauxFocus::NoHeld(len) => (Focus { held: None }, len),
        }
    }
}


#[derive(Debug)]
pub struct Tableaux<F = ()> {
    index: usize,
    cards: Vec<Card>,
    revealed_len: usize,
    focus: F,
}

pub type FocusedTableaux = Tableaux<TableauxFocus>;

impl<F> Tableaux<F> {
    pub fn new(index: usize, cards: Vec<Card>) -> Tableaux {
        let revealed_index = cards.iter()
            .position(|(i, card)| card.face_up)
            .unwrap_or_default();
        let revealed_len = cards.len() - revealed_index;

        Tableaux {
            index,
            cards,
            revealed_len,
            focus: (),
        }
    }

    fn as_stack_helper<'a>(&'a self, focus: Option<&'a TableauxFocus>) -> Stack<'a> {
        Stack::new(
            &self.cards,
            StackDetails {
                len: self.cards.len(),
                visible_len: self.cards.len(),
                spread_len: self.revealed_len,
                selection: focus.map(|focus| {
                    match focus {
                        TableauxFocus::Held(Held { cards, .. }) => {
                            StackSelection::Stack(cards.len())
                        }
                        TableauxFocus::NoHeld(len) => {
                            StackSelection::Cards(*len)
                        }
                    }
                }),
            },
        )
    }

    fn take_focus_unsafe(self) -> (Tableaux, F) {
        (self.with_focus_unsafe(()), self.focus)
    }

    fn with_focus_unsafe<G>(self, focus: G) -> Tableaux<G> {
        Tableaux {
            index: self.index,
            cards: self.cards,
            revealed_len: self.revealed_len,
            focus,
        }
    }
}

impl Area for Tableaux {
    fn id(&self) -> AreaId {
        AreaId::Tableaux(self.base.index)
    }

    fn as_stack(&self) -> Stack {
        self.as_stack_helper(None)
    }
}

impl Area for FocusedTableaux {
    fn id(&self) -> AreaId {
        AreaId::Tableaux(self.base.index)
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

impl UnfocusedArea for Tableaux {
    type Focused = FocusedTableaux;

    fn accepts_focus(&self, focus: &Focus) -> bool {
        if let Some(ref held) = focus.held {
            if let Some(card) = held.cards.first() {
                match self.base.cards.last() {
                    Some(tableaux_card) =>
                        tableaux_card.face_up
                            && tableaux_card.rank.is_followed_by(card.rank)
                            && tableaux_card.color() != card.color(),
                    None =>
                        card.rank.is_king(),
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
            let tableaux_focus = TableauxFocus::from_focus(focus);
            Ok(self.with_focus_unsafe(tableaux_focus))
        } else {
            Err((self, focus))
        }
    }
}

impl FocusedArea for FocusedTableaux {
    type Unfocused = Tableaux;

    fn try_move_focus<A>(self, other: A) -> Result<(Self::Unfocused, A::Focused), (Self, A)>
        where Self: Sized, Self::Unfocused: Sized, A: UnfocusedArea, A::Focused: Sized {
        let (tableaux, tableaux_focus) = self.take_focus_unsafe();
        let (focus, len) = tableaux_focus.to_focus();

        match other.try_give_focus(focus) {
            Ok(other) => {
                Ok((tableaux, other))
            }
            Err((unfocused_area, focus)) => {
                let tableaux_focus = TableauxFocus::from_focus_with_len(focus, len);
                let tableaux = tableaux.with_focus_unsafe(tableaux_focus);
                Err((tableaux, other))
            }
        }
    }
}
