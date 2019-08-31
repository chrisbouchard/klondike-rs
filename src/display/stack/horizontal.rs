use std::convert::TryFrom;

use crate::{
    display::{
        bounds::Bounds,
        card::{CardWidget, CardWidgetMode, CARD_SIZE},
        coords::Coords,
        selector::SelectorWidget,
    },
    model::stack::Stack,
    utils::usize::BoundedSub,
};

use super::{common::*, Offsets, StackWidget};

static OFFSETS: Offsets = Offsets {
    unspread: Coords::from_x(1),
    collapsed_spread: Coords::from_x(4),
    uncollapsed_spread: Coords::from_x(4),
    selected: Coords::from_x(1),
    collapse_unspread_len: 0,
    collapse_spread_len: 0,
};

static SELECTOR_OFFSET: Coords = Coords::from_y(0);

pub fn offsets(display: &StackWidget) -> Offsets {
    OFFSETS
}

pub fn card_widget_iter<'a>(
    display: &'a StackWidget,
    offsets: &'a Offsets,
) -> impl Iterator<Item = CardWidget<'a>> {
    let face_up_index = display.stack.details.face_up_index();

    card_iter(display, offsets).map(|(index, coords, card)| {
        let mode = {
            if index < face_up_index {
                CardWidgetMode::FullFaceDown
            } else {
                CardWidgetMode::FullFaceUp
            }
        };

        CardWidget { card, coords, mode }
    })
}

pub fn selector_widget(display: &StackWidget, offsets: &Offsets) -> Option<SelectorWidget> {
    let coords = display.coords;
    let ref details = display.stack.details;

    details.selection.map(|selection| {
        let selection_index = details.selection_index().unwrap_or_default();

        debug!("selection_index: {}", selection_index);

        /* Be careful about getting the last index. It's possible for the stack to actually be empty,
         * in which case we can't subtract from a 0 usize. */
        let end_index = details.len.bounded_sub(1);

        let start_coords = card_coords(coords, selection_index, offsets, details).unwrap_or(coords)
            + CARD_SIZE.to_y()
            + SELECTOR_OFFSET;
        let end_coords = card_coords(coords, end_index, offsets, details).unwrap_or(coords)
            + CARD_SIZE
            + SELECTOR_OFFSET;

        debug!("start_coords: {:?}", start_coords);
        debug!("end_coords: {:?}", end_coords);

        let len = u16::try_from(end_coords.x - start_coords.x).unwrap();

        SelectorWidget {
            coords: start_coords,
            len,
            orientation: details.orientation,
        }
    })
}
