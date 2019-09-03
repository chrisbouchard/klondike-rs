pub trait CharacterLength {
    fn char_len(&self) -> usize;
}

impl CharacterLength for String {
    fn char_len(&self) -> usize {
        self.chars().count()
    }
}

impl CharacterLength for str {
    fn char_len(&self) -> usize {
        self.chars().count()
    }
}

impl<'a> CharacterLength for &'a str {
    fn char_len(&self) -> usize {
        self.chars().count()
    }
}
