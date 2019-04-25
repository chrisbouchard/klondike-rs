use std::fmt;

use crate::model::{AreaId, Game};

use super::{card::CARD_SIZE, coords::Coords, stack::StackPainter, Result};

static STOCK_COORDS: Coords = Coords::from_xy(2, 0);
static TALON_COORDS: Coords = Coords::from_xy(13, 0);
static FOUNDATION_COORDS: Coords = Coords::from_xy(35, 0);
static TABLEAUX_COORDS: Coords = Coords::from_xy(2, 5);

static COLUMN_OFFSET: Coords = Coords::from_x(3);

pub struct GameDisplay<'a, P> {
    stack_painter: &'a mut P,
}

impl<'a, P> GameDisplay<'a, P>
where
    P: StackPainter,
{
    pub fn new(stack_painter: &'a mut P) -> GameDisplay<'a, P> {
        GameDisplay { stack_painter }
    }

    pub fn draw_game(&mut self, game: &Game, area_ids: impl IntoIterator<Item = AreaId>) -> Result {
        for area_id in area_ids {
            let coords = coords_for_area(area_id);
            info!("Printing {:?} at {:?}", area_id, coords);
            self.stack_painter
                .draw_stack(coords, &game.stack(area_id))?;
        }

        Ok(())
    }
}

impl<'a, P> fmt::Display for GameDisplay<'a, P> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "GameDisplay {..}")
    }
}

fn coords_for_area(area_id: AreaId) -> Coords {
    match area_id {
        AreaId::Stock => STOCK_COORDS,
        AreaId::Talon => TALON_COORDS,
        AreaId::Foundation(index) => {
            FOUNDATION_COORDS + (index as i32) * (CARD_SIZE.to_x() + COLUMN_OFFSET)
        }
        AreaId::Tableaux(index) => {
            TABLEAUX_COORDS + (index as i32) * (CARD_SIZE.to_x() + COLUMN_OFFSET)
        }
    }
}
