use std::{cmp::min, convert::TryFrom};

use crate::{
    display::{
        card::{CardWidget, CardWidgetMode, CARD_SIZE},
        selector::SelectorWidget,
    },
    utils::{
        bounds::Bounds,
        coords::{self, Coords},
    },
};

use super::{
    common::{card_coords, card_iter, Offsets},
    StackWidget,
};

static UNCOLLAPSED_OFFSETS: Offsets = Offsets {
    unspread: Coords::from_y(1),
    collapsed_spread: Coords::from_y(1),
    uncollapsed_spread: Coords::from_y(2),
    selected: Coords::from_x(1),
    collapse_unspread_len: 0,
    collapse_spread_len: 0,
};

static SELECTOR_OFFSET: Coords = Coords::from_x(-2);

pub fn offsets(widget: &StackWidget) -> Offsets {
    let ref details = widget.stack.details;

    let mut offsets = UNCOLLAPSED_OFFSETS.clone();
    let mut collapse_len = collapse_len(widget, &offsets);

    debug!("collapse_len: {}", collapse_len);

    if collapse_len > 0 {
        let reserve_unspread_len = if details.spread_len > 0 { 0 } else { 1 };
        let unspread_len = details.unspread_len();
        let collapse_unspread_len = unspread_len.saturating_sub(reserve_unspread_len + 1);
        debug!(
            "unspread_len: {}, collapse_unspread_len: {}",
            unspread_len, collapse_unspread_len
        );

        offsets.collapse_unspread_len = collapse_unspread_len;
        collapse_len = collapse_len.saturating_sub(collapse_unspread_len);
    }

    if collapse_len > 0 {
        offsets.collapse_spread_len = min(details.spread_len.saturating_sub(1), collapse_len);
    }

    offsets
}

fn collapse_len(widget: &StackWidget, offsets: &Offsets) -> usize {
    if widget.stack.cards.is_empty() {
        return 0;
    }

    let coords = widget.bounds.top_left;
    let maximum_y = usize::try_from(widget.bounds.bottom_right.y).unwrap();

    let last_card_coords = card_coords(
        coords,
        widget.stack.cards.len() - 1,
        offsets,
        &widget.stack.details,
    )
    .unwrap_or_default();

    let uncollapsed_bounds = Bounds::with_size(last_card_coords, CARD_SIZE);
    let uncollapsed_y = usize::try_from(uncollapsed_bounds.bottom_right.y).unwrap();

    uncollapsed_y.saturating_sub(maximum_y)
}

pub fn card_widget_iter<'a>(
    widget: &'a StackWidget,
    offsets: &'a Offsets,
) -> impl Iterator<Item = CardWidget<'a>> {
    let ref details = widget.stack.details;

    // Index at which the collapsed unspread cards will be represented.
    let collapsed_unspread_index = details.visible_index() + offsets.collapse_unspread_len;

    let uncollapsed_spread_index = details.spread_index() + offsets.collapse_spread_len;

    // First index of a face up card. All cards before this are face down.
    let face_up_index = widget.stack.details.face_up_index();

    card_iter(widget, offsets).map(move |(index, coords, card)| {
        let mode = {
            if offsets.collapse_unspread_len > 0 && index <= collapsed_unspread_index {
                // Add 1 for the one visible card.
                let count = offsets.collapse_unspread_len + 1;
                CardWidgetMode::SliceFaceDown(count)
            } else if index < face_up_index {
                CardWidgetMode::FullFaceDown
            } else if offsets.collapse_spread_len > 0 && index < uncollapsed_spread_index {
                CardWidgetMode::SliceFaceUp
            } else {
                CardWidgetMode::FullFaceUp
            }
        };

        CardWidget { card, coords, mode }
    })
}

pub fn selector_widget(widget: &StackWidget, offsets: &Offsets) -> Option<SelectorWidget> {
    let coords = widget.bounds.top_left;
    let ref details = widget.stack.details;

    details.selection.as_ref().map(|selection| {
        let selection_index = details.selection_index().unwrap_or_default();

        // Be careful about getting the last index. It's possible for the stack to actually be
        // empty, in which case we can't subtract from a 0 usize.
        let end_index = details.len.saturating_sub(1);

        let held_offset = if selection.held {
            -UNCOLLAPSED_OFFSETS.selected
        } else {
            coords::ZERO
        };

        let start_coords = card_coords(coords, selection_index, offsets, details).unwrap_or(coords)
            + SELECTOR_OFFSET
            + held_offset;
        let end_coords = card_coords(coords, end_index, offsets, details).unwrap_or(coords)
            + CARD_SIZE.to_y()
            + SELECTOR_OFFSET
            + held_offset;

        let len = u16::try_from(end_coords.y - start_coords.y).unwrap();

        SelectorWidget {
            coords: start_coords,
            len,
            orientation: details.orientation,
        }
    })
}
