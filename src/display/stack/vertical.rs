use crate::display::card::*;
use crate::display::coords::*;
use crate::display::selector::*;
use crate::display::stack::common::*;
use crate::game::stack::*;

static OFFSETS: Offsets = Offsets {
    unspread: Coords::from_y(1),
    spread: Coords::from_y(2),
    selected: Coords::from_x(1),
};

static CARD_SELECTOR_OFFSET: Coords = Coords::from_x(-2);
static STACK_SELECTOR_OFFSET: Coords = Coords::from_y(0);

pub trait VerticalStackPainter {
    fn draw_vertical_card_stack(&mut self, coords: Coords, stack: &Stack);
}

impl<T> VerticalStackPainter for T where T: CardPainter + SelectorPainter {
    fn draw_vertical_card_stack(&mut self, coords: Coords, stack: &Stack) {
        for (i, card) in stack.iter().enumerate() {
            if let Some(coords) = card_coords(coords, i, &OFFSETS, stack) {
                self.draw_card(coords, card);
            }
        }

        /* Be careful about getting the last index. It's possible for the stack to actually be empty,
         * in which case we can't subtract from a 0 usize. */
        let stack_len = stack.len();
        let end_index =
            if stack_len > 0 {
                stack_len - 1
            } else {
                0
            };

        if let Some(selection) = stack.selection {
            match selection {
                StackSelection::Cards(_) => {
                    let selection_index = stack.selection_index().unwrap_or_default();

                    let start_coords =
                        card_coords(coords, selection_index, &OFFSETS, stack).unwrap_or(coords)
                            + CARD_SELECTOR_OFFSET;
                    let end_coords =
                        card_coords(coords, end_index, &OFFSETS, stack).unwrap_or(coords)
                            + CARD_SIZE.to_y()
                            + CARD_SELECTOR_OFFSET;

                    let selector_len = end_coords.y - start_coords.y;

                    self.draw_vertical_selector(start_coords, selector_len);
                }
                StackSelection::Stack(_) | StackSelection::FullStack => {
                    let start_coords =
                        card_coords(coords, end_index, &OFFSETS, stack).unwrap_or(coords)
                            + CARD_SIZE.to_y()
                            + STACK_SELECTOR_OFFSET;

                    let selector_len = CARD_SIZE.x;

                    self.draw_horizontal_selector(start_coords, selector_len);
                }
            }
        }
    }
}
