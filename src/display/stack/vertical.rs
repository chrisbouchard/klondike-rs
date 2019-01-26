use crate::display::*;
use crate::display::card::*;
use crate::display::selector::*;
use crate::display::stack::*;
use crate::game::stack::*;

static OFFSETS: Offsets = Offsets {
    unspread: Coords::y(1),
    spread: Coords::y(2),
    selected: Coords::x(1),
};

static CARD_SELECTOR_OFFSET: Coords = Coords::x(-1);
static STACK_SELECTOR_OFFSET: Coords = Coords::y(1);

pub fn draw_vertical_card_stack(display: &mut KlondikeDisplay, coords: Coords, stack: &Stack) {
    for (i, card) in stack.iter().enumerate() {
        if let Some(coords) = card_coords(coords, i, &OFFSETS, stack) {
            draw_card(display, coords, card);
        }
    }

    /* Be careful about getting the last index. It's possible for the stack to actually be empty,
     * in which case we can't subtract from a 0 usize. */
    let stack_len = stack.len();
    let end_index =
        if stack_len > 0 {
            stack_len
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

                draw_vertical_selector(display, start_coords, selector_len);
            }
            StackSelection::Stack(_) | StackSelection::FullStack => {
                let start_coords =
                    card_coords(coords, end_index, &OFFSETS, stack).unwrap_or(coords)
                        + CARD_SIZE.to_y()
                        + STACK_SELECTOR_OFFSET;

                let selector_len = CARD_SIZE.x;

                draw_horizontal_selector(display, start_coords, selector_len);
            }
        }
    }
}
