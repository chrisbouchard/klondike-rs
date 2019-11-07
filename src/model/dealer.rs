use std::fmt;

use rand::{seq::SliceRandom, thread_rng};

use super::{area, area_list, settings, Card, Game, Rank, Suit};
use crate::utils::vec::SplitOffBounded;

pub trait Dealer {
    fn deal_game(&self, settings: &settings::GameSettings) -> Game;
}

pub fn create_dealer(mode: settings::DealerMode) -> Box<dyn Dealer> {
    match mode {
        settings::DealerMode::AutoWin => Box::new(AutoWinDealer),
        settings::DealerMode::InOrder => Box::new(StandardDealer {
            shuffle: InOrderShuffle,
        }),
        settings::DealerMode::Random => Box::new(StandardDealer {
            shuffle: RandomShuffle,
        }),
    }
}

#[derive(Debug)]
struct AutoWinDealer;

impl Dealer for AutoWinDealer {
    fn deal_game(&self, settings: &settings::GameSettings) -> Game {
        let stock = area::stock::UnselectedStock::create(vec![], settings);
        let talon = area::talon::UnselectedTalon::create(vec![], 0);

        let mut tableaux = settings
            .tableaux_indices()
            .map(|index| area::tableaux::UnselectedTableaux::create(index, 0, vec![]))
            .collect::<Vec<_>>();

        let mut foundation = Suit::values()
            .map(|suit| {
                let cards = Rank::values()
                    .map(|rank| Card { suit, rank })
                    .collect::<Vec<_>>();
                area::foundation::UnselectedFoundation::create(suit, cards, settings)
            })
            .collect::<Vec<_>>();

        let mut areas: Vec<Box<dyn area::UnselectedArea>> = vec![stock, talon];
        areas.append(&mut foundation);
        areas.append(&mut tableaux);

        let areas = area_list::AreaList::new(areas).expect("Unable to create AreaList");
        Game::new(areas)
    }
}

#[derive(Debug)]
struct StandardDealer<S>
where
    S: Shuffle + fmt::Debug,
{
    shuffle: S,
}

impl<S> Dealer for StandardDealer<S>
where
    S: Shuffle + fmt::Debug,
{
    fn deal_game(&self, settings: &settings::GameSettings) -> Game {
        let mut deck = S::create_deck();

        let mut tableaux = settings
            .tableaux_indices()
            .map(|index| {
                let cards = deck.split_off_bounded(usize::from(index) + 1);
                area::tableaux::UnselectedTableaux::create(index, 1, cards)
            })
            .collect::<Vec<_>>();

        let stock = area::stock::UnselectedStock::create(deck, settings);
        let talon = area::talon::UnselectedTalon::create(vec![], 0);

        let mut foundation = Suit::values()
            .map(|index| area::foundation::UnselectedFoundation::create(index, vec![], settings))
            .collect::<Vec<_>>();

        let mut areas: Vec<Box<dyn area::UnselectedArea>> = vec![stock, talon];
        areas.append(&mut foundation);
        areas.append(&mut tableaux);

        let areas = area_list::AreaList::new(areas).expect("Unable to create AreaList");
        Game::new(areas)
    }
}

trait Shuffle {
    fn create_deck() -> Vec<Card>;
}

#[derive(Debug)]
struct InOrderShuffle;

impl Shuffle for InOrderShuffle {
    fn create_deck() -> Vec<Card> {
        let mut cards = Vec::new();

        for suit in Suit::values() {
            for rank in Rank::values() {
                cards.push(Card { rank, suit })
            }
        }

        cards
    }
}

#[derive(Debug)]
struct RandomShuffle;

impl Shuffle for RandomShuffle {
    fn create_deck() -> Vec<Card> {
        let mut deck = InOrderShuffle::create_deck();
        deck.shuffle(&mut thread_rng());
        deck
    }
}
