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

pub trait SplitOffAround: Sized {
    type Item: Sized;
    fn split_off_around(&mut self, at: usize) -> (Option<Self::Item>, Self);
}

impl<T> SplitOffAround for Vec<T> {
    type Item = T;

    fn split_off_around(&mut self, at: usize) -> (Option<Self::Item>, Self) {
        if at == self.len() {
            return (None, vec![]);
        }

        let rest = self.split_off(at + 1);
        let item = self.pop();

        (item, rest)
    }
}
