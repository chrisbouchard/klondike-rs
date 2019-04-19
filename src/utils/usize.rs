pub trait BoundedSub<T = Self> {
    type Output;
    fn bounded_sub(self, other: T) -> Self::Output;
    fn bounded_sub_with_min(self, other: T, min: Self::Output) -> Self::Output;
}

impl BoundedSub for usize {
    type Output = usize;

    fn bounded_sub(self, other: usize) -> usize {
        if other < self {
            self - other
        } else {
            0
        }
    }

    fn bounded_sub_with_min(self, other: usize, min: usize) -> usize {
        if other + min < self {
            self - other
        } else {
            min
        }
    }
}


pub trait SymmetricDifference<T = Self> {
    type Output;
    fn sym_diff(self, other: T) -> Self::Output;
}

impl SymmetricDifference for usize {
    type Output = usize;

    fn sym_diff(self, other: usize) -> usize {
        if other < self {
            self - other
        } else {
            other - self
        }
    }
}
