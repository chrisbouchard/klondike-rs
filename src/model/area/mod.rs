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

pub trait Area {
    fn id(&self) -> AreaId;
    fn as_stack(&self) -> Stack;
}

pub trait UnselectedArea: Area {
    fn select(self: Box<Self>) -> Result<Box<dyn SelectedArea>, Box<dyn UnselectedArea>>;
    fn select_with_held(
        self: Box<Self>,
        held: Held,
    ) -> Result<Box<dyn SelectedArea>, (Box<dyn UnselectedArea>, Held)>;

    fn as_area(&self) -> &dyn Area;
}

pub trait SelectedArea: Area {
    fn deselect(self: Box<Self>) -> (Box<dyn UnselectedArea>, Option<Held>);

    fn activate(&mut self) -> Option<Action>;
    fn select_more(&mut self);
    fn select_less(&mut self);

    fn as_area(&self) -> &dyn Area;
}

pub fn move_selection(
    source: Box<dyn SelectedArea>,
    target: Box<dyn UnselectedArea>,
) -> Result<
    (Box<dyn UnselectedArea>, Box<dyn SelectedArea>),
    (Box<dyn SelectedArea>, Box<dyn UnselectedArea>),
> {
    let (source_unselected, held) = source.deselect();

    if let Some(held) = held {
        match target.select_with_held(held) {
            Ok(target_selected) => Ok((source_unselected, target_selected)),

            Err((target_unselected, held)) => {
                let source_selected = target
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
                let source_selected = target.select().ok().expect("Unable to replace selection");
                Err((source_selected, target_unselected))
            }
        }
    }
}
