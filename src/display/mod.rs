extern crate ncurses;

use ncurses::*;

pub mod card;
pub mod coords;
pub mod game;
pub mod stack;

pub use self::coords::Coords;
pub use self::game::draw_game;

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

    pub fn getch(&mut self) -> i32 {
        /* Wait for one more character before exiting. */
        getch()
    }

    pub fn clear(&mut self) {
        clear();
    }

    pub fn refresh(&mut self) {
        refresh();
    }
}

impl Drop for KlondikeDisplay {
    fn drop(&mut self) {
        endwin();
    }
}
