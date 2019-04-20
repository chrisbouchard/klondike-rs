use std::collections::HashMap;
use std::fmt;

use super::{move_selection, Action, Area, AreaId, SelectedArea, UnselectedArea};
use crate::utils::vec::SplitOffAround;
use std::iter::once;

/// A list of [areas](Area) with one [selected](SelectedArea) and the rest
/// [unselected](UnselectedArea) that can efficiently move the selection and map [area ids](AreaId)
/// to areas.
///
/// Our implementation uses a zipper data structure. The selected_area is the zipper head, and the
/// before_areas and after_areas lists function as stacks; we pop areas back and forth to move the
/// selection back and forth.
pub struct AreaList<'a> {
    /// Map from area id to the index of the area in the list
    area_ids: HashMap<AreaId, usize>,
    /// The list of areas before the selected area
    before_areas: Vec<Box<dyn UnselectedArea<'a> + 'a>>,
    /// The selected area and head of the zipper
    selected_area: Box<dyn SelectedArea<'a> + 'a>,
    /// The list of areas after the selected area. This list is kept in reverse order so we can
    /// efficiently push and pop to move left and right.
    after_areas: Vec<Box<dyn UnselectedArea<'a> + 'a>>,
}

// This list is always non-empty
#[allow(clippy::len_without_is_empty)]
impl<'a> AreaList<'a> {
    pub fn new<T, I>(areas: T) -> AreaList<'a>
    where
        T: IntoIterator<Item = Box<dyn UnselectedArea<'a> + 'a>, IntoIter = I>,
        I: Iterator<Item = Box<dyn UnselectedArea<'a> + 'a>>,
    {
        // First collect up the incoming areas into a Vec. This will become after_areas later.
        let mut areas = areas.into_iter().collect::<Vec<_>>();

        // Compute the reverse mapping from area id to index. If there are any duplicate area ids,
        // this will keep only one entry per id. That's fine for right now; we'll deal with it in a
        // moment.
        let area_ids = areas
            .iter()
            .enumerate()
            .map(|(index, area)| (area.id(), index))
            .collect::<HashMap<_, _>>();

        // If we found fewer area ids than we have areas in the list, then at least two areas
        // reported the same area id.
        // TODO: Should we report *which* ids are duplicated?
        if area_ids.len() < areas.len() {
            panic!("Conflicting area ids");
        }

        // Reverse the list of areas, because we keep after_areas in reverse order for efficient
        // pushing and popping (since areas will be coming and going from the "beginning").
        areas.reverse();

        // Pop off the first area and select it. If anything goes wrong we'll immediately panic.
        // TODO: Should we let the user specify a selected index?
        let selected_area = areas
            .pop()
            .expect("Area list must not be empty")
            .select()
            .ok()
            .expect("Unable to select first area");

        AreaList {
            area_ids,
            before_areas: vec![],
            selected_area,
            after_areas: areas,
        }
    }

    pub fn len(&self) -> usize {
        self.before_areas.len() + self.after_areas.len() + 1
    }

    pub fn get_by_index(&self, index: usize) -> &dyn Area<'a> {
        let selected_area_id = self.selected_area.id();
        let selected_index = self.get_index(selected_area_id);

        if index == selected_index {
            self.selected_area.as_area()
        } else if index < selected_index {
            self.before_areas[index].as_area()
        } else {
            debug!(
                "index: {:?}, selected_index: {:?}, self: {:?}",
                index, selected_index, self
            );
            let effective_index = index - selected_index - 1;
            // Since after_areas is reversed, we have to get an index "from the end".
            let actual_index = self.after_areas.len() - effective_index - 1;
            self.after_areas[actual_index].as_area()
        }
    }

    pub fn get_by_index_mut(&mut self, index: usize) -> &mut dyn Area<'a> {
        let selected_area_id = self.selected_area.id();
        let selected_index = self.get_index(selected_area_id);

        if index == selected_index {
            self.selected_area.as_area_mut()
        } else if index < selected_index {
            self.before_areas[index].as_area_mut()
        } else {
            let effective_index = index - selected_index - 1;
            // Since after_areas is reversed, we have to get an index "from the end".
            let actual_index = self.after_areas.len() - effective_index - 1;
            self.after_areas[actual_index].as_area_mut()
        }
    }

    pub fn get_by_area_id(&self, area_id: AreaId) -> &dyn Area<'a> {
        let index = self.get_index(area_id);
        self.get_by_index(index)
    }

    pub fn get_by_area_id_mut(&mut self, area_id: AreaId) -> &mut dyn Area<'a> {
        let index = self.get_index(area_id);
        self.get_by_index_mut(index)
    }

    pub fn selected(&self) -> &dyn SelectedArea<'a> {
        self.selected_area.as_ref()
    }

    pub fn selected_mut(&mut self) -> &mut dyn SelectedArea<'a> {
        self.selected_area.as_mut()
    }

    #[allow(clippy::redundant_closure)]
    pub fn iter<'b>(&'b self) -> impl Iterator<Item = &'b Area<'a>>
    where
        'a: 'b,
    {
        let before_iter = self.before_areas.iter().map(|area| area.as_area());
        let after_iter = self.after_areas.iter().map(|area| area.as_area()).rev();

        before_iter
            .chain(once(self.selected_area.as_area()))
            .chain(after_iter)
    }

    #[allow(clippy::redundant_closure)]
    pub fn iter_left_from_selection<'b>(&'b self) -> impl Iterator<Item = &'b Area<'a>>
    where
        'a: 'b,
    {
        let before_iter = self.before_areas.iter().map(|area| area.as_area()).rev();
        let after_iter = self.after_areas.iter().map(|area| area.as_area());

        before_iter
            .chain(after_iter)
            .chain(once(self.selected_area.as_area()))
    }

    #[allow(clippy::redundant_closure)]
    pub fn iter_right_from_selection<'b>(&'b self) -> impl Iterator<Item = &'b Area<'a>>
    where
        'a: 'b,
    {
        let before_iter = self.before_areas.iter().map(|area| area.as_area());
        let after_iter = self.after_areas.iter().map(|area| area.as_area()).rev();

        after_iter
            .chain(before_iter)
            .chain(once(self.selected_area.as_area()))
    }

    pub fn move_selection(mut self, target_area_id: AreaId) -> (Self, bool) {
        let selected_area_id = self.selected_area.id();

        // No work to do if we're already where we want to end up.
        if selected_area_id == target_area_id {
            return (self, true);
        }

        let selected_index = self.get_index(selected_area_id);
        let target_index = self.get_index(target_area_id);

        let target_is_before = target_index < selected_index;

        // Decide how far we'll have to move to get to the target, either into the before_areas
        // or after_areas list. Note that the after_areas list is reversed so we can efficiently
        // pop to move areas.
        //
        // Absolute Index:  0  1  2  3  4  5  6
        // Relative Index:  0  1  2  x  2  1  0
        //                 before... | ...after
        //                        selected
        let relative_index = if target_is_before {
            target_index
        } else {
            self.after_areas.len() - (target_index - selected_index)
        };

        let (target_vec, other_vec) = if target_is_before {
            (&mut self.before_areas, &mut self.after_areas)
        } else {
            (&mut self.after_areas, &mut self.before_areas)
        };

        let (target_area, areas_to_move) = target_vec.split_off_around(relative_index);
        let target_area = target_area.unwrap();

        match move_selection(self.selected_area, target_area) {
            // If we were able to move the selection, move the previously selected area and the
            // intermediate areas over to the other side.
            Ok((unselected_area, target_area)) => {
                self.selected_area = target_area;
                other_vec.push(unselected_area);
                other_vec.extend(areas_to_move.into_iter().rev());

                (self, true)
            }

            // If we were *un*able to move the selection, put everything back where we found it.
            Err((selected_area, target_area)) => {
                self.selected_area = selected_area;
                target_vec.push(target_area);
                target_vec.extend(areas_to_move.into_iter());

                (self, false)
            }
        }
    }

    pub fn activate_selected(mut self) -> Self {
        if let Some(action) = self.selected_area.activate() {
            match action {
                Action::Draw(len) => {
                    let held = self.get_by_area_id_mut(AreaId::Stock).take_cards(len);

                    match self.get_by_area_id_mut(AreaId::Talon).give_cards(held) {
                        Ok(()) => {}
                        Err(held) => self
                            .get_by_area_id_mut(AreaId::Stock)
                            .give_cards(held)
                            .expect("Unable to replace held cards on the stock"),
                    }
                }
                Action::Restock => {
                    let held = self.get_by_area_id_mut(AreaId::Talon).take_all_cards();

                    match self.get_by_area_id_mut(AreaId::Stock).give_cards(held) {
                        Ok(()) => {}
                        Err(held) => self
                            .get_by_area_id_mut(AreaId::Talon)
                            .give_cards(held)
                            .expect("Unable to replace held cards on the talon"),
                    }
                }
            }
        }

        self
    }

    fn get_index(&self, area_id: AreaId) -> usize {
        *self
            .area_ids
            .get(&area_id)
            .unwrap_or_else(|| panic!("Unknown area id {:?}", area_id))
    }
}

impl<'a> fmt::Debug for AreaList<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // We don't assume that Areas implement Debug, so we'll just format them as their area ids.
        // Clippy 0.0.212 wants to replace the following closures with Area::id, but that is not
        // valid because our value is &Box<...>, which requires dereferencing.
        #[allow(clippy::redundant_closure)]
        let before_area_ids = self
            .before_areas
            .iter()
            .map(|area| area.id())
            .collect::<Vec<_>>();
        #[allow(clippy::redundant_closure)]
        let after_area_ids = self
            .after_areas
            .iter()
            .map(|area| area.id())
            .collect::<Vec<_>>();

        fmt.debug_struct("AreaList")
            .field("area_ids", &self.area_ids)
            .field("before_areas", &before_area_ids)
            .field("selected_area", &self.selected_area.id())
            .field("after_areas", &after_area_ids)
            .finish()
    }
}
