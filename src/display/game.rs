use crate::model::{AreaId, KlondikeGame};

use super::card::CARD_SIZE;
use super::coords::Coords;
use super::Result;
use super::stack::StackPainter;

static STOCK_COORDS: Coords = Coords::from_xy(2, 0);
static TALON_COORDS: Coords = Coords::from_xy(13, 0);
static FOUNDATION_COORDS: Coords = Coords::from_xy(35, 0);
static TABLEAUX_COORDS: Coords = Coords::from_xy(2, 5);

static COLUMN_OFFSET: Coords = Coords::from_x(3);

pub trait GamePainter {
    fn draw_game(&mut self, game: &KlondikeGame) -> Result;
}

impl<T> GamePainter for T where T: StackPainter {
    fn draw_game(&mut self, game: &KlondikeGame) -> Result {
        info!("Printing stock at {:?}", STOCK_COORDS);
        self.draw_horizontal_card_stack(
            STOCK_COORDS,
            &game.stack(AreaId::Stock)
        )?;

        info!("Printing talon at {:?}", TALON_COORDS);
        self.draw_horizontal_card_stack(
            TALON_COORDS,
            &game.stack(AreaId::Talon)
        )?;

        for i in 0..4 {
            let coords =
                FOUNDATION_COORDS
                    + (i as i32) * (CARD_SIZE.to_x() + COLUMN_OFFSET);
            info!("Printing foundation stack {:?} at {:?}", i, coords);
            self.draw_horizontal_card_stack(
                coords,
                &game.stack(AreaId::Foundation(i))
            )?;
        }

        for i in 0..7 {
            let coords =
                TABLEAUX_COORDS
                    + (i as i32) * (CARD_SIZE.to_x() + COLUMN_OFFSET);
            info!("Printing tableaux stack {} at {:?}", i, coords);
            self.draw_vertical_card_stack(
                coords,
                &game.stack(AreaId::Tableaux(i))
            )?;
        }

        Ok(())
    }
}
