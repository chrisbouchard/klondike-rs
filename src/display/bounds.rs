//! Bounds represent an area, defined by the top-left an bottom-right coordinates.

use std::{
    cmp::{max, min, Ordering},
    ops,
};

use super::coords::{self, Coords};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Bounds {
    pub top_left: Coords,
    pub bottom_right: Coords,
}

impl Bounds {
    pub fn new(top_left: Coords, bottom_right: Coords) -> Bounds {
        assert!(top_left <= bottom_right);
        Bounds {
            top_left,
            bottom_right,
        }
    }

    pub fn with_size(top_left: Coords, size: Coords) -> Bounds {
        assert!(size >= coords::ZERO);

        // We need to subtract 1 from each size dimension because our minimum size is 1 x 1; in
        // other words, a Bounds with coords (x, y), (x, y) has size 1 x 1.
        let bottom_right = top_left + size - Coords::from_xy(1, 1);

        Bounds {
            top_left,
            bottom_right,
        }
    }

    pub fn with_size_from_zero(size: Coords) -> Bounds {
        Self::with_size(coords::ZERO, size)
    }

    pub fn width(&self) -> i32 {
        self.bottom_right.x - self.top_left.x + 1
    }

    pub fn height(&self) -> i32 {
        self.bottom_right.y - self.top_left.y + 1
    }

    pub fn contains(&self, coords: Coords) -> bool {
        self.top_left <= coords && coords <= self.bottom_right
    }

    pub fn inset(&self, delta: i32) -> Bounds {
        let inset_top_left = self.top_left + Coords::from_xy(delta, delta);
        let inset_bottom_right = self.bottom_right - Coords::from_xy(delta, delta);

        Bounds::new(inset_top_left, inset_bottom_right)
    }

    pub fn coords_iter(&self) -> impl Iterator<Item = Coords> {
        let Bounds {
            top_left,
            bottom_right,
        } = *self;
        (top_left.y..=bottom_right.y)
            .flat_map(move |y| (top_left.x..=bottom_right.x).map(move |x| Coords::from_xy(x, y)))
    }
}

impl PartialOrd for Bounds {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.top_left < other.top_left && self.bottom_right > other.bottom_right {
            Some(Ordering::Greater)
        } else if self.top_left > other.top_left && self.bottom_right < other.bottom_right {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

impl ops::Add for Bounds {
    type Output = Bounds;

    fn add(self, other: Self) -> Self::Output {
        let mut value = self;
        value += other;
        value
    }
}

impl ops::AddAssign for Bounds {
    fn add_assign(&mut self, other: Self) {
        self.top_left = Coords::from_xy(
            min(self.top_left.x, other.top_left.x),
            min(self.top_left.y, other.top_left.y),
        );

        self.bottom_right = Coords::from_xy(
            max(self.bottom_right.x, other.bottom_right.x),
            max(self.bottom_right.y, other.bottom_right.y),
        );
    }
}
