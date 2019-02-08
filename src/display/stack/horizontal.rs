use crate::display::card::*;
use crate::display::coords::*;
use crate::display::Result;
use crate::display::selector::*;
use crate::display::stack::common::*;
use crate::game::stack::*;

static OFFSETS: Offsets = Offsets {
    unspread: Coords::from_x(1),
    spread: Coords::from_x(4),
    selected: Coords::from_x(1),
};

static SELECTOR_OFFSET: Coords = Coords::from_y(0);

pub trait HorizontalStackPainter {
    fn draw_horizontal_card_stack(&mut self, coords: Coords, stack: &Stack) -> Result;
}

impl<T> HorizontalStackPainter for T where T: CardPainter + SelectorPainter {
    fn draw_horizontal_card_stack(&mut self, coords: Coords, stack: &Stack) -> Result {
        let stack_details = stack.details();

        for (i, card) in stack.into_iter().enumerate() {
            if let Some(coords) = card_coords(coords, i, &OFFSETS, stack_details) {
                self.draw_card(coords, card)?;
            }
        }

        if stack_details.selection.is_some() {
            let selection_index = stack_details.selection_index().unwrap_or_default();

            debug!("selection_index: {}", selection_index);

            /* Be careful about getting the last index. It's possible for the stack to actually be empty,
         * in which case we can't subtract from a 0 usize. */
            let stack_len = stack_details.len;
            let end_index =
                if stack_len > 0 {
                    stack_len - 1
                } else {
                    0
                };

            let start_coords =
                card_coords(coords, selection_index, &OFFSETS, stack_details).unwrap_or(coords)
                    + CARD_SIZE.to_y()
                    + SELECTOR_OFFSET;
            let end_coords =
                card_coords(coords, end_index, &OFFSETS, stack_details).unwrap_or(coords)
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
