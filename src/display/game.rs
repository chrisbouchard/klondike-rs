use std::{cell::RefCell, collections::HashMap, convert::TryFrom, fmt};
use termion::clear;

use crate::{
    model::{AreaId, Game, Suit},
    utils::{bounds::Bounds, coords::Coords},
};

use super::{
    blank::BlankWidget, card::CARD_SIZE, help::HelpWidget, stack::StackWidget, DisplayState, Widget,
};

static STOCK_COORDS: Coords = Coords::from_xy(2, 0);
static TALON_COORDS: Coords = Coords::from_xy(13, 0);
static FOUNDATION_COORDS: Coords = Coords::from_xy(35, 0);
static TABLEAUX_COORDS: Coords = Coords::from_xy(2, 5);

static COLUMN_OFFSET: Coords = Coords::from_x(3);

#[derive(Debug, Default)]
pub struct GameWidgetStateValue {
    bounds_cache: HashMap<AreaId, Bounds>,
    prev_display_state: Option<DisplayState>,
    prev_bounds: Option<Bounds>,
}

#[derive(Debug, Default)]
pub struct GameWidgetState {
    cell: RefCell<GameWidgetStateValue>,
}

#[derive(Debug)]
pub struct GameWidget<'a, 'g>
where
    'g: 'a,
{
    pub area_ids: Vec<AreaId>,
    pub bounds: Bounds,
    pub game: &'a Game<'g>,
    pub display_state: DisplayState,
    pub widget_state: &'a GameWidgetState,
}

impl<'a, 'g> Widget for GameWidget<'a, 'g>
where
    'g: 'a,
{
    fn bounds(&self) -> Bounds {
        self.bounds
    }
}

impl<'a, 'g> fmt::Display for GameWidget<'a, 'g>
where
    'g: 'a,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let full_refresh_required = self.is_full_refresh_required();

        if full_refresh_required {
            // We're going to clear the whole terminal, so no need to remember where the widgets were.
            self.widget_state.cell.borrow_mut().bounds_cache.clear();

            let clear = clear::All;
            write!(fmt, "{}", clear)?;
        }

        match self.display_state {
            DisplayState::Playing => {
                let area_ids = if full_refresh_required {
                    self.game.area_ids()
                } else {
                    self.area_ids.clone()
                };

                for area_id in area_ids {
                    self.write_area(area_id, fmt)?;
                }
            }
            DisplayState::HelpMessageOpen => {
                self.write_help(fmt)?;
            }
            _ => {}
        }

        let mut state = self.widget_state.cell.borrow_mut();
        state.prev_bounds = Some(self.bounds);
        state.prev_display_state = Some(self.display_state);

        Ok(())
    }
}

impl<'a, 'g> GameWidget<'a, 'g>
where
    'g: 'a,
{
    fn is_full_refresh_required(&self) -> bool {
        let state = self.widget_state.cell.borrow();

        let bounds_changed = state
            .prev_bounds
            .map(|prev_bounds| prev_bounds != self.bounds)
            .unwrap_or(true);
        let display_state_changed = state
            .prev_display_state
            .map(|prev_display_state| prev_display_state != self.display_state)
            .unwrap_or(true);

        bounds_changed || display_state_changed
    }

    fn write_area(&self, area_id: AreaId, fmt: &mut fmt::Formatter) -> fmt::Result {
        let bounds = bounds_for_area(area_id, self.bounds);

        info!("Printing {:?} at {:?}", area_id, bounds.top_left);

        // Let's just go ahead and borrow mutably right away. We'll need it at the end of the
        // method when we update the bounds, anyway. Better to know up-front if there's any
        // weird aliasing going on.
        let bounds_cache = &mut self.widget_state.cell.borrow_mut().bounds_cache;

        if let Some(&bounds) = bounds_cache.get(&area_id) {
            let blank_widget = BlankWidget { bounds };
            write!(fmt, "{}", blank_widget)?;
        }

        let stack = self.game.stack(area_id);

        let stack_widget = StackWidget {
            bounds,
            stack: &stack,
        };

        let new_bounds = stack_widget.bounds();
        write!(fmt, "{}", stack_widget)?;

        // Ignore return value, because we don't need the old value.
        let _ = bounds_cache.insert(area_id, new_bounds);

        Ok(())
    }

    fn write_help(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let widget = HelpWidget {
            bounds: self.bounds,
        };

        write!(fmt, "{}", widget)?;

        Ok(())
    }
}

fn bounds_for_area(area_id: AreaId, widget_bounds: Bounds) -> Bounds {
    let top_left = coords_for_area(area_id);

    match area_id {
        AreaId::Stock => {
            let right = coords_for_area(AreaId::Talon).x - 1;
            let bottom = coords_for_area(AreaId::Tableaux(0)).y - 1;
            let bottom_right = Coords::from_xy(right, bottom);

            Bounds::new(top_left, bottom_right)
        }
        AreaId::Talon => {
            let first_suit = Suit::try_from(0).unwrap();
            let right = coords_for_area(AreaId::Foundation(first_suit)).x - 1;
            let bottom = coords_for_area(AreaId::Tableaux(0)).y - 1;
            let bottom_right = Coords::from_xy(right, bottom);

            Bounds::new(top_left, bottom_right)
        }
        AreaId::Foundation(suit) => {
            let next_suit = Suit::try_from(u8::from(suit) + 1).ok();
            let right = if let Some(next_suit) = next_suit {
                coords_for_area(AreaId::Foundation(next_suit)).x - 1
            } else {
                widget_bounds.bottom_right.x
            };
            let bottom = coords_for_area(AreaId::Tableaux(0)).y - 1;
            let bottom_right = Coords::from_xy(right, bottom);

            Bounds::new(top_left, bottom_right)
        }
        AreaId::Tableaux(index) => {
            let right = coords_for_area(AreaId::Tableaux(index + 1)).x - 1;
            let bottom = widget_bounds.bottom_right.y;
            let bottom_right = Coords::from_xy(right, bottom);

            Bounds::new(top_left, bottom_right)
        }
    }
}

fn coords_for_area(area_id: AreaId) -> Coords {
    match area_id {
        AreaId::Stock => STOCK_COORDS,
        AreaId::Talon => TALON_COORDS,
        AreaId::Foundation(suit) => {
            FOUNDATION_COORDS + i32::from(u8::from(suit)) * (CARD_SIZE.to_x() + COLUMN_OFFSET)
        }
        AreaId::Tableaux(index) => {
            TABLEAUX_COORDS + i32::from(index) * (CARD_SIZE.to_x() + COLUMN_OFFSET)
        }
    }
}
