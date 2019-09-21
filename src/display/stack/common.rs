use num_traits::ToPrimitive;

use crate::{
    display::geometry,
    model::{stack::StackDetails, Card},
};

use super::StackWidget;

#[derive(Clone, Debug)]
pub struct Offsets {
    pub unspread: geometry::Vector2D<i16>,
    pub collapsed_spread: geometry::Vector2D<i16>,
    pub uncollapsed_spread: geometry::Vector2D<i16>,
    pub selected: geometry::Vector2D<i16>,
    pub collapse_unspread_len: usize,
    pub collapse_spread_len: usize,
}

pub fn card_iter<'a>(
    widget: &'a StackWidget<'a>,
    offsets: &'a Offsets,
) -> impl Iterator<Item = (usize, geometry::Point2D<u16>, &'a Card)> {
    let coords = widget.bounds.origin;

    widget
        .stack
        .into_iter()
        .enumerate()
        .filter_map(move |(index, card)| {
            card_coords(coords, index, offsets, &widget.stack.details)
                .map(|coords| (index, coords, card))
        })
}

pub fn card_coords(
    origin: geometry::Point2D<u16>,
    index: usize,
    offsets: &Offsets,
    stack_details: &StackDetails,
) -> Option<geometry::Point2D<u16>> {
    let visible_index = stack_details.visible_index() + offsets.collapse_unspread_len;
    let collapsed_spread_index = stack_details.spread_index();
    let uncollapsed_spread_index = collapsed_spread_index + offsets.collapse_spread_len;

    if index >= uncollapsed_spread_index {
        let unspread_len = (collapsed_spread_index - visible_index).to_i16().unwrap();
        let collapsed_spread_len = (uncollapsed_spread_index - collapsed_spread_index)
            .to_i16()
            .unwrap();
        let uncollapsed_spread_len = (index - uncollapsed_spread_index).to_i16().unwrap();
        Some(
            (origin.cast::<i16>()
                + offsets.unspread * unspread_len
                + offsets.collapsed_spread * collapsed_spread_len
                + offsets.uncollapsed_spread * uncollapsed_spread_len
                + card_shift(index, offsets, stack_details))
            .cast::<u16>(),
        )
    } else if index >= collapsed_spread_index {
        let unspread_len = (collapsed_spread_index - visible_index).to_i16().unwrap();
        let collapsed_spread_len = (index - collapsed_spread_index).to_i16().unwrap();
        Some(
            (origin.cast::<i16>()
                + offsets.unspread * unspread_len
                + offsets.collapsed_spread * collapsed_spread_len
                + card_shift(index, offsets, stack_details))
            .cast::<u16>(),
        )
    } else if index >= visible_index {
        let unspread_len = (index - visible_index).to_i16().unwrap();
        Some(
            (origin.cast::<i16>()
                + offsets.unspread * unspread_len
                + card_shift(index, offsets, stack_details))
            .cast::<u16>(),
        )
    } else {
        None
    }
}

fn card_shift(
    index: usize,
    offsets: &Offsets,
    stack_details: &StackDetails,
) -> geometry::Vector2D<i16> {
    stack_details
        .selection_index()
        .filter(|_| stack_details.held())
        .filter(|&selection_index| index >= selection_index)
        .map(|_| offsets.selected)
        .unwrap_or_default()
}
