use super::{card::Card, stack::Stack};

pub mod area_list;
pub mod foundation;
pub mod stock;
pub mod tableaux;
pub mod talon;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
pub enum Action {
    SendTo { area: AreaId, held: Held },
    TakeFrom { area: AreaId },
}

pub trait Area<'a> {
    fn id(&self) -> AreaId;
    fn as_stack<'b>(&'b self) -> Stack<'b>;
}

pub trait UnselectedArea<'a>: Area<'a> {
    fn select<'b>(
        self: Box<Self>,
    ) -> Result<Box<dyn SelectedArea<'a> + 'b>, Box<dyn UnselectedArea<'a> + 'b>>
    where
        'a: 'b;
    fn select_with_held<'b>(
        self: Box<Self>,
        held: Held
    ) -> Result<Box<dyn SelectedArea<'a> + 'b>, (Box<dyn UnselectedArea<'a> + 'b>, Held)>
    where
        'a: 'b;

    fn as_area<'b>(&'b self) -> &'b dyn Area<'a>
    where
        'a: 'b;
}

pub trait SelectedArea<'a>: Area<'a> {
    fn deselect<'b>(self: Box<Self>) -> (Box<dyn UnselectedArea<'a> + 'b>, Option<Held>)
    where
        'a: 'b;

    fn activate(&mut self) -> Option<Action>;
    fn select_more(&mut self);
    fn select_less(&mut self);

    fn as_area<'b>(&'b self) -> &'b dyn Area<'a>
    where
        'a: 'b;
}

pub fn move_selection<'a, 'b>(
    source: Box<dyn SelectedArea<'a> + 'b>,
    target: Box<dyn UnselectedArea<'a> + 'b>,
) -> Result<
    (
        Box<dyn UnselectedArea<'a> + 'b>,
        Box<dyn SelectedArea<'a> + 'b>,
    ),
    (
        Box<dyn SelectedArea<'a> + 'b>,
        Box<dyn UnselectedArea<'a> + 'b>,
    ),
>
where
    'a: 'b,
{
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
                let source_selected = source_unselected.select().ok().expect("Unable to replace selection");
                Err((source_selected, target_unselected))
            }
        }
    }
}
