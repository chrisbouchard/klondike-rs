use std::iter::{once, successors};

use crate::game::card::*;
use crate::game::deck::*;
use crate::game::stack::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum GameArea {
    Stock,
    Talon,
    Foundation(usize),
    Tableaux(usize),
}

#[derive(Debug)]
struct GameHolding {
    source: GameArea,
    cards: Vec<Card>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum GameSelection {
    Stock,
    Talon,
    Foundation(usize),
    Tableaux(usize, TableauxSelection),
}

#[derive(Debug)]
pub struct TableauxSpread {
    cards: Vec<Card>,
    revealed_len: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TableauxSelection {
    Cards(usize),
    Stack,
}

#[derive(Debug)]
pub struct KlondikeGame {
    stock: Vec<Card>,

    talon: Vec<Card>,
    talon_len: usize,

    foundation: Vec<Vec<Card>>,

    tableaux: Vec<TableauxSpread>,

    holding: Option<GameHolding>,
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
                revealed_len: 1,
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

            foundation: vec![
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],

            tableaux,

            holding: None,
            selection: GameSelection::Stock,
        }
    }


    pub fn stock(&self) -> Stack {
        Stack::with_cards(
            &self.stock,
            StackDetails {
                len: self.stock.len(),
                visible_len: 2,
                spread_len: 1,
                selection: match self.selection {
                    GameSelection::Stock => Some(StackSelection::Cards(1)),
                    _ => None
                },
            },
        )
    }

    pub fn talon(&self) -> Stack {
        if let Some(ref holding) = self.holding {
            match self.selection {
                GameSelection::Talon =>
                    Stack::with_floating_cards(
                        &self.talon,
                        &holding.cards,
                        StackDetails {
                            len: self.talon.len() + holding.cards.len(),
                            visible_len: self.talon_len + holding.cards.len() + 1,
                            spread_len: self.talon_len + holding.cards.len(),
                            selection: Some(StackSelection::Cards(holding.cards.len())),
                        },
                    ),
                _ =>
                    Stack::with_cards(
                        &self.talon,
                        StackDetails {
                            len: self.talon.len(),
                            visible_len: self.talon_len + 1,
                            spread_len: self.talon_len,
                            selection: match self.selection {
                                GameSelection::Talon => Some(StackSelection::Cards(1)),
                                _ => None
                            },
                        },
                    )
            }
        } else {
            Stack::with_cards(
                &self.talon,
                StackDetails {
                    len: self.talon.len(),
                    visible_len: self.talon_len + 1,
                    spread_len: self.talon_len,
                    selection: match self.selection {
                        GameSelection::Talon => Some(StackSelection::Cards(1)),
                        _ => None
                    },
                },
            )
        }
    }

    pub fn foundation(&self) -> impl Iterator<Item=Stack> {
        self.foundation.iter().enumerate()
            .map(|(index, cards)| self.foundation_helper(index, cards))
            /* Collect into a temporary vector to force the map(...) to be evaluated *now*,
             * ending the borrow on self. */
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn foundation_helper<'a>(&'a self, index: usize, cards: &'a [Card]) -> Stack<'a> {
        if let Some(ref holding) = self.holding {
            match self.selection {
                GameSelection::Foundation(selected_index) if index == selected_index =>
                    Stack::with_floating_cards(
                        cards,
                        &holding.cards,
                        StackDetails {
                            len: cards.len() + holding.cards.len(),
                            visible_len: 2,
                            spread_len: 1,
                            selection: Some(StackSelection::Stack(1)),
                        },
                    ),
                _ =>
                    Stack::with_cards(
                        cards,
                        StackDetails {
                            len: cards.len(),
                            visible_len: 2,
                            spread_len: 1,
                            selection: None,
                        },
                    )
            }
        } else {
            Stack::with_cards(
                cards,
                StackDetails {
                    len: cards.len(),
                    visible_len: 2,
                    spread_len: 1,
                    selection: match self.selection {
                        GameSelection::Foundation(selected_index) if index == selected_index =>
                            Some(StackSelection::FullStack),
                        _ => None
                    },
                },
            )
        }
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
        if let Some(ref holding) = self.holding {
            match self.selection {
                GameSelection::Tableaux(selected_index, _) if index == selected_index =>
                    Stack::with_floating_cards(
                        &spread.cards,
                        &holding.cards,
                        StackDetails {
                            len: spread.cards.len() + holding.cards.len(),
                            visible_len: spread.cards.len() + holding.cards.len(),
                            spread_len: spread.revealed_len + holding.cards.len(),
                            selection: Some(StackSelection::Stack(holding.cards.len())),
                        },
                    ),
                _ =>
                    Stack::with_cards(
                        &spread.cards,
                        StackDetails {
                            len: spread.cards.len(),
                            visible_len: spread.cards.len(),
                            spread_len: spread.revealed_len,
                            selection: None,
                        },
                    ),
            }
        } else {
            Stack::with_cards(
                &spread.cards,
                StackDetails {
                    len: spread.cards.len(),
                    visible_len: spread.cards.len(),
                    spread_len: spread.revealed_len,
                    selection: match self.selection {
                        GameSelection::Tableaux(selected_index, tableaux_selection) if index == selected_index =>
                            match tableaux_selection {
                                TableauxSelection::Cards(len) =>
                                    Some(StackSelection::Cards(len)),
                                TableauxSelection::Stack =>
                                    Some(StackSelection::FullStack),
                            }
                        _ => None
                    },
                },
            )
        }
    }


    pub fn move_to_stock(&mut self) {
        if let Some(selection) = self.first_valid_move(once(GameSelection::Stock)) {
            self.selection = selection;
        }
    }

    pub fn move_to_talon(&mut self) {
        if let Some(selection) = self.first_valid_move(once(GameSelection::Talon)) {
            self.selection = selection;
        }
    }

    pub fn move_to_tableaux(&mut self, index: usize) {
        let moves_iter =
            once(GameSelection::Tableaux(index, TableauxSelection::Cards(1)));

        if let Some(selection) = self.first_valid_move(moves_iter) {
            self.selection = selection;
        }
    }

    pub fn move_left(&mut self) {
        let moves_iter =
            successors(Some(self.selection), |selection| {
                self.next_selection_left(*selection)
            }).skip(1);  // Skip first result because it's just self.selection.

        if let Some(selection) = self.first_valid_move(moves_iter) {
            self.selection = selection;
        }
    }

    pub fn move_right(&mut self) {
        let moves_iter =
            successors(Some(self.selection), |selection| {
                self.next_selection_right(*selection)
            }).skip(1);  // Skip first result because it's just self.selection.

        if let Some(selection) = self.first_valid_move(moves_iter) {
            self.selection = selection;
        }
    }

    pub fn move_up(&mut self) {
        if let GameSelection::Tableaux(index, TableauxSelection::Cards(len)) = self.selection {
            let moves_iter =
                once(GameSelection::Tableaux(index, TableauxSelection::Cards(len + 1)));

            if let Some(selection) = self.first_valid_move(moves_iter) {
                self.selection = selection;
            }
        }
    }

    pub fn move_down(&mut self) {
        if let GameSelection::Tableaux(index, TableauxSelection::Cards(len)) = self.selection {
            let moves_iter =
                once(GameSelection::Tableaux(index, TableauxSelection::Cards(len - 1)));

            if let Some(selection) = self.first_valid_move(moves_iter) {
                self.selection = selection;
            }
        }
    }


    fn first_valid_move(&self, moves_iter: impl Iterator<Item=GameSelection>) -> Option<GameSelection> {
        moves_iter.filter(|selection| {
            debug!("first_valid_move: {:?}", selection);
            self.is_valid_selection(*selection)
        }).next()
    }

    fn next_selection_left(&self, selection: GameSelection) -> Option<GameSelection> {
        match selection {
            GameSelection::Stock => None,
            GameSelection::Talon => Some(GameSelection::Stock),
            GameSelection::Foundation(0) => Some(GameSelection::Talon),
            GameSelection::Foundation(index) => Some(GameSelection::Foundation(index - 1)),
            GameSelection::Tableaux(0, _) => Some(GameSelection::Foundation(3)),
            GameSelection::Tableaux(index, tableaux_selection) =>
                Some(GameSelection::Tableaux(index - 1, tableaux_selection)),
        }
    }

    fn next_selection_right(&self, selection: GameSelection) -> Option<GameSelection> {
        match selection {
            GameSelection::Stock => Some(GameSelection::Talon),
            GameSelection::Talon => Some(GameSelection::Foundation(0)),
            GameSelection::Foundation(index) if index < 3 =>
                Some(GameSelection::Foundation(index + 1)),
            GameSelection::Foundation(_) =>
                Some(GameSelection::Tableaux(0, TableauxSelection::Cards(1))),
            GameSelection::Tableaux(index, tableaux_selection) if index < 6 =>
                Some(GameSelection::Tableaux(index + 1, tableaux_selection)),
            GameSelection::Tableaux(_, _) => None
        }
    }

    fn is_valid_selection(&self, selection: GameSelection) -> bool {
        if let Some(ref holding) = self.holding {
            match selection {
                GameSelection::Stock => false,
                GameSelection::Talon => holding.source == GameArea::Talon,
                GameSelection::Foundation(index) =>
                    if let Some((card, &[])) = holding.cards.split_first() {
                        match self.foundation[index].last() {
                            Some(foundation_card) =>
                                index == card.suit.index()
                                    && card.rank.is_followed_by(foundation_card.rank),
                            None =>
                                index == card.suit.index()
                                    && card.rank.is_ace()
                        }
                    } else {
                        false
                    },
                GameSelection::Tableaux(index, _) =>
                    if let Some(card) = holding.cards.first() {
                        match self.tableaux[index].cards.last() {
                            Some(tableaux_card) =>
                                tableaux_card.face_up
                                    && tableaux_card.rank.is_followed_by(card.rank)
                                    && tableaux_card.color() != card.color(),
                            None =>
                                card.rank.is_king()
                        }
                    } else {
                        false
                    },
            }
        } else {
            match selection {
                GameSelection::Stock => true,
                GameSelection::Talon => !self.talon.is_empty(),
                GameSelection::Foundation(index) => !self.foundation[index].is_empty(),
                GameSelection::Tableaux(index, tableaux_spread) => {
                    let spread = &self.tableaux[index];
                    match tableaux_spread {
                        TableauxSelection::Cards(len) =>
                            !spread.cards.is_empty()
                                && 0 < len
                                && len <= spread.revealed_len,
                        TableauxSelection::Stack => !spread.cards.is_empty(),
                    }
                }
            }
        }
    }
}
