use crate::game::card::Card;
use crate::game::game::area::{Area, AreaId, Focus};
use crate::game::stack::{Stack, StackDetails, StackSelection};

#[derive(Debug)]
pub struct Stock {
    cards: Vec<Card>,
    focus: Option<Focus>,
}

impl Stock {
    pub fn new(cards: Vec<Card>) -> Stock {
        Stock { cards, focus: None }
    }
}

impl Area for Stock {
    fn id(&self) -> AreaId {
        AreaId::Stock
    }

    fn is_focused(&self) -> bool {
        self.focus.is_some()
    }

    fn accepts_focus(&self, focus: &Focus) -> bool {
        focus.held.is_none()
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
                visible_len: 2,
                spread_len: 1,
                selection: self.focus.as_ref().map(|_| StackSelection::Cards(1)),
            },
        )
    }
}
