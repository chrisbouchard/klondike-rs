use std::ops;

#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq, Eq)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl ops::Add<Coords> for Coords {
    type Output = Coords;

    fn add(self, other: Coords) -> Self::Output {
        Coords {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign<Coords> for Coords {
    fn add_assign(&mut self, other: Coords) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::Sub<Coords> for Coords {
    type Output = Coords;

    fn sub(self, other: Coords) -> Self::Output {
        Coords {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::SubAssign<Coords> for Coords {
    fn sub_assign(&mut self, other: Coords) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl ops::Mul<i32> for Coords {
    type Output = Coords;

    fn mul(self, scalar: i32) -> Self::Output {
        Coords {
            x: scalar * self.x,
            y: scalar * self.y,
        }
    }
}

impl ops::MulAssign<i32> for Coords {
    fn mul_assign(&mut self, scalar: i32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl ops::Mul<Coords> for i32 {
    type Output = Coords;

    fn mul(self, coords: Coords) -> Self::Output {
        coords.mul(self)
    }
}
