use std::collections::HashMap;
use std::fmt;

use crate::model::{AreaId, Game};

use super::{
    blank::BlankPainter, bounds::Bounds, card::CARD_SIZE, coords::Coords, stack::StackPainter,
    Result,
};

static STOCK_COORDS: Coords = Coords::from_xy(2, 0);
static TALON_COORDS: Coords = Coords::from_xy(13, 0);
static FOUNDATION_COORDS: Coords = Coords::from_xy(35, 0);
static TABLEAUX_COORDS: Coords = Coords::from_xy(2, 5);

static COLUMN_OFFSET: Coords = Coords::from_x(3);

pub struct GameDisplay<'a, P> {
    painter: &'a mut P,
    area_bounds: HashMap<AreaId, Bounds>,
}

impl<'a, P> GameDisplay<'a, P>
where
    P: BlankPainter + StackPainter,
{
    pub fn new(painter: &'a mut P) -> GameDisplay<'a, P> {
        GameDisplay {
            painter,
            area_bounds: HashMap::new(),
        }
    }

    pub fn draw_area(&mut self, game: &Game, area_id: AreaId) -> Result {
        let coords = coords_for_area(area_id);
        let stack = game.stack(area_id);

        info!("Printing {:?} at {:?}", area_id, coords);

        let new_bounds = self.painter.draw_stack(coords, &stack)?;

        if let Some(old_bounds) = self.area_bounds.insert(area_id, new_bounds) {
            self.painter.draw_blank_excess(old_bounds, new_bounds)?;
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
