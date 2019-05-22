#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Black,
    Red,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rank(u8);

impl Rank {
    pub fn new(value: u8) -> Rank {
        assert!(1 <= value && value <= 13, "Invalid rank: {}", value);
        Rank(value)
    }

    pub fn value(self) -> u8 {
        self.0
    }

    pub fn is_followed_by(self, other: Rank) -> bool {
        self.value() + 1 == other.value()
    }

    pub fn is_ace(self) -> bool {
        self.value() == 1
    }

    pub fn is_king(self) -> bool {
        self.value() == 13
    }

    pub fn label(self) -> String {
        match self {
            Rank(1) => "A".to_string(),
            Rank(11) => "J".to_string(),
            Rank(12) => "Q".to_string(),
            Rank(13) => "K".to_string(),
            Rank(value) => format!("{}", value),
        }
    }

    pub fn values() -> impl Iterator<Item = Rank> {
        (1..14).map(Rank::new)
    }
}

#[derive(
    Copy, Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, IntoPrimitive, TryFromPrimitive,
)]
#[repr(u8)]
pub enum Suit {
    Spades = 0,
    Hearts = 1,
    Diamonds = 2,
    Clubs = 3,
}

/// Canonical order
static SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

impl Suit {
    pub fn color(self) -> Color {
        match self {
            Suit::Clubs | Suit::Spades => Color::Black,
            Suit::Diamonds | Suit::Hearts => Color::Red,
        }
    }

    pub fn symbol(self) -> String {
        match self {
            Suit::Spades => "♠",
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
        }
        .to_string()
    }

    pub fn values() -> impl Iterator<Item = Suit> {
        SUITS.iter().cloned()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn color(&self) -> Color {
        self.suit.color()
    }
}
