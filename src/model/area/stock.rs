use crate::model::{
    card::Card,
    settings::Settings,
    stack::{Stack, StackDetails, StackSelection},
};

use super::{Action, Area, AreaId, Held, SelectedArea, UnselectedArea};

#[derive(Copy, Clone, Debug)]
pub struct Selection;

#[derive(Debug)]
pub struct Stock<'a, S> {
    cards: Vec<Card>,
    settings: &'a Settings,
    selection: S,
}

pub type UnselectedStock<'a> = Stock<'a, ()>;
pub type SelectedStock<'a> = Stock<'a, Selection>;

impl<'a, S> Stock<'a, S> {
    fn as_stack(&self, mode: Option<Selection>) -> Stack {
        Stack {
            cards: &self.cards,
            details: StackDetails {
                len: self.cards.len(),
                face_up_len: 0,
                visible_len: 2,
                spread_len: 1,
                selection: mode.map(|_| StackSelection::Cards(1)),
            },
        }
    }
}

impl<'a> UnselectedStock<'a> {
    pub fn create<'b>(cards: Vec<Card>, settings: &'a Settings) -> Box<dyn UnselectedArea<'a> + 'b>
    where
        'a: 'b,
    {
        Box::new(Stock {
            cards,
            settings,
            selection: (),
        })
    }
}

impl<'a> Area<'a> for UnselectedStock<'a> {
    fn id(&self) -> AreaId {
        AreaId::Stock
    }

    fn as_stack(&self) -> Stack {
        self.as_stack(None)
    }
}

impl<'a> Area<'a> for SelectedStock<'a> {
    fn id(&self) -> AreaId {
        AreaId::Stock
    }

    fn as_stack(&self) -> Stack {
        self.as_stack(Some(self.selection))
    }
}

impl<'a> UnselectedArea<'a> for UnselectedStock<'a> {
    fn select<'b>(
        self: Box<Self>,
    ) -> Result<Box<dyn SelectedArea<'a> + 'b>, Box<dyn UnselectedArea<'a> + 'b>>
    where
        'a: 'b,
    {
        Ok(Box::new(Stock {
            cards: self.cards,
            settings: self.settings,
            selection: Selection,
        }))
    }

    fn select_with_held<'b>(
        self: Box<Self>,
        held: Held,
    ) -> Result<Box<dyn SelectedArea<'a> + 'b>, (Box<dyn UnselectedArea<'a> + 'b>, Held)>
    where
        'a: 'b,
    {
        Err((self, held))
    }

    fn as_area<'b>(&'b self) -> &'b dyn Area<'a>
    where
        'a: 'b,
    {
        self
    }
}

impl<'a> SelectedArea<'a> for SelectedStock<'a> {
    fn deselect<'b>(self: Box<Self>) -> (Box<dyn UnselectedArea<'a> + 'b>, Option<Held>)
    where
        'a: 'b,
    {
        let unselected = Box::new(Stock {
            cards: self.cards,
            settings: self.settings,
            selection: (),
        });

        (unselected, None)
    }

    fn activate(&mut self) -> Option<Action> {
        if self.cards.is_empty() {
            Some(Action::SendTo {
                area: AreaId::Talon,
                held: Held {
                    source: self.id(),
                    cards: vec![], // TODO: Take from the stock's cards
                },
            })
        } else {
            Some(Action::TakeFrom {
                area: AreaId::Talon,
            })
        }
    }

    fn select_more(&mut self) {}
    fn select_less(&mut self) {}

    fn as_area<'b>(&'b self) -> &'b dyn Area<'a>
    where
        'a: 'b,
    {
        self
    }
}
