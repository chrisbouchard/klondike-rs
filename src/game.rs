extern crate failure;

use std::cmp::Ordering;
use std::slice::Iter;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid rank: {}", value)]
    InvalidRank {
        value: u8
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;


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


#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub fn color(&self) -> Color {
        match self {
            Suit::Clubs | Suit::Spades => Color::Black,
            Suit::Diamonds | Suit::Hearts => Color::Red
        }
    }

    pub fn symbol(&self) -> String {
        match self {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠"
        }.to_string()
    }

    pub fn iterator() -> Iter<'static, Suit> {
        /* Canonical order. */
        static SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds];
        SUITS.iter()
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct Card {
    pub face_up: bool,
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn color(&self) -> Color {
        self.suit.color()
    }
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


#[derive(Copy, Clone, Debug)]
pub struct CardStack<'a> {
    pub pile: &'a [Card],
    pub fanned: &'a [Card],
}


#[derive(Debug)]
pub struct KlondikeGame {
    stock: Vec<Card>,
    talon_pile: Vec<Card>,
    talon_fanned: Vec<Card>,

    foundation_clubs: Vec<Card>,
    foundation_diamonds: Vec<Card>,
    foundation_hearts: Vec<Card>,
    foundation_spades: Vec<Card>,

    tableaux: Vec<Vec<Card>>,
}

impl KlondikeGame {
    pub fn stock(&self) -> CardStack {
        CardStack {
            pile: &self.stock,
            fanned: &[],
        }
    }

    pub fn talon(&self) -> CardStack {
        CardStack {
            pile: &self.talon_pile,
            fanned: &self.talon_fanned,
        }
    }

    pub fn foundation_for_suit(&self, suit: &Suit) -> CardStack {
        CardStack {
            pile: match suit {
                Suit::Clubs => &self.foundation_clubs,
                Suit::Diamonds => &self.foundation_diamonds,
                Suit::Hearts => &self.foundation_hearts,
                Suit::Spades => &self.foundation_spades
            },
            fanned: &[],
        }
    }

    pub fn foundation(&self) -> impl Iterator<Item=(Suit, CardStack)> {
            Suit::iterator()
                .map(|suit| (suit.clone(), self.foundation_for_suit(suit)))
                /* Collect into a temporary vector to force the map(...) to be evaluated *now*,
                 * ending the borrow on self. */
                .collect::<Vec<_>>()
                .into_iter()
    }

    pub fn tableaux_stack(&self, index: usize) -> Option<CardStack> {
        self.tableaux.get(index)
            .map(|tableaux| CardStack {
                pile: &[],
                fanned: tableaux,
            })
    }

    pub fn tableaux(&self) -> impl Iterator<Item=CardStack> {
        self.tableaux.iter()
            .map(|tableaux| CardStack {
                pile: &[],
                fanned: tableaux,
            })
    }
}
