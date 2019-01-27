use rustty::ui::Painter;
use crate::display::*;

pub trait SelectorPainter {
    fn draw_horizontal_selector(&mut self, coords: Coords, len: i32);
    fn draw_vertical_selector(&mut self, coords: Coords, len: i32);
}

impl<T> SelectorPainter for T where T: Painter {
    fn draw_horizontal_selector(&mut self, coords: Coords, len: i32) {
        /* The outside world speaks i32, but rustty speaks usize. */
        let (x, y) = coords.as_pos();
        let len = len as usize;

        self.printline(x, y, "╘");

        for i in 1..(len - 1) {
            self.printline(x + i, y, "═");
        }

        // TODO: If len == 1, this will overwrite the opening character.
        self.printline(x + len - 1, y, "╛");

        debug!("coords: {:?}, len: {}", coords, len);
    }

    fn draw_vertical_selector(&mut self, coords: Coords, len: i32) {
        /* The outside world speaks i32, but rustty speaks usize. */
        let (x, y) = coords.as_pos();
        let len = len as usize;

        self.printline(x, y, "╓");

        for i in 1..(len - 1) {
            self.printline(x, y + i, "║");
        }

        // TODO: If len == 1, this will overwrite the opening character.
        self.printline(x, y + len - 1, "╙");

        debug!("coords: {:?}, len: {}", coords, len);
    }
}
