use super::{
    card::{Card, Suit},
    stack::Stack,
};

pub mod area_list;
pub mod foundation;
pub mod stock;
pub mod tableaux;
pub mod talon;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum AreaId {
    Stock,
    Talon,
    Foundation(Suit),
    Tableaux(u8),
}

#[derive(Debug)]
pub struct Held {
    pub source: AreaId,
    pub cards: Vec<Card>,
}

#[derive(Debug)]
pub enum Action {
    Draw(usize),
    Restock,
}

pub trait Area<'a> {
    fn id(&self) -> AreaId;

    fn give_cards(&mut self, held: Held) -> Result<(), Held>;
    fn take_cards(&mut self, len: usize) -> Held;
    fn take_all_cards(&mut self) -> Held;

    fn peek_top_card(&self) -> Option<&Card>;

    fn as_stack(&self) -> Stack;
}

pub trait UnselectedArea<'a>: Area<'a> {
    fn select(
        self: Box<Self>,
    ) -> Result<Box<dyn SelectedArea<'a> + 'a>, Box<dyn UnselectedArea<'a> + 'a>>;
    fn select_with_held(
        self: Box<Self>,
        held: Held,
    ) -> Result<Box<dyn SelectedArea<'a> + 'a>, (Box<dyn UnselectedArea<'a> + 'a>, Held)>;

    fn as_area<'b>(&'b self) -> &'b dyn Area<'a>
    where
        'a: 'b;
    fn as_area_mut<'b>(&'b mut self) -> &'b mut dyn Area<'a>
    where
        'a: 'b;
}

pub trait SelectedArea<'a>: Area<'a> {
    fn deselect(self: Box<Self>) -> (Box<dyn UnselectedArea<'a> + 'a>, Option<Held>);

    fn activate(&mut self) -> Option<Action>;
    fn pick_up(&mut self);
    fn put_down(&mut self);
    fn select_more(&mut self);
    fn select_less(&mut self);

    fn held_from(&self) -> Option<AreaId>;

    fn as_area<'b>(&'b self) -> &'b dyn Area<'a>
    where
        'a: 'b;
    fn as_area_mut<'b>(&'b mut self) -> &'b mut dyn Area<'a>
    where
        'a: 'b;
}

pub type SuccessfulMove<'a> = (
    Box<dyn UnselectedArea<'a> + 'a>,
    Box<dyn SelectedArea<'a> + 'a>,
);

pub type UnsuccessfulMove<'a> = (
    Box<dyn SelectedArea<'a> + 'a>,
    Box<dyn UnselectedArea<'a> + 'a>,
);

pub fn move_selection<'a>(
    source: Box<dyn SelectedArea<'a> + 'a>,
    target: Box<dyn UnselectedArea<'a> + 'a>,
) -> Result<SuccessfulMove<'a>, UnsuccessfulMove<'a>> {
    let (source_unselected, held) = source.deselect();

    if let Some(held) = held {
        match target.select_with_held(held) {
            Ok(target_selected) => Ok((source_unselected, target_selected)),

            Err((target_unselected, held)) => {
                let source_selected = source_unselected
                    .select_with_held(held)
                    .ok()
                    .expect("Unable to replace selection with held cards");
                Err((source_selected, target_unselected))
            }
        }
    } else {
        match target.select() {
            Ok(target_selected) => Ok((source_unselected, target_selected)),
            Err(target_unselected) => {
                let source_selected = source_unselected
                    .select()
                    .ok()
                    .expect("Unable to replace selection");
                Err((source_selected, target_unselected))
            }
        }
    }
}
