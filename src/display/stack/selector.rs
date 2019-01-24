extern crate ncurses;

use ncurses::*;

use crate::display::*;

pub fn draw_horizontal_selector(_display: &mut KlondikeDisplay, coords: Coords, len: i32) {
    let Coords { x, y } = coords;

    mvprintw(y, x, "╘");

    for i in 1..(len - 1) {
        mvprintw(y, x + i, "═");
    }

    // TODO: If len == 1, this will overwrite the opening character.
    mvprintw(y, x + len - 1, "╛");
}

pub fn draw_vertical_selector(_display: &mut KlondikeDisplay, coords: Coords, len: i32) {
    let Coords { x, y } = coords;

    mvprintw(y, x, "╓");

    for i in 1..(len - 1) {
        mvprintw(y + i, x, "║");
    }

    // TODO: If len == 1, this will overwrite the opening character.
    mvprintw(y + len - 1, x, "╙");
}
