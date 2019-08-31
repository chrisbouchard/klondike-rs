use std::fmt;

use crate::model::stack::{Orientation, Stack};

use super::{
    bounds::Bounds, card::CardWidget, coords::Coords, selector::SelectorWidget, widget::Widget,
};

mod common;
mod horizontal;
mod vertical;

pub struct StackWidget<'a> {
    pub coords: Coords,
    pub stack: &'a Stack<'a>,
}

impl<'a> Widget for StackWidget<'a> {
    fn bounds(&self) -> Bounds {
        let offsets = self.offsets();

        let mut bounds = Bounds::new(self.coords, self.coords);

        for card_widget in self.card_widget_iter(&offsets) {
            bounds += card_widget.bounds();
        }

        if let Some(selector_widget) = self.selector_widget(&offsets) {
            bounds += selector_widget.bounds();
        }

        bounds
    }
}

impl<'a> fmt::Display for StackWidget<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let offsets = self.offsets();

        for card_widget in self.card_widget_iter(&offsets) {
            write!(fmt, "{}", card_widget)?;
        }

        if let Some(selector_widget) = self.selector_widget(&offsets) {
            write!(fmt, "{}", selector_widget)?;
        }

        Ok(())
    }
}

impl<'a> StackWidget<'a> {
    fn offsets(&self) -> Offsets {
        match self.stack.details.orientation {
            Orientation::Horizontal => horizontal::offsets(self),
            Orientation::Vertical => vertical::offsets(self),
        }
    }

    fn card_widget_iter(&'a self, offsets: &'a Offsets) -> impl Iterator<Item = CardWidget<'a>> {
        // We can't just match and return the iterators like we do for the other methods, because
        // they have different opaque iterator types. So we'll create separate variables for both
        // but only populate one, and then we'll chain the optional iterators.
        let mut horizontal_iter = None;
        let mut vertical_iter = None;

        match self.stack.details.orientation {
            Orientation::Horizontal => {
                horizontal_iter = Some(horizontal::card_widget_iter(self, offsets))
            }
            Orientation::Vertical => {
                vertical_iter = Some(vertical::card_widget_iter(self, offsets))
            }
        }

        // Flatten the optional iterators to regular (possibly empty) iterators.
        let horizontal_iter = horizontal_iter.into_iter().flatten();
        let vertical_iter = vertical_iter.into_iter().flatten();

        horizontal_iter.chain(vertical_iter)
    }

    fn selector_widget(&self, offsets: &Offsets) -> Option<SelectorWidget> {
        match self.stack.details.orientation {
            Orientation::Horizontal => horizontal::selector_widget(self, offsets),
            Orientation::Vertical => vertical::selector_widget(self, offsets),
        }
    }
}

#[derive(Clone, Debug)]
struct Offsets {
    pub unspread: Coords,
    pub collapsed_spread: Coords,
    pub uncollapsed_spread: Coords,
    pub selected: Coords,
    pub collapse_unspread_len: usize,
    pub collapse_spread_len: usize,
}
