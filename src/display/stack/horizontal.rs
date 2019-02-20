use crate::{
    display::{
        card::{CardPainter, CARD_SIZE},
        coords::Coords,
        selector::SelectorPainter,
        Result,
    },
    model::stack::Stack,
    utils::usize::BoundedSub,
};

use super::common::*;

static OFFSETS: Offsets = Offsets {
    unspread: Coords::from_x(1),
    spread: Coords::from_x(4),
    selected: Coords::from_x(1),
};

static SELECTOR_OFFSET: Coords = Coords::from_y(0);

pub trait HorizontalStackPainter {
    fn draw_horizontal_card_stack(&mut self, coords: Coords, stack: &Stack) -> Result;
}

impl<T> HorizontalStackPainter for T
where
    T: CardPainter + SelectorPainter,
{
    fn draw_horizontal_card_stack(&mut self, coords: Coords, stack: &Stack) -> Result {
        let face_up_index = stack.details.face_up_index();

        for (i, card) in stack.into_iter().enumerate() {
            if let Some(coords) = card_coords(coords, i, &OFFSETS, &stack.details) {
                if i < face_up_index {
                    self.draw_card_face_down(coords, card)?;
                } else {
                    self.draw_card_face_up(coords, card)?;
                }
            }
        }

        if stack.details.selection.is_some() {
            let selection_index = stack.details.selection_index().unwrap_or_default();

            debug!("selection_index: {}", selection_index);

            /* Be careful about getting the last index. It's possible for the stack to actually be empty,
             * in which case we can't subtract from a 0 usize. */
            let end_index = stack.details.len.bounded_sub(1);

            let start_coords = card_coords(coords, selection_index, &OFFSETS, &stack.details)
                .unwrap_or(coords)
                + CARD_SIZE.to_y()
                + SELECTOR_OFFSET;
            let end_coords = card_coords(coords, end_index, &OFFSETS, &stack.details)
                .unwrap_or(coords)
                + CARD_SIZE
                + SELECTOR_OFFSET;

            let selector_len = end_coords.x - start_coords.x;

            debug!("start_coords: {:?}", start_coords);
            debug!("end_coords: {:?}", end_coords);
            debug!("selector_len: {:?}", selector_len);

            self.draw_horizontal_selector(start_coords, selector_len)?;
        }

        Ok(())
    }
}
