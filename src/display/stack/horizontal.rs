use crate::display::card::*;
use crate::display::coords::*;
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
    fn draw_horizontal_card_stack(&mut self, coords: Coords, stack: &Stack);
}

impl<T> HorizontalStackPainter for T where T: CardPainter + SelectorPainter {
    fn draw_horizontal_card_stack(&mut self, coords: Coords, stack: &Stack) {
        for (i, card) in stack.iter().enumerate() {
            if let Some(coords) = card_coords(coords, i, &OFFSETS, stack) {
                self.draw_card(coords, card);
            }
        }

        if stack.selection.is_some() {
            let selection_index = stack.selection_index().unwrap_or_default();

            debug!("selection_index: {}", selection_index);

            /* Be careful about getting the last index. It's possible for the stack to actually be empty,
         * in which case we can't subtract from a 0 usize. */
            let stack_len = stack.len();
            let end_index =
                if stack_len > 0 {
                    stack_len - 1
                } else {
                    0
                };

            let start_coords =
                card_coords(coords, selection_index, &OFFSETS, stack).unwrap_or(coords)
                    + CARD_SIZE.to_y()
                    + SELECTOR_OFFSET;
            let end_coords =
                card_coords(coords, end_index, &OFFSETS, stack).unwrap_or(coords)
                    + CARD_SIZE
                    + SELECTOR_OFFSET;

            let selector_len = end_coords.x - start_coords.x;

            debug!("start_coords: {:?}", start_coords);
            debug!("end_coords: {:?}", end_coords);
            debug!("selector_len: {:?}", selector_len);

            self.draw_horizontal_selector(start_coords, selector_len);
        }
    }
}
