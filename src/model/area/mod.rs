use snafu;
use std::{error, fmt};

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

pub trait Area {
    fn id(&self) -> AreaId;

    fn is_selected(&self) -> bool;
    fn is_held(&self) -> bool;

    fn give_cards(&mut self, held: Held) -> MoveResult<(), Held>;
    fn take_cards(&mut self, len: usize) -> Held;
    fn take_all_cards(&mut self) -> Held;

    fn peek_top_card(&self) -> Option<&Card>;

    fn as_stack(&self) -> Stack;

    fn as_area(&self) -> &dyn Area;
    fn as_area_mut(&mut self) -> &mut dyn Area;
}

pub trait UnselectedArea: Area {
    fn select(self: Box<Self>) -> MoveResult<Box<dyn SelectedArea>, Box<dyn UnselectedArea>>;
    fn select_with_held(
        self: Box<Self>,
        held: Held,
    ) -> MoveResult<Box<dyn SelectedArea>, (Box<dyn UnselectedArea>, Held)>;
}

pub trait SelectedArea: Area {
    fn deselect(self: Box<Self>) -> (Box<dyn UnselectedArea>, Option<Held>);

    fn activate(&mut self) -> Result<Option<Action>>;
    fn pick_up(&mut self) -> Result;
    fn put_down(&mut self) -> Result;
    fn select_more(&mut self) -> Result;
    fn select_less(&mut self) -> Result;

    fn held_from(&self) -> Option<AreaId>;
}

pub struct SelectionMove {
    pub selected: Box<dyn SelectedArea>,
    pub unselected: Box<dyn UnselectedArea>,
}

impl fmt::Debug for SelectionMove {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("SelectionMove")
            .field("selected", &format_args!("<{:?}>", self.selected.id()))
            .field("unselected", &format_args!("<{:?}>", self.unselected.id()))
            .finish()
    }
}

pub fn move_selection<'a>(
    source: Box<dyn SelectedArea>,
    target: Box<dyn UnselectedArea>,
) -> MoveResult<SelectionMove, SelectionMove> {
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
