use std::cmp::Ordering;

use super::{Error, Result};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Black,
    Red,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rank(pub u8);

impl Rank {
    pub fn new(value: u8) -> Result<Rank> {
        match value {
            1...13 => Ok(Rank(value)),
            _ => Err(Error::InvalidRank { value }),
        }
    }

    pub fn value(self) -> u8 {
        let Rank(value) = self;
        value
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
            Rank(value @ 2...10) => format!("{}", value),
            Rank(11) => "J".to_string(),
            Rank(12) => "Q".to_string(),
            Rank(13) => "K".to_string(),
            _ => "E".to_string(),
        }
    }

    pub fn values() -> impl Iterator<Item = Rank> {
        (1..14).map(Rank)
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub fn color(self) -> Color {
        match self {
            Suit::Clubs | Suit::Spades => Color::Black,
            Suit::Diamonds | Suit::Hearts => Color::Red,
        }
    }

    pub fn symbol(self) -> String {
        match self {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        }
        .to_string()
    }

    pub fn values() -> impl Iterator<Item = Suit> {
        /* Canonical order. */
        static SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds];
        SUITS.iter().cloned()
    }

    pub fn index(self) -> usize {
        match self {
            Suit::Clubs => 0,
            Suit::Spades => 1,
            Suit::Hearts => 2,
            Suit::Diamonds => 3,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Card {
    pub face_up: bool,
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn color(&self) -> Color {
        self.suit.color()
    }

    pub fn face_up(mut self) -> Self {
        self.face_up = true;
        self
    }

    pub fn face_down(mut self) -> Self {
        self.face_up = false;
        self
    }
}

impl PartialOrd<Card> for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        if self.suit == other.suit {
            Some(self.rank.cmp(&other.rank))
        } else {
            None
        }
    }
}
