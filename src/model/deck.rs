use super::card::{Card, Rank, Suit};

#[derive(Clone, Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();

        for suit in Suit::values() {
            for rank in Rank::values() {
                cards.push(Card { rank, suit })
            }
        }

        Deck { cards }
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn cards_mut(&mut self) -> &mut [Card] {
        &mut self.cards
    }

    pub fn deal_one(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn deal(&mut self, count: usize) -> Vec<Card> {
        let len = self.cards.len();

        if count <= len {
            self.cards.split_off(len - count)
        } else {
            vec![]
        }
    }

    pub fn deal_rest(&mut self) -> Vec<Card> {
        self.cards.drain(0..).collect()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Deck::new()
    }
}

impl<'a> From<&'a Deck> for &'a [Card] {
    fn from(deck: &'a Deck) -> Self {
        &deck.cards
    }
}

impl<'a> From<&'a mut Deck> for &'a mut [Card] {
    fn from(deck: &'a mut Deck) -> Self {
        &mut deck.cards
    }
}
