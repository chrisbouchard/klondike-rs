use super::{area, area_list, settings, Card, Game, Rank, Suit};

pub trait Dealer<'a> {
    fn deal_game(&'a mut self) -> Game<'a>;
}

pub fn dealer_for_mode<'a>(settings: &'a settings::GameSettings) -> Box<dyn Dealer<'a> + 'a> {
    // TODO: Match on settings.dealer and choose an implementation.
    Box::new(AutoWinDealer { settings })
}

// TODO: Create the remaining implementations.
struct AutoWinDealer<'a> {
    settings: &'a settings::GameSettings,
}

impl<'a> Dealer<'a> for AutoWinDealer<'a> {
    fn deal_game(&'a mut self) -> Game<'a> {
        let stock = area::stock::UnselectedStock::create(vec![], self.settings);
        let talon = area::talon::UnselectedTalon::create(vec![], 0, self.settings);

        let mut tableaux = self
            .settings
            .tableaux_indices()
            .map(|index| {
                area::tableaux::UnselectedTableaux::create(index, 0, vec![], self.settings)
            })
            .collect::<Vec<_>>();

        let mut foundation = Suit::values()
            .map(|suit| {
                let cards = Rank::values()
                    .map(|rank| Card { suit, rank })
                    .collect::<Vec<_>>();
                area::foundation::UnselectedFoundation::create(suit, cards, self.settings)
            })
            .collect::<Vec<_>>();

        let mut areas: Vec<Box<dyn area::UnselectedArea>> = vec![stock, talon];
        areas.append(&mut foundation);
        areas.append(&mut tableaux);

        let areas = area_list::AreaList::new(areas).expect("Unable to create AreaList");
        let last_area = areas.selected().id();

        Game {
            areas,
            last_area,
            settings: self.settings,
        }
    }
}

fn create_deck() -> Vec<Card> {
    let mut cards = Vec::new();

    for suit in Suit::values() {
        for rank in Rank::values() {
            cards.push(Card { rank, suit })
        }
    }

    cards
}
