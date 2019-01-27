use rustty::Cell;
use rustty::ui::Painter;

use crate::display::coords::*;
use crate::game::*;

pub static CARD_SIZE: Coords = Coords::from_xy(8, 4);

pub trait CardPainter {
    fn draw_card(&mut self, coords: Coords, card: &Card);
}

impl<T> CardPainter for T where T: Painter {
    fn draw_card(&mut self, coords: Coords, card: &Card) {
        draw_card_frame( self, coords);

        let (x, y) = coords.as_pos();

        if card.face_up {
            let rank_str = card.rank.label();
            let suit_str = card.suit.symbol();

            let offset = 2 - card.rank.label().len();

            let cell =
                Cell::with_style(
                    card_color_pair(card),
                    rustty::Color::Default,
                    rustty::Attr::Default,
                );

            self.printline_with_cell(x + 2, y + 1, &rank_str, cell);
            self.printline_with_cell(x + 5, y + 1, &suit_str, cell);
            self.printline_with_cell(x + 2, y + 2, &suit_str, cell);
            self.printline_with_cell(x + 4 + offset, y + 2, &rank_str, cell);
        } else {
            let cell =
                Cell::with_style(
                    rustty::Color::Blue,
                    rustty::Color::Default,
                    rustty::Attr::Default,
                );
            self.printline_with_cell(x + 2, y + 1, "░░░░", cell);
            self.printline_with_cell(x + 2, y + 2, "░░░░", cell);
        }
    }
}

fn draw_card_frame(painter: &mut Painter, coords: Coords) {
    let (x, y) = coords.as_pos();

    painter.printline(x + 0, y + 0, "╭──────╮");
    painter.printline(x + 0, y + 1, "│      │");
    painter.printline(x + 0, y + 2, "│      │");
    painter.printline(x + 0, y + 3, "╰──────╯");
}

fn card_color_pair(card: &Card) -> rustty::Color {
    match card.suit.color() {
        Color::Black => rustty::Color::Default,
        Color::Red => rustty::Color::Red,
    }
}
