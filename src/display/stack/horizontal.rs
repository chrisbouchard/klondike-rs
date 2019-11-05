use std::convert::TryFrom;

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
    static ref OFFSETS: Offsets = Offsets {
        unspread: geometry::vec2(1, 0),
        collapsed_spread: geometry::vec2(4, 0),
        uncollapsed_spread: geometry::vec2(4, 0),
        selected: geometry::vec2(1, 0),
        collapse_unspread_len: 0,
        collapse_spread_len: 0,
    };
}

pub fn offsets(_widget: &StackWidget) -> Offsets {
    OFFSETS.clone()
}

pub fn card_widget_iter<'a>(
    widget: &'a StackWidget,
    offsets: &'a Offsets,
) -> impl Iterator<Item = CardWidget<'a>> {
    let face_up_index = widget.stack.details.face_up_index();

    card_iter(widget, offsets).map(move |(index, coords, card)| {
        let mode = {
            if index < face_up_index {
                CardWidgetMode::FullFaceDown
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
    let details = &widget.stack.details;

    details.selection.as_ref().map(|_| {
        let selection_index = details.selection_index().unwrap_or_default();

        debug!("selection_index: {}", selection_index);

        /* Be careful about getting the last index. It's possible for the stack to actually be empty,
         * in which case we can't subtract from a 0 usize. */
        let end_index = details.len.saturating_sub(1);

        let start_coords = card_coords(coords, selection_index, offsets, details)
            .unwrap_or(coords)
            .cast::<i16>()
            + geometry::vec2(0, CARD_SIZE.height).cast::<i16>();
        let end_coords = card_coords(coords, end_index, offsets, details)
            .unwrap_or(coords)
            .cast::<i16>()
            + CARD_SIZE.cast::<i16>();

        debug!("start_coords: {:?}", start_coords);
        debug!("end_coords: {:?}", end_coords);

        let len = u16::try_from(end_coords.x - start_coords.x).unwrap();

        SelectorWidget {
            origin: start_coords.cast::<u16>(),
            len,
            orientation: details.orientation,
        }
    })
}
