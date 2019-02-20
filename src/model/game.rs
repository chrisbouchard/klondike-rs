use std::iter::once;

use super::{
    area::{
        foundation::Foundation, stock::Stock, tableaux::Tableaux, talon::Talon, Action, Area,
        AreaId, Selection, SelectionMode,
    },
    card::Card,
    deck::Deck,
    settings::Settings,
    stack::Stack,
};

#[derive(Debug)]
pub struct GameAreas<'a> {
    stock: Stock<'a>,
    talon: Talon<'a>,
    foundation: Vec<Foundation<'a>>,
    tableaux: Vec<Tableaux<'a>>,

    ids: Vec<AreaId>,
}

impl<'a> GameAreas<'a> {
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

    fn area_id_iter<'b>(&'b self, start: AreaId) -> impl DoubleEndedIterator<Item = AreaId> + 'b {
        let equals_pred = |area_id: &AreaId| start == *area_id;

        let (before, after) = if let Some(position) = self.ids.iter().position(equals_pred) {
            self.ids.split_at(position)
        } else {
            let empty: &[AreaId] = &[];
            (empty, empty)
        };

        after.iter().skip(1).chain(before).cloned()
    }

    fn area_id_iter_rev<'b>(
        &'b self,
        start: AreaId,
    ) -> impl DoubleEndedIterator<Item = AreaId> + 'b {
        self.area_id_iter(start).rev()
    }
}

#[derive(Debug)]
pub struct Game<'a> {
    areas: GameAreas<'a>,
    selection: Selection,
    settings: &'a Settings,
}

impl<'a> Game<'a> {
    pub fn new<'d>(deck: &'d mut Deck, settings: &'a Settings) -> Game<'a> {
        let ids = once(AreaId::Stock)
            .chain(once(AreaId::Talon))
            .chain(settings.foundation_indices().map(AreaId::Foundation))
            .chain(settings.tableaux_indices().map(AreaId::Tableaux))
            .collect::<Vec<_>>();

        let tableaux = settings
            .tableaux_indices()
            .map(|index| {
                let cards = deck.deal(index + 1);
                Tableaux::new(index, cards, settings)
            })
            .collect::<Vec<_>>();

        let talon = Talon::new(Vec::new(), 0, settings);

        let stock_cards = deck.deal_rest();
        let stock = Stock::new(stock_cards, settings);

        let foundation = settings
            .foundation_indices()
            .map(|index| Foundation::new(index, Vec::new(), settings))
            .collect();

        let selection = Selection::default();

        Game {
            areas: GameAreas {
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

    pub fn move_to(mut self, area_id: AreaId) -> Game<'a> {
        let mode = self.selection.mode.moved_ref();
        let moves_iter = once(area_id);

        if let Some(area_id) = self.first_valid_move(mode, moves_iter) {
            self.selection = self.selection.move_to(area_id);
        }

        self
    }

    pub fn move_to_foundation(mut self) -> Game<'a> {
        let mode = self.selection.mode.moved_ref();
        let moves_iter = self.settings.foundation_indices().map(AreaId::Foundation);

        if let Some(area_id) = self.first_valid_move(mode, moves_iter) {
            self.selection = self.selection.move_to(area_id);
        }

        self
    }

    pub fn move_left(mut self) -> Game<'a> {
        let mode = self.selection.mode.moved_ref();

        let starting_area_id = self.selection.target;
        let moves_iter = self.areas.area_id_iter_rev(starting_area_id);

        if let Some(area_id) = self.first_valid_move(mode, moves_iter) {
            self.selection = self.selection.move_to(area_id);
        }

        self
    }

    pub fn move_right(mut self) -> Game<'a> {
        let mode = self.selection.mode.moved_ref();

        let starting_area_id = self.selection.target;
        let moves_iter = self.areas.area_id_iter(starting_area_id);

        if let Some(area_id) = self.first_valid_move(mode, moves_iter) {
            self.selection = self.selection.move_to(area_id);
        }

        self
    }

    pub fn move_up(mut self) -> Game<'a> {
        if let SelectionMode::Cards(len) = self.selection.mode {
            let mode = SelectionMode::Cards(len + 1);
            let moves_iter = once(self.selection.target);

            if self.first_valid_move(&mode, moves_iter).is_some() {
                self.selection = self.selection.select(mode);
            }
        }

        self
    }

    pub fn move_down(mut self) -> Game<'a> {
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

    pub fn activate(mut self) -> Game<'a> {
        let selected_area = self.areas.area_mut(self.selection.target);

        match selected_area.activate(&mut self.selection.mode) {
            Some(Action::Draw) => {
                let cards = self.areas.stock.draw();
                self.areas.talon.place(cards);
                self
            }
            Some(Action::MoveTo(area_id)) => self.move_to(area_id),
            Some(Action::Restock) => {
                let cards = self.areas.talon.flip();
                self.areas.stock.place(cards);
                self
            }
            None => self,
        }
    }
}
