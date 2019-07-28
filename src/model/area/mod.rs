use snafu;
use std::error;

use super::{
    card::{Card, Suit},
    stack::Stack,
};

pub mod foundation;
pub mod stock;
pub mod tableaux;
pub mod talon;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Invalid card: {}", message))]
    InvalidCard { message: String },

    #[snafu(display("Too many cards: {}", message))]
    TooManyCards { message: String },

    #[snafu(display("Operation not supported: {}", message))]
    NotSupported { message: String },

    #[snafu(display("Nothing to select: {}", message))]
    NothingToSelect { message: String },

    #[snafu(display("Cards already held"))]
    AlreadyHeld,

    #[snafu(display("No cards held"))]
    NothingHeld,

    #[snafu(display("Maximum selection"))]
    MaxSelection,

    #[snafu(display("Minimum selection"))]
    MinSelection,
}

pub type Result<T = (), E = Error> = ::std::result::Result<T, E>;

#[derive(Debug)]
pub enum MoveResult<T, U, E = Error> {
    Moved(T),
    Unmoved(U, E),
}

impl<T, U, E> MoveResult<T, U, E> {
    pub fn into_result(self) -> ::std::result::Result<T, E> {
        match self {
            MoveResult::Moved(value) => Ok(value),
            MoveResult::Unmoved(_, error) => Err(error),
        }
    }
}

trait SnafuSelectorExt<E> {
    fn fail_move<T, U>(self, value: U) -> MoveResult<T, U, E>;
}

impl<S, E> SnafuSelectorExt<E> for S
where
    E: error::Error + snafu::ErrorCompat,
    S: snafu::IntoError<E, Source = snafu::NoneError>,
{
    fn fail_move<T, U>(self, value: U) -> MoveResult<T, U, E> {
        MoveResult::Unmoved(value, self.into_error(snafu::NoneError))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

    fn give_cards(&mut self, held: Held) -> MoveResult<(), Held>;
    fn take_cards(&mut self, len: usize) -> Held;
    fn take_all_cards(&mut self) -> Held;

    fn peek_top_card(&self) -> Option<&Card>;

    fn as_stack(&self) -> Stack;
}

pub trait UnselectedArea<'a>: Area<'a> {
    fn select(
        self: Box<Self>,
    ) -> MoveResult<Box<dyn SelectedArea<'a> + 'a>, Box<dyn UnselectedArea<'a> + 'a>>;
    fn select_with_held(
        self: Box<Self>,
        held: Held,
    ) -> MoveResult<Box<dyn SelectedArea<'a> + 'a>, (Box<dyn UnselectedArea<'a> + 'a>, Held)>;

    fn as_area<'b>(&'b self) -> &'b dyn Area<'a>
    where
        'a: 'b;
    fn as_area_mut<'b>(&'b mut self) -> &'b mut dyn Area<'a>
    where
        'a: 'b;
}

pub trait SelectedArea<'a>: Area<'a> {
    fn deselect(self: Box<Self>) -> (Box<dyn UnselectedArea<'a> + 'a>, Option<Held>);

    fn activate(&mut self) -> Result<Option<Action>>;
    fn pick_up(&mut self) -> Result;
    fn put_down(&mut self) -> Result;
    fn select_more(&mut self) -> Result;
    fn select_less(&mut self) -> Result;

    fn held_from(&self) -> Option<AreaId>;

    fn as_area<'b>(&'b self) -> &'b dyn Area<'a>
    where
        'a: 'b;
    fn as_area_mut<'b>(&'b mut self) -> &'b mut dyn Area<'a>
    where
        'a: 'b;
}

pub struct SelectionMove<'a> {
    pub selected: Box<dyn SelectedArea<'a> + 'a>,
    pub unselected: Box<dyn UnselectedArea<'a> + 'a>,
}

pub fn move_selection<'a>(
    source: Box<dyn SelectedArea<'a> + 'a>,
    target: Box<dyn UnselectedArea<'a> + 'a>,
) -> MoveResult<SelectionMove<'a>, SelectionMove<'a>> {
    let (source_unselected, held) = source.deselect();

    if let Some(held) = held {
        match target.select_with_held(held) {
            MoveResult::Moved(target_selected) => MoveResult::Moved(SelectionMove {
                selected: target_selected,
                unselected: source_unselected,
            }),

            MoveResult::Unmoved((target_unselected, held), error) => {
                let source_selected = source_unselected
                    .select_with_held(held)
                    .into_result()
                    .unwrap();
                MoveResult::Unmoved(
                    SelectionMove {
                        selected: source_selected,
                        unselected: target_unselected,
                    },
                    error,
                )
            }
        }
    } else {
        match target.select() {
            MoveResult::Moved(target_selected) => MoveResult::Moved(SelectionMove {
                selected: target_selected,
                unselected: source_unselected,
            }),

            MoveResult::Unmoved(target_unselected, error) => {
                let source_selected = source_unselected.select().into_result().unwrap();
                MoveResult::Unmoved(
                    SelectionMove {
                        selected: source_selected,
                        unselected: target_unselected,
                    },
                    error,
                )
            }
        }
    }
}
