use std::{cmp::min, convert::TryFrom};

use crate::display::{
    card::{CardWidget, CardWidgetMode, CARD_SIZE},
    geometry,
    selector::SelectorWidget,
};

use super::{
    common::{card_coords, card_iter, Offsets},
    StackWidget,
};

lazy_static! {
    static ref SELECTOR_OFFSET: geometry::Vector2D<i16> = geometry::vec2(-2, 0);
    static ref UNCOLLAPSED_OFFSETS: Offsets = Offsets {
        unspread: geometry::vec2(0, 1),
        collapsed_spread: geometry::vec2(0, 1),
        uncollapsed_spread: geometry::vec2(0, 2),
        selected: geometry::vec2(1, 0),
        collapse_unspread_len: 0,
        collapse_spread_len: 0,
    };
}

pub fn offsets(widget: &StackWidget) -> Offsets {
    let ref details = widget.stack.details;

    let mut offsets = UNCOLLAPSED_OFFSETS.clone();
    let mut collapse_len: usize = collapse_len(widget, &offsets).into();

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

fn collapse_len(widget: &StackWidget, offsets: &Offsets) -> u16 {
    if widget.stack.cards.is_empty() {
        return 0;
    }

    let origin = widget.bounds.origin;
    let maximum_y = widget.bounds.max_y();

    let last_card_coords = card_coords(
        origin,
        widget.stack.cards.len() - 1,
        offsets,
        &widget.stack.details,
    )
    .unwrap_or_default();

    let uncollapsed_bounds = geometry::Rect::new(last_card_coords, *CARD_SIZE);
    let uncollapsed_y = uncollapsed_bounds.max_y();

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

        CardWidget {
            card,
            origin: coords,
            mode,
        }
    })
}

pub fn selector_widget(widget: &StackWidget, offsets: &Offsets) -> Option<SelectorWidget> {
    let coords = widget.bounds.origin;
    let ref details = widget.stack.details;

    details.selection.as_ref().map(|selection| {
        let selection_index = details.selection_index().unwrap_or_default();

        // Be careful about getting the last index. It's possible for the stack to actually be
        // empty, in which case we can't subtract from a 0 usize.
        let end_index = details.len.saturating_sub(1);

        let held_offset = if selection.held {
            -UNCOLLAPSED_OFFSETS.selected
        } else {
            Default::default()
        };

        let start_coords = card_coords(coords, selection_index, offsets, details)
            .unwrap_or(coords)
            .cast::<i16>()
            + *SELECTOR_OFFSET
            + held_offset;
        let end_coords = card_coords(coords, end_index, offsets, details)
            .unwrap_or(coords)
            .cast::<i16>()
            + geometry::vec2(0, CARD_SIZE.height).cast::<i16>()
            + *SELECTOR_OFFSET
            + held_offset;

        let len = u16::try_from(end_coords.y - start_coords.y).unwrap();

        SelectorWidget {
            origin: start_coords.cast::<u16>(),
            len,
            orientation: details.orientation,
        }
    })
}
