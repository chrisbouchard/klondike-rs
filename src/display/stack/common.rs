use crate::{display::Coords, model::stack::StackDetails};

#[derive(Clone, Debug)]
pub struct Offsets {
    pub unspread: Coords,
    pub collapsed_spread: Coords,
    pub uncollapsed_spread: Coords,
    pub selected: Coords,
    pub collapse_unspread_len: usize,
    pub collapse_spread_len: usize,
}

pub fn card_coords(
    base_coords: Coords,
    index: usize,
    offsets: &Offsets,
    stack_details: &StackDetails,
) -> Option<Coords> {
    let visible_index = stack_details.visible_index() + offsets.collapse_unspread_len;
    let collapsed_spread_index = stack_details.spread_index();
    let uncollapsed_spread_index = collapsed_spread_index + offsets.collapse_spread_len;

    if index >= uncollapsed_spread_index {
        let unspread_len = collapsed_spread_index - visible_index;
        let collapsed_spread_len = uncollapsed_spread_index - collapsed_spread_index;
        let uncollapsed_spread_len = index - uncollapsed_spread_index;
        Some(
            base_coords
                + (unspread_len as i32) * offsets.unspread
                + (collapsed_spread_len as i32) * offsets.collapsed_spread
                + (uncollapsed_spread_len as i32) * offsets.uncollapsed_spread
                + card_shift(index, offsets, stack_details),
        )
    } else if index >= collapsed_spread_index {
        let unspread_len = collapsed_spread_index - visible_index;
        let collapsed_spread_len = index - collapsed_spread_index;
        Some(
            base_coords
                + (unspread_len as i32) * offsets.unspread
                + (collapsed_spread_len as i32) * offsets.collapsed_spread
                + card_shift(index, offsets, stack_details),
        )
    } else if index >= visible_index {
        let unspread_len = index - visible_index;
        Some(
            base_coords
                + (unspread_len as i32) * offsets.unspread
                + card_shift(index, offsets, stack_details),
        )
    } else {
        None
    }
}

fn card_shift(index: usize, offsets: &Offsets, stack_details: &StackDetails) -> Coords {
    stack_details
        .selection_index()
        .filter(|&selection_index| index >= selection_index)
        .map(|_| offsets.selected)
        .unwrap_or_default()
}
