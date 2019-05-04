use crate::{
    error::Result,
    model::stack::{Orientation, Stack},
};

use super::{bounds::Bounds, coords::Coords};

use self::{horizontal::HorizontalStackPainter, vertical::VerticalStackPainter};

mod common;
mod horizontal;
mod vertical;

pub trait StackPainter {
    fn draw_stack(&mut self, coords: Coords, stack: &Stack) -> Result<Bounds>;
}

impl<T> StackPainter for T
where
    T: HorizontalStackPainter,
    T: VerticalStackPainter,
{
    fn draw_stack(&mut self, coords: Coords, stack: &Stack) -> Result<Bounds> {
        match stack.details.orientation {
            Orientation::Horizontal => self.draw_horizontal_stack(coords, stack),
            Orientation::Vertical => self.draw_vertical_stack(coords, stack),
        }
    }
}
