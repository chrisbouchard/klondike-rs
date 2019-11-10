use std::{convert::TryInto, fmt};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Black,
    Red,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Rank {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn is_followed_by(self, other: Rank) -> bool {
        u8::from(self) + 1 == u8::from(other)
    }

    pub fn values() -> impl Iterator<Item = Rank> {
        (1u8..14).map(|value| value.try_into().unwrap())
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rank::Ace => write!(fmt, "A"),
            Rank::Jack => write!(fmt, "J"),
            Rank::Queen => write!(fmt, "Q"),
            Rank::King => write!(fmt, "K"),
            &rank => write!(fmt, "{}", u8::from(rank)),
        }
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Ord,
    PartialOrd,
    IntoPrimitive,
    TryFromPrimitive,
    Display,
)]
#[repr(u8)]
pub enum Suit {
    #[display(fmt = "♠")]
    Spades,
    #[display(fmt = "♥")]
    Hearts,
    #[display(fmt = "♦")]
    Diamonds,
    #[display(fmt = "♣")]
    Clubs,
}

impl Suit {
    pub fn color(self) -> Color {
        match self {
            Suit::Clubs | Suit::Spades => Color::Black,
            Suit::Diamonds | Suit::Hearts => Color::Red,
        }
    }

    pub fn values() -> impl Iterator<Item = Suit> {
        (0u8..4).map(|value| value.try_into().unwrap())
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
