use crate::game::card::Card;
use crate::game::game::area::{Area, AreaId, Focus, Held};
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

    fn into_focus(self) -> (Focus, usize) {
        match self {
            TableauxFocus::Held(held) => (Focus { held: Some(held) }, 1),
            TableauxFocus::NoHeld(len) => (Focus { held: None }, len),
        }
    }
}


#[derive(Debug)]
pub struct Tableaux {
    index: usize,
    cards: Vec<Card>,
    revealed_len: usize,
    focus: Option<TableauxFocus>,
}

impl Tableaux {
    pub fn new(index: usize, cards: Vec<Card>) -> Tableaux {
        let revealed_index = cards.iter()
            .position(|(i, card)| card.face_up)
            .unwrap_or_default();
        let revealed_len = cards.len() - revealed_index;

        Tableaux {
            index,
            cards,
            revealed_len,
            focus: None,
        }
    }
}

impl Area for Tableaux {
    fn id(&self) -> AreaId {
        AreaId::Tableaux(self.base.index)
    }

    fn is_focused(&self) -> bool {
        self.focus.is_some()
    }

    fn accepts_focus(&self, focus: &Focus) -> bool {
        if let Some(ref held) = focus.held {
            if let Some(card) = held.cards.first() {
                match self.cards.last() {
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

    fn try_give_focus(&mut self, focus: Focus) -> Result<(), Focus> {
        if self.is_focused() {
            panic!("Duplicated focus!");
        }

        if self.accepts_focus(&focus) {
            self.focus = Some(TableauxFocus::from_focus(focus));
            Ok(())
        } else {
            Err(focus)
        }
    }

    fn try_move_focus(&mut self, other: &mut Area) -> bool {
        let tableaux_focus =
            self.focus.take().expect("Attempting to move focus but no focus present!");
        let (focus, len) = tableaux_focus.into_focus();

        match other.try_give_focus(focus) {
            Ok(_) => true,
            Err(focus) => {
                self.focus = Some(TableauxFocus::from_focus_with_len(focus, len));
                false
            }
        }
    }

    fn as_stack(&self) -> Stack {
        Stack::new(
            &self.cards,
            StackDetails {
                len: self.cards.len(),
                visible_len: self.cards.len(),
                spread_len: self.revealed_len,
                selection: self.focus.map(|ref focus| {
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
}
