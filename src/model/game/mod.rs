use std::iter::once;

use super::card::Card;
use super::deck::Deck;
use super::stack::Stack;

use self::area::{Action, Area, AreaId, Selection, SelectionMode};
use self::foundation::Foundation;
use self::settings::KlondikeGameSettings;
use self::stock::Stock;
use self::tableaux::Tableaux;
use self::talon::Talon;

pub mod area;
pub mod foundation;
pub mod history;
pub mod settings;
pub mod stock;
pub mod tableaux;
pub mod talon;


#[derive(Debug)]
pub struct KlondikeGameAreas<'a> {
    stock: Stock<'a>,
    talon: Talon<'a>,
    foundation: Vec<Foundation<'a>>,
    tableaux: Vec<Tableaux<'a>>,

    ids: Vec<AreaId>,
}

impl<'a> KlondikeGameAreas<'a> {
    fn area(&self, area_id: AreaId) -> &Area {
        match area_id {
            AreaId::Stock => &self.stock,
            AreaId::Talon => &self.talon,
            AreaId::Foundation(index) => &self.foundation[index],
            AreaId::Tableaux(index) => &self.tableaux[index],
        }
    }

    fn area_mut(&mut self, area_id: AreaId) -> &mut Area {
        match area_id {
            AreaId::Stock => &mut self.stock,
            AreaId::Talon => &mut self.talon,
            AreaId::Foundation(index) => &mut self.foundation[index],
            AreaId::Tableaux(index) => &mut self.tableaux[index],
        }
    }
}

#[derive(Debug)]
pub struct KlondikeGame<'a> {
    areas: KlondikeGameAreas<'a>,
    selection: Selection,
    settings: &'a KlondikeGameSettings,
}

impl<'a> KlondikeGame<'a> {
    pub fn new<'d>(deck: &'d mut Deck, settings: &'a KlondikeGameSettings) -> KlondikeGame<'a> {
        let ids = once(AreaId::Stock)
            .chain(once(AreaId::Talon))
            .chain(settings.foundation_indices().map(AreaId::Foundation))
            .chain(settings.tableaux_indices().map(AreaId::Tableaux))
            .collect::<Vec<_>>();

        let tableaux = settings.tableaux_indices()
            .map(|index| {
                let cards = deck
                    .deal(index)
                    .into_iter()
                    .chain(deck.deal_one().map(Card::face_up))
                    .collect::<Vec<_>>();
                Tableaux::new(index, cards, settings)
            })
            .collect::<Vec<_>>();

        let talon = Talon::new(Vec::new(), 0, settings);

        let stock_cards = deck.deal_rest();
        let stock = Stock::new(stock_cards, settings);

        let foundation = settings.foundation_indices()
            .map(|index| Foundation::new(index, Vec::new(), settings))
            .collect();

        let selection = Selection::new();

        KlondikeGame {
            areas: KlondikeGameAreas {
                stock,
                talon,
                foundation,
                tableaux,
                ids,
            },
            selection,
            settings,
        }
    }

    pub fn stack(&self, area_id: AreaId) -> Stack {
        let mode = if self.selection.matches(area_id) {
            Some(&self.selection.mode)
        } else {
            None
        };

        self.areas.area(area_id).as_stack(mode)
    }

    pub fn move_to(mut self, area_id: AreaId) -> KlondikeGame<'a> {
        let mode = self.selection.mode.moved_ref();
        let moves_iter = once(area_id);

        if let Some(area_id) = self.first_valid_move(mode, moves_iter) {
            self.selection = self.selection.move_to(area_id);
        }

        self
    }

    pub fn move_to_foundation(mut self) -> KlondikeGame<'a> {
        let mode = self.selection.mode.moved_ref();
        let moves_iter = self.settings.foundation_indices().map(AreaId::Foundation);

        if let Some(area_id) = self.first_valid_move(mode, moves_iter) {
            self.selection = self.selection.move_to(area_id);
        }

        self
    }

    pub fn move_left(mut self) -> KlondikeGame<'a> {
        let mode = self.selection.mode.moved_ref();

        let starting_area_id = self.selection.target;
        let moves_iter = self
            .areas
            .ids
            .iter()
            .rev()
            .cycle()
            .skip_while(|area_id| **area_id != starting_area_id)
            .skip(1)
            .take_while(|area_id| **area_id != starting_area_id)
            .cloned();

        if let Some(area_id) = self.first_valid_move(mode, moves_iter) {
            self.selection = self.selection.move_to(area_id);
        }

        self
    }

    pub fn move_right(mut self) -> KlondikeGame<'a> {
        let mode = self.selection.mode.moved_ref();

        let starting_area_id = self.selection.target;
        let moves_iter = self
            .areas
            .ids
            .iter()
            .cycle()
            .skip_while(|area_id| **area_id != starting_area_id)
            .skip(1)
            .take_while(|area_id| **area_id != starting_area_id)
            .cloned();

        if let Some(area_id) = self.first_valid_move(mode, moves_iter) {
            self.selection = self.selection.move_to(area_id);
        }

        self
    }

    pub fn move_up(mut self) -> KlondikeGame<'a> {
        if let SelectionMode::Cards(len) = self.selection.mode {
            let mode = SelectionMode::Cards(len + 1);
            let moves_iter = once(self.selection.target);

            if self.first_valid_move(&mode, moves_iter).is_some() {
                self.selection = self.selection.select(mode);
            }
        }

        self
    }

    pub fn move_down(mut self) -> KlondikeGame<'a> {
        if let SelectionMode::Cards(len) = self.selection.mode {
            if len > 1 {
                let mode = SelectionMode::Cards(len - 1);
                let moves_iter = once(self.selection.target);

                if self.first_valid_move(&mode, moves_iter).is_some() {
                    self.selection = self.selection.select(mode);
                }
            }
        }

        self
    }

    fn first_valid_move<I>(&self, mode: &SelectionMode, mut moves_iter: I) -> Option<AreaId>
    where
        I: Iterator<Item = AreaId>,
    {
        moves_iter.find(|area_id| {
            debug!("Checking focus: area: {:?}, mode: {:?}", area_id, mode);
            let area = self.areas.area(*area_id);
            area.accepts_focus(mode)
        })
    }

    pub fn activate(mut self) -> KlondikeGame<'a> {
        let selected_area = self.areas.area_mut(self.selection.target);

        if let Some(action) = selected_area.activate(&mut self.selection.mode) {
            match action {
                Action::Draw => {
                    let cards = self.areas.stock.draw();
                    self.areas.talon.place(cards);
                }
                Action::MoveTo(area_id) => {
                    self = self.move_to(area_id);
                }
                Action::Restock => {
                    let cards = self.areas.talon.flip();
                    self.areas.stock.place(cards);
                }
            }
        }

        self
    }
}
