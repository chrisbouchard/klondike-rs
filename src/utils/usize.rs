pub trait BoundedSub<T = Self> {
    type Output;
    fn bounded_sub(self, other: T) -> Self::Output;
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
}
