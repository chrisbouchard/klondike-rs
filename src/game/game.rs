use crate::game::deck::*;

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
    pub fn new(deck: &mut Deck) -> KlondikeGame {
        let mut tableaux: Vec<Vec<Card>> = Vec::new();

        for i in 0..7 {
            tableaux.push(
                deck.deal(i as usize).into_iter()
                    .chain(deck.deal_one().map(Card::face_up))
                    .collect()
            );
        }

        let talon_fanned =
            deck.deal(3).into_iter()
                .map(Card::face_up)
                .collect();
        let stock = deck.deal_rest();

        KlondikeGame {
            stock,
            talon_pile: vec![],
            talon_fanned,

            foundation_clubs: vec![],
            foundation_diamonds: vec![],
            foundation_hearts: vec![],
            foundation_spades: vec![],

            tableaux
        }
    }

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

    pub fn foundation_for_suit(&self, suit: Suit) -> CardStack {
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
        Suit::values()
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
