extern crate ncurses;

use ncurses::*;

use crate::game::*;


#[derive(Copy, Clone, Debug)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}


/* Color pairs; foreground && background. */
static COLOR_PAIR_DEFAULT: i16 = 1;
static COLOR_PAIR_CARD_BLACK: i16 = 2;
static COLOR_PAIR_CARD_RED: i16 = 3;
static COLOR_PAIR_CARD_BACK: i16 = 4;

pub struct KlondikeDisplay {
    _secret: ()
}

impl KlondikeDisplay {
    pub fn init() -> KlondikeDisplay {
        let locale_conf = LcCategory::all;
        setlocale(locale_conf, "en_US.UTF-8");

        /* Setup ncurses. */
        initscr();
        cbreak();

        /* Allow for extended keyboard (like F1). */
        keypad(stdscr(), true);
        noecho();

        /* Invisible cursor. */
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        /* Start colors. */
        start_color();
        use_default_colors();

        init_pair(COLOR_PAIR_DEFAULT, COLOR_WHITE, COLOR_BLACK);
        init_pair(COLOR_PAIR_CARD_BLACK, COLOR_WHITE, COLOR_BLACK);
        init_pair(COLOR_PAIR_CARD_RED, COLOR_RED, COLOR_BLACK);
        init_pair(COLOR_PAIR_CARD_BACK, COLOR_BLUE, COLOR_BLACK);

        /* Set the window's background color. */
        bkgd(' ' as chtype | COLOR_PAIR(COLOR_PAIR_DEFAULT) as chtype);

        KlondikeDisplay { _secret: () }
    }

    pub fn refresh(&mut self) {
        refresh();
    }

    pub fn draw_card_frame(&mut self, coords: Coords) {
        let Coords { x, y } = coords;

        attron(COLOR_PAIR(COLOR_PAIR_DEFAULT));
        mvprintw(y + 0, x + 0, "╭────────╮");
        mvprintw(y + 1, x + 0, "│        │");
        mvprintw(y + 2, x + 0, "│        │");
        mvprintw(y + 3, x + 0, "╰────────╯");
        attroff(COLOR_PAIR(COLOR_PAIR_DEFAULT));
    }

    pub fn draw_card(&mut self, coords: Coords, card: &Card) {
        self.draw_card_frame(coords);

        let Coords { x, y } = coords;

        if card.face_up {
            let rank_str = card.rank.label();
            let suit_str = card.suit.symbol();

            let offset: i32 = 2 - card.rank.label().len() as i32;

            let color_pair = card_color_pair(card);

            attron(COLOR_PAIR(color_pair));
            mvprintw(y + 1, x + 2, &rank_str);
            mvprintw(y + 1, x + 7, &suit_str);
            mvprintw(y + 2, x + 2, &suit_str);
            mvprintw(y + 2, x + 6 + offset, &rank_str);
            attroff(COLOR_PAIR(color_pair));
        } else {
            attron(COLOR_PAIR(COLOR_PAIR_CARD_BACK));
            mvprintw(y + 1, x + 2, "░░░░░░");
            mvprintw(y + 2, x + 2, "░░░░░░");
            attroff(COLOR_PAIR(COLOR_PAIR_CARD_BACK));
        }
    }

    pub fn draw_horizontal_card_stack(
        &mut self,
        coords: Coords,
        stack: &CardStack,
    ) {
        let mut x = coords.x;
        let y = coords.y;

        if !stack.fanned.is_empty() {
            if !stack.pile.is_empty() {
                self.draw_card_frame(Coords { x, y });
                x += 1;
            }

            for card in stack.fanned.iter() {
                self.draw_card(Coords { x, y }, card);
                x += 4;
            }
        }
        else if let Some((top_card, rest)) = stack.pile.split_last() {
            if !rest.is_empty() {
                self.draw_card_frame(Coords { x, y });
                x += 1;
            }

            self.draw_card(Coords { x, y }, top_card);
        }
    }

    pub fn draw_vertical_card_stack(
        &mut self,
        coords: Coords,
        stack: &CardStack,
    ) {
        let x = coords.x;
        let mut y = coords.y;

        if !stack.fanned.is_empty() {
            for card in stack.fanned.iter() {
                self.draw_card(Coords { x, y }, card);
                y += 2;
            }
        }
        else if let Some(top_card) = stack.pile.last() {
            self.draw_card(Coords { x, y }, top_card);
        }
    }
}

impl Drop for KlondikeDisplay {
    fn drop(&mut self) {
        /* Wait for one more character before exiting. */
        getch();
        endwin();
    }
}


fn card_color_pair(card: &Card) -> i16 {
    match card.suit.color() {
        Color::BLACK => COLOR_PAIR_CARD_BLACK,
        Color::RED => COLOR_PAIR_CARD_RED
    }
}
