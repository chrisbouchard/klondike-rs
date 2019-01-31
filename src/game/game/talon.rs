use crate::game::card::Card;
use crate::game::game::area::{Area, AreaId, Focus};
use crate::game::stack::{Stack, StackDetails, StackSelection};

#[derive(Debug)]
pub struct Talon {
    cards: Vec<Card>,
    fanned_len: usize,
    focus: Option<Focus>,
}

impl Talon {
    pub fn new(cards: Vec<Card>, fanned_len: usize) -> Talon {
        Talon { cards, fanned_len, focus: None }
    }
}

impl Area for Talon {
    fn id(&self) -> AreaId {
        AreaId::Talon
    }

    fn is_focused(&self) -> bool {
        self.focus.is_some()
    }

    fn accepts_focus(&self, focus: &Focus) -> bool {
        if let Some(ref held) = focus.held {
            held.source == AreaId::Talon
        } else {
            !self.base.cards.is_empty()
        }
    }

    fn try_give_focus(&mut self, focus: Focus) -> Result<(), Focus> {
        if self.is_focused() {
            panic!("Duplicated focus!");
        }

        if self.accepts_focus(&focus) {
            self.focus = Some(focus);
            Ok(())
        } else {
            Err(focus)
        }
    }

    fn try_move_focus(&mut self, other: &mut Area) -> bool {
        let focus =
            self.focus.take().expect("Attempting to move focus but no focus present!");

        match other.try_give_focus(focus) {
            Ok(_) => true,
            Err(focus) => {
                self.focus = Some(focus);
                false
            }
        }
    }

    fn as_stack(&self) -> Stack {
        Stack::new(
            &self.cards,
            StackDetails {
                len: self.cards.len(),
                visible_len: self.fanned_len + 1,
                spread_len: self.fanned_len,
                selection: self.focus.map(|_| StackSelection::Cards(1)),
            },
        )
    }
}
