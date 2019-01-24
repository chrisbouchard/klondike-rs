pub fn index_of_last_n<T>(len: usize, slice: &[T]) -> usize {
    let slice_len = slice.len();

    if len <= slice_len {
        slice_len - len
    } else {
        0
    }
}
