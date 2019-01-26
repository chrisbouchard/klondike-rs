use crate::display::coords::*;
use crate::game::stack::*;

#[derive(Debug)]
pub struct Offsets {
    pub unspread: Coords,
    pub spread: Coords,
    pub selected: Coords,
}

pub fn card_coords(base_coords: Coords, index: usize, offsets: &Offsets, stack: &Stack) -> Option<Coords> {
    let visible_index = stack.visible_index();
    let spread_index = stack.spread_index();

    if index >= spread_index {
        let unspread_len = spread_index - visible_index;
        let spread_len = index - spread_index;
        Some(
            base_coords
                + (unspread_len as i32) * offsets.unspread
                + (spread_len as i32) * offsets.spread
                + card_shift(index, offsets, stack)
        )
    } else if index >= visible_index {
        let unspread_len = index - visible_index;
        Some(
            base_coords
                + (unspread_len as i32) * offsets.unspread
                + card_shift(index, offsets, stack)
        )
    } else {
        None
    }
}

pub fn card_shift(index: usize, offsets: &Offsets, stack: &Stack) -> Coords {
    stack.selection_index()
        .filter(|&selection_index| index >= selection_index)
        .map(|_| offsets.selected)
        .unwrap_or_default()
}
