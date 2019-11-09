use std::{cell::RefCell, collections::HashMap, convert::TryFrom, fmt};
use termion::clear;

use crate::model::{AreaId, Game, Suit};

use super::{
    blank::BlankWidget, card::CARD_SIZE, geometry, help::HelpWidget, stack::StackWidget,
    win::WinWidget, DisplayState, Widget,
};

lazy_static! {
    static ref STOCK_COORDS: geometry::Point2D<u16> = geometry::point2(2, 0);
    static ref TALON_COORDS: geometry::Point2D<u16> = geometry::point2(13, 0);
    static ref FOUNDATION_COORDS: geometry::Point2D<u16> = geometry::point2(35, 0);
    static ref TABLEAUX_COORDS: geometry::Point2D<u16> = geometry::point2(2, 5);
    static ref COLUMN_OFFSET: geometry::Vector2D<u16> = geometry::vec2(3, 0);
}

#[derive(Debug, Default)]
pub struct GameWidgetStateValue {
    bounds_cache: HashMap<AreaId, geometry::Rect<u16>>,
    prev_display_state: Option<DisplayState>,
    prev_bounds: Option<geometry::Rect<u16>>,
}

#[derive(Debug, Default)]
pub struct GameWidgetState {
    cell: RefCell<GameWidgetStateValue>,
}

#[derive(Debug)]
pub struct GameWidget<'a> {
    pub area_ids: &'a [AreaId],
    pub bounds: geometry::Rect<u16>,
    pub game: &'a Game,
    pub display_state: DisplayState,
    pub widget_state: &'a GameWidgetState,
}

impl<'a> Widget for GameWidget<'a> {
    fn bounds(&self) -> geometry::Rect<u16> {
        self.bounds
    }
}

impl<'a> fmt::Display for GameWidget<'a> {
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
                let game_area_ids = self.game.area_ids();

                let area_ids = if full_refresh_required {
                    &game_area_ids
                } else {
                    self.area_ids
                };

                for area_id in area_ids {
                    self.write_area(*area_id, fmt)?;
                }
            }
            DisplayState::HelpMessageOpen => {
                self.write_help(fmt)?;
            }
            DisplayState::WinMessageOpen => {
                self.write_win(fmt)?;
            }
            _ => {}
        }

        let mut state = self.widget_state.cell.borrow_mut();
        state.prev_bounds = Some(self.bounds);
        state.prev_display_state = Some(self.display_state);

        Ok(())
    }
}

impl<'a> GameWidget<'a> {
    fn is_full_refresh_required(&self) -> bool {
        let state = self.widget_state.cell.borrow();

        let bounds_changed = state
            .prev_bounds
            .map(|prev_bounds| prev_bounds != self.bounds)
            .unwrap_or(true);
        let display_state_changed = state
            .prev_display_state
            .map(|prev_display_state| {
                prev_display_state != self.display_state && self.state_requires_refresh()
            })
            .unwrap_or(true);

        bounds_changed || display_state_changed
    }

    fn state_requires_refresh(&self) -> bool {
        match self.display_state {
            DisplayState::WinMessageOpen => false,
            _ => true,
        }
    }

    fn write_area(&self, area_id: AreaId, fmt: &mut fmt::Formatter) -> fmt::Result {
        let bounds = bounds_for_area(area_id, self.bounds);

        info!("Printing {:?} at {:?}", area_id, bounds.origin);

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

    fn write_win(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let widget = WinWidget {
            bounds: self.bounds,
        };

        write!(fmt, "{}", widget)?;

        Ok(())
    }
}

fn bounds_for_area(area_id: AreaId, widget_bounds: geometry::Rect<u16>) -> geometry::Rect<u16> {
    let top_left = coords_for_area(area_id);

    match area_id {
        AreaId::Stock => {
            let right = coords_for_area(AreaId::Talon).x - 1;
            let bottom = coords_for_area(AreaId::Tableaux(0)).y - 1;
            let bottom_right = geometry::point2(right, bottom);

            geometry::Box2D::new(top_left, bottom_right).to_rect()
        }
        AreaId::Talon => {
            let first_suit = Suit::try_from(0).unwrap();
            let right = coords_for_area(AreaId::Foundation(first_suit)).x - 1;
            let bottom = coords_for_area(AreaId::Tableaux(0)).y - 1;
            let bottom_right = geometry::point2(right, bottom);

            geometry::Box2D::new(top_left, bottom_right).to_rect()
        }
        AreaId::Foundation(suit) => {
            let next_suit = Suit::try_from(u8::from(suit) + 1).ok();
            let right = if let Some(next_suit) = next_suit {
                coords_for_area(AreaId::Foundation(next_suit)).x - 1
            } else {
                widget_bounds.max_x()
            };
            let bottom = coords_for_area(AreaId::Tableaux(0)).y - 1;
            let bottom_right = geometry::point2(right, bottom);

            geometry::Box2D::new(top_left, bottom_right).to_rect()
        }
        AreaId::Tableaux(index) => {
            let right = coords_for_area(AreaId::Tableaux(index + 1)).x - 1;
            let bottom = widget_bounds.max_y();
            let bottom_right = geometry::point2(right, bottom);

            geometry::Box2D::new(top_left, bottom_right).to_rect()
        }
    }
}

fn coords_for_area(area_id: AreaId) -> geometry::Point2D<u16> {
    let card_offset = geometry::vec2(CARD_SIZE.width, 0);

    match area_id {
        AreaId::Stock => *STOCK_COORDS,
        AreaId::Talon => *TALON_COORDS,
        AreaId::Foundation(suit) => {
            *FOUNDATION_COORDS + (card_offset + *COLUMN_OFFSET) * u16::from(u8::from(suit))
        }
        AreaId::Tableaux(index) => {
            *TABLEAUX_COORDS + (card_offset + *COLUMN_OFFSET) * u16::from(index)
        }
    }
}
