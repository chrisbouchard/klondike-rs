extern crate ncurses;

use crate::display::*;
use crate::display::card::*;
use crate::display::coords::*;
use crate::game::*;

pub fn draw_horizontal_card_stack(
    display: &mut KlondikeDisplay,
    coords: Coords,
    stack: &CardStack,
) {
    let mut x = coords.x;
    let y = coords.y;

    if !stack.fanned.is_empty() {
        if !stack.pile.is_empty() {
            draw_card_frame(display, Coords { x, y });
            x += 1;
        }

        for card in stack.fanned.iter() {
            draw_card(display, Coords { x, y }, card);
            x += 4;
        }
    } else if let Some((top_card, rest)) = stack.pile.split_last() {
        if !rest.is_empty() {
            draw_card_frame(display, Coords { x, y });
            x += 1;
        }

        draw_card(display, Coords { x, y }, top_card);
    }
}

pub fn draw_vertical_card_stack(
    display: &mut KlondikeDisplay,
    coords: Coords,
    stack: &CardStack,
) {
    let x = coords.x;
    let mut y = coords.y;

    if !stack.fanned.is_empty() {
        for card in stack.fanned.iter() {
            draw_card(display, Coords { x, y }, card);
            y += 2;
        }
    } else if let Some(top_card) = stack.pile.last() {
        draw_card(display, Coords { x, y }, top_card);
    }
}
