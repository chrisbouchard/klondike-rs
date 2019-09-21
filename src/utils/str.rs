use super::format_str::FormattedString;

pub trait CharacterLength {
    fn char_len(&self) -> usize;
}

impl CharacterLength for str {
    fn char_len(&self) -> usize {
        self.chars().count()
    }
}

impl CharacterLength for FormattedString {
    fn char_len(&self) -> usize {
        self.len()
    }
}
