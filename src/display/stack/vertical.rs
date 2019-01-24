use crate::display::*;
use crate::display::stack::selector::*;
use crate::display::card::*;
use crate::game::*;

static FAN_OFFSET: Coords = Coords::y(2);
static SHIFT_OFFSET: Coords = Coords::x(1);

static CARD_SELECTOR_OFFSET: Coords = Coords::x(-1);
static STACK_SELECTOR_OFFSET: Coords = Coords::y(1);

pub fn draw_vertical_card_stack(display: &mut KlondikeDisplay, coords: Coords, stack: &Stack) {
    for (i, card) in stack.cards.iter().enumerate() {
        if let Some(coords) = visible_card_coords(coords, i, stack) {
            draw_card(display, coords, card);
        }
    }

    /* Be careful about getting the last index. It's possible for the stack to actually be empty,
     * in which case we can't subtract from a 0 usize. */
    let stack_len = stack.cards.len();
    let end_index =
        if stack_len > 0 {
            stack_len
        } else {
            0
        };

    match stack.selection {
        StackSelection::Cards(_) => {
            let selection_index = stack.selection_index().unwrap_or_default();

            let start_coords =
                card_coords(coords, selection_index, stack)
                    + CARD_SELECTOR_OFFSET;
            let end_coords =
                card_coords(coords, end_index, stack)
                    + CARD_SIZE.to_y()
                    + CARD_SELECTOR_OFFSET;

            let selector_len = end_coords.y - start_coords.y;

            draw_vertical_selector(display, start_coords, selector_len);
        }
        StackSelection::Stack => {
            let start_coords =
                card_coords(coords, end_index, stack)
                    + CARD_SIZE.to_y()
                    + STACK_SELECTOR_OFFSET;

            let selector_len = CARD_SIZE.x;

            draw_horizontal_selector(display, start_coords, selector_len);
        }
        StackSelection::None => {}
    }
}

fn visible_card_coords(base_coords: Coords, index: usize, stack: &Stack) -> Option<Coords> {
    let visible_index = stack.visible_index();

    if index >= visible_index {
        Some(card_coords(base_coords, index, stack))
    } else {
        None
    }
}

fn card_coords(base_coords: Coords, index: usize, stack: &Stack) -> Coords {
    let visible_index = stack.visible_index();

    if index >= visible_index {
        let offset = index - visible_index;
        base_coords
            + (offset as i32) * FAN_OFFSET
            + card_shift(index, stack)
    } else {
        base_coords
    }
}

fn card_shift(index: usize, stack: &Stack) -> Coords {
    stack.selection_index()
        .filter(|&selection_index| index >= selection_index)
        .map(|_| SHIFT_OFFSET)
        .unwrap_or_default()
}
