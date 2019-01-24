extern crate ncurses;

use ncurses::*;

use crate::display::*;
use crate::display::coords::*;
use crate::game::*;

pub static CARD_HEIGHT: i32 = 8;
pub static CARD_WIDTH: i32 = 4;

pub fn draw_card_frame(display: &mut KlondikeDisplay, coords: Coords) {
    let Coords { x, y } = coords;

    attron(COLOR_PAIR(COLOR_PAIR_DEFAULT));
    mvprintw(y + 0, x + 0, "╭──────╮");
    mvprintw(y + 1, x + 0, "│      │");
    mvprintw(y + 2, x + 0, "│      │");
    mvprintw(y + 3, x + 0, "╰──────╯");
    attroff(COLOR_PAIR(COLOR_PAIR_DEFAULT));
}

pub fn draw_card(display: &mut KlondikeDisplay, coords: Coords, card: &Card) {
    draw_card_frame(display, coords);

    let Coords { x, y } = coords;

    if card.face_up {
        let rank_str = card.rank.label();
        let suit_str = card.suit.symbol();

        let offset: i32 = 2 - card.rank.label().len() as i32;

        let color_pair = card_color_pair(card);

        attron(COLOR_PAIR(color_pair));
        mvprintw(y + 1, x + 2, &rank_str);
        mvprintw(y + 1, x + 5, &suit_str);
        mvprintw(y + 2, x + 2, &suit_str);
        mvprintw(y + 2, x + 4 + offset, &rank_str);
        attroff(COLOR_PAIR(color_pair));
    } else {
        attron(COLOR_PAIR(COLOR_PAIR_CARD_BACK));
        mvprintw(y + 1, x + 2, "░░░░");
        mvprintw(y + 2, x + 2, "░░░░");
        attroff(COLOR_PAIR(COLOR_PAIR_CARD_BACK));
    }
}

fn card_color_pair(card: &Card) -> i16 {
    match card.suit.color() {
        Color::Black => COLOR_PAIR_CARD_BLACK,
        Color::Red => COLOR_PAIR_CARD_RED
    }
}
