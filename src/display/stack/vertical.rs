use std::{cmp::min, convert::TryFrom};
use termion;

use crate::display::{
    card::{CardWidget, CardWidgetMode, CARD_SIZE},
    coords::{Coords, ZERO},
    selector::SelectorWidget,
};

use super::{common::*, Offsets, StackWidget};

static UNCOLLAPSED_OFFSETS: Offsets = Offsets {
    unspread: Coords::from_y(1),
    collapsed_spread: Coords::from_y(1),
    uncollapsed_spread: Coords::from_y(2),
    selected: Coords::from_x(1),
    collapse_unspread_len: 0,
    collapse_spread_len: 0,
};

static SELECTOR_OFFSET: Coords = Coords::from_x(-2);

pub fn offsets(display: &StackWidget) -> Offsets {
    let ref details = display.stack.details;

    let mut offsets = UNCOLLAPSED_OFFSETS.clone();
    let mut collapse_len = collapse_len(display, &offsets);

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

fn collapse_len(display: &StackWidget, offsets: &Offsets) -> usize {
    // TODO: Get this from the widget, not from termion.
    let terminal_height = usize::from(termion::terminal_size().unwrap().1);
    debug!("terminal_height: {}", terminal_height);

    let stack_height = usize::try_from(
        (0..display.stack.cards.len())
            .flat_map(|i| card_coords(display.coords, i, offsets, &display.stack.details))
            .map(|coords| coords + CARD_SIZE)
            .map(|coords| coords.y)
            .max()
            .unwrap_or(0)
            // Add 1 to turn the coordinate into a length.
            + 1,
    )
    .unwrap();
    debug!("stack_height: {}", stack_height);

    stack_height.saturating_sub(terminal_height)
}

pub fn card_widget_iter<'a>(
    display: &'a StackWidget,
    offsets: &'a Offsets,
) -> impl Iterator<Item = CardWidget<'a>> {
    let ref details = display.stack.details;

    // Index at which the collapsed unspread cards will be represented.
    let collapsed_unspread_index = details.visible_index() + offsets.collapse_unspread_len;

    let uncollapsed_spread_index = details.spread_index() + offsets.collapse_spread_len;

    // First index of a face up card. All cards before this are face down.
    let face_up_index = display.stack.details.face_up_index();

    card_iter(display, offsets).map(|(index, coords, card)| {
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

pub fn selector_widget(display: &StackWidget, offsets: &Offsets) -> Option<SelectorWidget> {
    let coords = display.coords;
    let ref details = display.stack.details;

    details.selection.map(|selection| {
        let selection_index = details.selection_index().unwrap_or_default();

        // Be careful about getting the last index. It's possible for the stack to actually be
        // empty, in which case we can't subtract from a 0 usize.
        let end_index = details.len.saturating_sub(1);

        let held_offset = if selection.held {
            -UNCOLLAPSED_OFFSETS.selected
        } else {
            ZERO
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
