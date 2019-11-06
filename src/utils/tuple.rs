pub fn both<T, U>(t: Option<T>, u: Option<U>) -> Option<(T, U)> {
    if let (Some(t), Some(u)) = (t, u) {
        Some((t, u))
    } else {
        None
    }
}
