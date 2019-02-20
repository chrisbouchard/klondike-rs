use super::usize::BoundedSub;

pub trait SplitOffBounded {
    fn split_off_bounded(&mut self, len: usize) -> Self;
}

impl<T> SplitOffBounded for Vec<T> {
    fn split_off_bounded(&mut self, len: usize) -> Self {
        let split_index = self.len().bounded_sub(len);
        self.split_off(split_index)
    }
}
