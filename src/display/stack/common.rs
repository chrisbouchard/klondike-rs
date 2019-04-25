use crate::{display::Coords, model::stack::StackDetails};

#[derive(Debug)]
pub struct Offsets {
    pub unspread: Coords,
    pub spread: Coords,
    pub selected: Coords,
}

pub fn card_coords(
    base_coords: Coords,
    index: usize,
    offsets: &Offsets,
    stack_details: &StackDetails,
) -> Option<Coords> {
    let visible_index = stack_details.visible_index();
    let spread_index = stack_details.spread_index();

    if index >= spread_index {
        let unspread_len = spread_index - visible_index;
        let spread_len = index - spread_index;
        Some(
            base_coords
                + (unspread_len as i32) * offsets.unspread
                + (spread_len as i32) * offsets.spread
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

pub fn card_shift(index: usize, offsets: &Offsets, stack_details: &StackDetails) -> Coords {
    stack_details
        .selection_index()
        .filter(|&selection_index| index >= selection_index)
        .map(|_| offsets.selected)
        .unwrap_or_default()
}
