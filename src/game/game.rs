use crate::game::card::*;
use crate::game::deck::*;
use crate::game::stack::*;

#[derive(Copy, Clone, Debug)]
enum GameSelection {
    Stock,
    Talon,

    Foundation(Suit),

    TableauxCards {
        locked_in: bool,
        index: usize,
        len: usize,
    },
}

#[derive(Debug)]
pub struct TableauxSpread {
    cards: Vec<Card>,
    revealed_len: usize,
}

#[derive(Debug)]
pub struct KlondikeGame {
    stock: Vec<Card>,

    talon: Vec<Card>,
    talon_len: usize,

    foundation_clubs: Vec<Card>,
    foundation_diamonds: Vec<Card>,
    foundation_hearts: Vec<Card>,
    foundation_spades: Vec<Card>,

    tableaux: Vec<TableauxSpread>,

    selection: GameSelection,
}

impl KlondikeGame {
    pub fn new(deck: &mut Deck) -> KlondikeGame {
        let mut tableaux: Vec<TableauxSpread> = Vec::new();

        for i in 0..7 {
            tableaux.push(TableauxSpread {
                cards: deck.deal(i as usize).into_iter()
                    .chain(deck.deal_one().map(Card::face_up))
                    .collect(),
                revealed_len: 1
            });
        }

        let talon =
            deck.deal(3).into_iter()
                .map(Card::face_up)
                .collect();
        let stock = deck.deal_rest();

        KlondikeGame {
            stock,

            talon,
            talon_len: 3,

            foundation_clubs: Vec::new(),
            foundation_diamonds: Vec::new(),
            foundation_hearts: Vec::new(),
            foundation_spades: Vec::new(),

            tableaux,

            selection: GameSelection::Stock,
        }
    }

    pub fn stock(&self) -> Stack {
        Stack {
            cards: &self.stock,
            visible_len: 2,
            spread_len: 1,
            selection: match self.selection {
                GameSelection::Stock => Some(StackSelection::Cards(1)),
                _ => None
            },
        }
    }

    pub fn talon(&self) -> Stack {
        Stack {
            cards: &self.talon,
            visible_len: self.talon_len + 1,
            spread_len: self.talon_len,
            selection: match self.selection {
                GameSelection::Talon => Some(StackSelection::Cards(1)),
                _ => None
            },
        }
    }

    pub fn foundation_for_suit(&self, suit: Suit) -> Stack {
        Stack {
            cards: match suit {
                Suit::Clubs => &self.foundation_clubs,
                Suit::Diamonds => &self.foundation_diamonds,
                Suit::Hearts => &self.foundation_hearts,
                Suit::Spades => &self.foundation_spades
            },
            visible_len: 2,
            spread_len: 1,
            selection: match self.selection {
                GameSelection::Foundation(selected_suit) if suit == selected_suit => Some(StackSelection::FullStack),
                _ => None
            },
        }
    }

    pub fn foundation(&self) -> impl Iterator<Item=(Suit, Stack)> {
        Suit::values()
            .map(|suit| (suit.clone(), self.foundation_for_suit(suit)))
            /* Collect into a temporary vector to force the map(...) to be evaluated *now*,
             * ending the borrow on self. */
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn tableaux_stack(&self, index: usize) -> Option<Stack> {
        self.tableaux.get(index)
            .map(|spread| self.tableaux_spread_helper(index, spread))
    }

    pub fn tableaux(&self) -> impl Iterator<Item=Stack> {
        self.tableaux.iter().enumerate()
            .map(|(index, spread)| self.tableaux_spread_helper(index, spread))
            /* Collect into a temporary vector to force the map(...) to be evaluated *now*,
             * ending the borrow on self. */
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn tableaux_spread_helper<'a>(&'a self, index: usize, spread: &'a TableauxSpread) -> Stack<'a> {
        Stack {
            cards: &spread.cards,
            visible_len: spread.cards.len(),
            spread_len: spread.revealed_len,
            selection: match self.selection {
                GameSelection::TableauxCards { locked_in, index: selected_index, len } if index == selected_index =>
                    if locked_in {
                        Some(StackSelection::Stack(len))
                    } else {
                        Some(StackSelection::Cards(len))
                    },
                _ => None
            },
        }
    }
}
