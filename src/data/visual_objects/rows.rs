use std::collections::HashMap;

use rand::Rng;

use super::row_slices::RowSlice;

use crate::data::{details::patterns::TargetPattern, game_logic::games::SIDE_AMOUNT};

pub struct Row {
    local_rotations: Vec<i32>,
    index: i32,
    slice_collection: HashMap<i32, RowSlice>,
}

impl Row {
    pub fn new(index: i32) -> Self {
        Self {
            local_rotations: Vec::new(),
            index,
            slice_collection: HashMap::from([(
                index,
                RowSlice::new(rand::thread_rng().gen_range(0..=SIDE_AMOUNT - 1), index),
            )]),
        }
    }

    pub fn get_slice(&mut self, index: i32) -> &mut RowSlice {
        self.index = index;
        if let std::collections::hash_map::Entry::Vacant(e) = self.slice_collection.entry(index) {
            e.insert(RowSlice::new(
                rand::thread_rng().gen_range(0..=SIDE_AMOUNT - 1),
                index,
            ));
            self.slice_collection
                .get_mut(&index)
                .expect("Just added the RowSlice to the list. ")
        } else {
            self.slice_collection
                .get_mut(&index)
                .expect("Just checked the RowSlice object exists in the list. ")
        }
    }

    fn contains_pattern(&self, target_pattern: &TargetPattern) -> bool {
        let slice_rotations: Vec<i32> = self
            .slice_collection
            .values()
            .map(|x| x.rotations())
            .collect();
        let slice_len = target_pattern.pattern.len();
        for window in slice_rotations.windows(slice_len) {
            if window == target_pattern.pattern.as_slice() {
                return true;
            }
        }
        false
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.local_rotations == other.local_rotations
            && self.index == other.index
            && self.slice_collection == other.slice_collection
    }
}

#[cfg(test)]
mod row_tests {
    //use std::collections::HashMap;

    //use crate::data::{details::patterns::TargetPattern, visual_objects::row_slices::RowSlice};

    use super::Row;

    fn fake_eq(left: &Row, right: &Row) -> bool {
        left.local_rotations == right.local_rotations && left.index == right.index
    }

    #[test]
    fn new() {
        assert_eq!(fake_eq(&Row::new(0), &Row::new(0)), true);
        assert_eq!(fake_eq(&Row::new(5), &Row::new(5)), true);
        assert_eq!(fake_eq(&Row::new(-1), &Row::new(-1)), true);
    }

    #[test]
    fn get_row_slice() {
        let mut row = Row::new(0);
        assert_eq!(row.get_slice(0).rotations(), row.get_slice(0).rotations());
        assert_eq!(row.get_slice(10).rotations(), row.get_slice(10).rotations());
    }

    //Untestable because random hashmap order.
/*
    fn contains_pattern() {
        let slice_collection: HashMap<i32, RowSlice> = [2, -1, 4, 5]
            .iter()
            .cloned()
            .map(|r| (r, RowSlice::new(r, r as i32)))
            .collect();

        let mut row = Row {
            local_rotations: Vec::new(),
            index: 0,
            slice_collection,
        };
        let mut target_pattern = TargetPattern::new(2);

        target_pattern.set_pattern(Vec::from([2]));
        assert_eq!(row.contains_pattern(&target_pattern), true);

        target_pattern.set_pattern(Vec::from([3, 0]));
        assert_eq!(row.contains_pattern(&target_pattern), true);

        target_pattern.set_pattern(Vec::from([2, 3, 0, 1]));
        assert_eq!(row.contains_pattern(&target_pattern), true);

        target_pattern.set_pattern(Vec::from([4, 0]));
        assert_eq!(row.contains_pattern(&target_pattern), false);
    }
*/
}
