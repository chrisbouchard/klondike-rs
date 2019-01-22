extern crate failure;

use std::cmp::Ordering;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid rank: {}", value)]
    InvalidRank {
        value: u8
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;


#[derive(Debug, Eq, PartialEq)]
pub enum Color {
    BLACK,
    RED,
}


#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rank(pub u8);

impl Rank {
    pub fn new(value: u8) -> Result<Rank> {
        match value {
            1...13 => Ok(Rank(value)),
            _ => Err(Error::InvalidRank { value })
        }
    }

    pub fn label(&self) -> String {
        match self {
            Rank(1) => "A".to_string(),
            Rank(value @ 2...10) => format!("{}", value),
            Rank(11) => "J".to_string(),
            Rank(12) => "Q".to_string(),
            Rank(13) => "K".to_string(),
            _ => "E".to_string()
        }
    }
}


#[derive(Debug, Eq, PartialEq)]
pub enum Suit {
    CLUBS,
    DIAMONDS,
    HEARTS,
    SPADES,
}

impl Suit {
    pub fn color(&self) -> Color {
        match self {
            Suit::CLUBS | Suit::SPADES => Color::BLACK,
            Suit::DIAMONDS | Suit::HEARTS => Color::RED
        }
    }

    pub fn symbol(&self) -> String {
        match self {
            Suit::CLUBS => "♣",
            Suit::DIAMONDS => "♦",
            Suit::HEARTS => "♥",
            Suit::SPADES => "♠"
        }.to_string()
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct Card {
    pub face_up: bool,
    pub rank: Rank,
    pub suit: Suit,
}

impl PartialOrd<Card> for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        if self.suit == other.suit {
            Option::Some(self.rank.cmp(&other.rank))
        } else {
            Option::None
        }
    }
}


#[derive(Debug)]
pub struct CardStack {
    pub pile: Vec<Card>,
    pub fanned: Vec<Card>
}
