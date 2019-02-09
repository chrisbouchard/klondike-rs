use crate::display::{
    card::{CardPainter, CARD_SIZE},
    coords::Coords,
    selector::SelectorPainter,
    Result,
};
use crate::model::stack::{Stack, StackSelection};

use super::common::*;

static OFFSETS: Offsets = Offsets {
    unspread: Coords::from_y(1),
    spread: Coords::from_y(2),
    selected: Coords::from_x(1),
};

static CARD_SELECTOR_OFFSET: Coords = Coords::from_x(-2);
static STACK_SELECTOR_OFFSET: Coords = Coords::from_y(0);

pub trait VerticalStackPainter {
    fn draw_vertical_card_stack(&mut self, coords: Coords, stack: &Stack) -> Result;
}

impl<T> VerticalStackPainter for T
where
    T: CardPainter + SelectorPainter,
{
    fn draw_vertical_card_stack(&mut self, coords: Coords, stack: &Stack) -> Result {
        let stack_details = stack.details();

        for (i, card) in stack.into_iter().enumerate() {
            if let Some(coords) = card_coords(coords, i, &OFFSETS, stack_details) {
                self.draw_card(coords, card)?;
            }
        }

        /* Be careful about getting the last index. It's possible for the stack to actually be empty,
         * in which case we can't subtract from a 0 usize. */
        let stack_len = stack_details.len;
        let end_index = if stack_len > 0 { stack_len - 1 } else { 0 };

        if let Some(selection) = stack_details.selection {
            match selection {
                StackSelection::Cards(_) => {
                    let selection_index = stack_details.selection_index().unwrap_or_default();

                    let start_coords =
                        card_coords(coords, selection_index, &OFFSETS, stack_details)
                            .unwrap_or(coords)
                            + CARD_SELECTOR_OFFSET;
                    let end_coords = card_coords(coords, end_index, &OFFSETS, stack_details)
                        .unwrap_or(coords)
                        + CARD_SIZE.to_y()
                        + CARD_SELECTOR_OFFSET;

                    let selector_len = end_coords.y - start_coords.y;

                    self.draw_vertical_selector(start_coords, selector_len)?;
                }
                StackSelection::Stack(_) | StackSelection::FullStack => {
                    let start_coords = card_coords(coords, end_index, &OFFSETS, stack_details)
                        .unwrap_or(coords)
                        + CARD_SIZE.to_y()
                        + STACK_SELECTOR_OFFSET;

                    let selector_len = CARD_SIZE.x;

                    self.draw_horizontal_selector(start_coords, selector_len)?;
                }
            }
        }

        Ok(())
    }
}
