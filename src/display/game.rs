use crate::display::card::*;
use crate::display::coords::*;
use crate::display::stack::*;
use crate::game::*;

static STOCK_COORDS: Coords = Coords::from_xy(2, 0);
static TALON_COORDS: Coords = Coords::from_xy(13, 0);
static FOUNDATION_COORDS: Coords = Coords::from_xy(35, 0);
static TABLEAUX_COORDS: Coords = Coords::from_xy(2, 5);

static COLUMN_OFFSET: Coords = Coords::from_x(3);

pub trait GamePainter {
    fn draw_game(&mut self, game: &KlondikeGame);
}

impl<T> GamePainter for T where T: StackPainter {
    fn draw_game(&mut self, game: &KlondikeGame) {
        info!("Printing stock at {:?}", STOCK_COORDS);
        self.draw_horizontal_card_stack(STOCK_COORDS, &game.stock());

        info!("Printing talon at {:?}", TALON_COORDS);
        self.draw_horizontal_card_stack(TALON_COORDS, &game.talon());

        for (i, (suit, stack)) in game.foundation().enumerate() {
            let coords =
                FOUNDATION_COORDS
                    + (i as i32) * (CARD_SIZE.to_x() + COLUMN_OFFSET);
            info!("Printing {:?} foundation at {:?}", suit, coords);
            self.draw_horizontal_card_stack(coords, &stack);
        }

        for (i, stack) in game.tableaux().enumerate() {
            let coords =
                TABLEAUX_COORDS
                    + (i as i32) * (CARD_SIZE.to_x() + COLUMN_OFFSET);
            info!("Printing tableaux stack {} at {:?}", i, coords);
            self.draw_vertical_card_stack(coords, &stack);
        }
    }
}
