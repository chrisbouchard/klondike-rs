use std::{collections::HashMap, fmt, io};

use crate::model::{AreaId, Game};

use super::{
    blank::BlankPainter, bounds::Bounds, card::CARD_SIZE, coords::Coords, help::HelpPainter,
    stack::StackPainter, Result,
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

impl<'a, W> GameDisplay<'a, W>
where
    W: io::Write,
{
    pub fn new(painter: &'a mut W) -> GameDisplay<'a, W> {
        GameDisplay {
            painter,
            area_bounds: HashMap::new(),
        }
    }

    pub fn draw_area(&mut self, game: &Game, area_id: AreaId) -> Result {
        let coords = coords_for_area(area_id);
        let stack = game.stack(area_id);

        info!("Printing {:?} at {:?}", area_id, coords);

        if let Some(&bounds) = self.area_bounds.get(&area_id) {
            self.painter.draw_blank_bounds(bounds)?;
        }

        let new_bounds = self.painter.draw_stack(coords, &stack)?;

        // Ignore return value, because we don't need the old value.
        let _ = self.area_bounds.insert(area_id, new_bounds);

        Ok(())
    }

    pub fn draw_all_areas(&mut self, game: &Game) -> Result {
        self.painter.draw_blank_all()?;
        self.area_bounds.clear();

        for area_id in game.area_ids() {
            self.draw_area(game, area_id)?;
        }

        Ok(())
    }

    pub fn draw_help(&mut self) -> Result {
        self.painter.draw_help_message()?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result {
        self.painter.flush()?;
        Ok(())
    }
}

impl<'a, P> fmt::Debug for GameDisplay<'a, P>
where
    P: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("GameDisplay")
            .field("painter", self.painter)
            .field("area_bounds", &self.area_bounds)
            .finish()
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
            TABLEAUX_COORDS + i32::from(index) * (CARD_SIZE.to_x() + COLUMN_OFFSET)
        }
    }
}
