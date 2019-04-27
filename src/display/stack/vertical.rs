use std::cmp::min;
use termion;

use crate::{
    display::{
        bounds::Bounds,
        card::{CardPainter, CARD_SIZE},
        coords::Coords,
        selector::SelectorPainter,
        Result,
    },
    model::stack::{Stack, StackSelection},
    utils::usize::BoundedSub,
};

use super::common::*;

static UNCOLLAPSED_OFFSETS: Offsets = Offsets {
    unspread: Coords::from_y(1),
    collapsed_spread: Coords::from_y(1),
    uncollapsed_spread: Coords::from_y(2),
    selected: Coords::from_x(1),
    collapse_unspread_len: 0,
    collapse_spread_len: 0,
};

static CARD_SELECTOR_OFFSET: Coords = Coords::from_x(-2);
static STACK_SELECTOR_OFFSET: Coords = Coords::from_y(0);

pub trait VerticalStackPainter {
    fn draw_vertical_stack(&mut self, coords: Coords, stack: &Stack) -> Result<Bounds>;
}

impl<T> VerticalStackPainter for T
where
    T: CardPainter + SelectorPainter,
{
    fn draw_vertical_stack(&mut self, coords: Coords, stack: &Stack) -> Result<Bounds> {
        let offsets = offset_with_collapse(coords, stack)?;

        // Index at which the collapsed unspread cards will be represented.
        let collapsed_unspread_index =
            stack.details.visible_index() + offsets.collapse_unspread_len;

        let uncollapsed_spread_index = stack.details.spread_index() + offsets.collapse_spread_len;

        // First index of a face up card. All cards before this are face down.
        let face_up_index = stack.details.face_up_index();

        let mut bounds = Bounds::new(coords, coords);

        for (i, card) in stack.into_iter().enumerate() {
            if let Some(coords) = card_coords(coords, i, &offsets, &stack.details) {
                if offsets.collapse_unspread_len > 0 && i <= collapsed_unspread_index {
                    // Add 1 for the one visible card.
                    let count = offsets.collapse_unspread_len + 1;
                    bounds += self.draw_card_face_down_with_count(coords, count)?;
                } else if i < face_up_index {
                    bounds += self.draw_card_face_down(coords)?;
                } else if offsets.collapse_spread_len > 0 && i < uncollapsed_spread_index {
                    bounds += self.draw_card_face_up_slice(coords, card)?;
                } else {
                    bounds += self.draw_card_face_up(coords, card)?;
                }
            }
        }

        /* Be careful about getting the last index. It's possible for the stack to actually be empty,
         * in which case we can't subtract from a 0 usize. */
        let end_index = stack.details.len.bounded_sub(1);

        if let Some(selection) = stack.details.selection {
            match selection {
                StackSelection::Cards(_) => {
                    let selection_index = stack.details.selection_index().unwrap_or_default();

                    let start_coords =
                        card_coords(coords, selection_index, &offsets, &stack.details)
                            .unwrap_or(coords)
                            + CARD_SELECTOR_OFFSET;
                    let end_coords = card_coords(coords, end_index, &offsets, &stack.details)
                        .unwrap_or(coords)
                        + CARD_SIZE.to_y()
                        + CARD_SELECTOR_OFFSET;

                    let len = (end_coords.y - start_coords.y) as u16;
                    bounds += self.draw_vertical_selector(start_coords, len)?;
                }
                StackSelection::Stack(_) | StackSelection::FullStack => {
                    let start_coords = card_coords(coords, end_index, &offsets, &stack.details)
                        .unwrap_or(coords)
                        + CARD_SIZE.to_y()
                        + STACK_SELECTOR_OFFSET;

                    let len = CARD_SIZE.x as u16;
                    bounds += self.draw_horizontal_selector(start_coords, len)?;
                }
            }
        }

        Ok(bounds)
    }
}

fn offset_with_collapse(coords: Coords, stack: &Stack) -> Result<Offsets> {
    let mut offsets = UNCOLLAPSED_OFFSETS.clone();
    let mut collapse_len = collapse_len(coords, &offsets, stack)?;

    debug!("collapse_len: {}", collapse_len);

    if collapse_len > 0 {
        let reserve_unspread_len = if stack.details.spread_len > 0 { 0 } else { 1 };
        let unspread_len = stack.details.unspread_len();
        let collapse_unspread_len = unspread_len.bounded_sub(reserve_unspread_len + 1);
        debug!(
            "unspread_len: {}, collapse_unspread_len: {}",
            unspread_len, collapse_unspread_len
        );

        offsets.collapse_unspread_len = collapse_unspread_len;
        collapse_len = collapse_len.bounded_sub(collapse_unspread_len);
    }

    if collapse_len > 0 {
        offsets.collapse_spread_len = min(stack.details.spread_len.bounded_sub(1), collapse_len);
    }

    Ok(offsets)
}

fn collapse_len(coords: Coords, offsets: &Offsets, stack: &Stack) -> Result<usize> {
    let terminal_height = usize::from(termion::terminal_size()?.1);
    debug!("terminal_height: {}", terminal_height);

    let stack_height: usize = (0..stack.cards.len())
        .flat_map(|i| card_coords(coords, i, offsets, &stack.details))
        .map(|coords| coords + CARD_SIZE)
        .map(|coords| coords.y)
        .max()
        // Add 1 for the selector, and 1 to turn the coord into a length.
        .unwrap_or(0) as usize
        + 2;
    debug!("stack_height: {}", stack_height);

    Ok(stack_height.bounded_sub(terminal_height))
}
