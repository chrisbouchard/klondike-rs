use crate::display::*;
use crate::display::stack::selector::*;
use crate::display::card::*;
use crate::game::*;

static FAN_OFFSET: Coords = Coords::x(4);
static PILE_OFFSET: Coords = Coords::x(1);
static SHIFT_OFFSET: Coords = Coords::x(1);

static SELECTOR_OFFSET: Coords = Coords::y(0);

pub fn draw_horizontal_card_stack(display: &mut KlondikeDisplay, coords: Coords, stack: &Stack) {
    for (i, card) in stack.iter().enumerate() {
        if let Some(coords) = visible_card_coords(coords, i, stack) {
            draw_card(display, coords, card);
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
            card_coords(coords, selection_index, stack)
                + CARD_SIZE.to_y()
                + SELECTOR_OFFSET;
        let end_coords =
            card_coords(coords, end_index, stack)
                + CARD_SIZE
                + SELECTOR_OFFSET;

        debug!("start_coords: {:?}", start_coords);
        debug!("end_coords: {:?}", end_coords);

        let selector_len = end_coords.x - start_coords.x;

        draw_horizontal_selector(display, start_coords, selector_len);
    }
}

fn visible_card_coords(base_coords: Coords, index: usize, stack: &Stack) -> Option<Coords> {
    let visible_index = stack.visible_index();

    /* Shift the index by one place, to account for the card "in the pile" if not all cards are
     * visible. */
    if index + 1 >= visible_index {
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
    let selection_shift =
        stack.selection_index()
            .filter(|&selection_index| index >= selection_index)
            .map(|_| SHIFT_OFFSET)
            .unwrap_or_default();

    let pile_shift =
        if stack.visible_index() > 0 {
            PILE_OFFSET
        } else {
            Coords::default()
        };

    selection_shift + pile_shift
}
