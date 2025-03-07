//! This is a generated file! Don't modify it!!!
use {
    crate::{BiMultiMap, HashSet, Rc},
    std::{borrow::Borrow, collections::hash_map::Keys, hash::Hash, ops::Deref},
};

impl<RightType: Hash + Eq, LeftType: Hash + Eq> BiMultiMap<LeftType, RightType> {
    pub fn get_right(&self, right: &RightType) -> Option<&HashSet<Rc<LeftType>>> {
        self.right_map_rc.get(right)
    }

    pub fn get_one_right(&self, right: &RightType) -> Option<&LeftType> {
        self.get_right(right)
            .and_then(|e| e.iter().next())
            .map(Deref::deref)
    }

    pub(crate) fn get_mut_right(
        &mut self,
        right: &RightType,
    ) -> Option<&mut HashSet<Rc<LeftType>>> {
        self.right_map_rc.get_mut(right)
    }

    pub fn get_right_vec(&self, right: &RightType) -> Option<Vec<&LeftType>> {
        self.right_map_rc
            .get(right)
            .map(|map| map.iter().map(Deref::deref).collect())
    }

    pub fn remove_right<RightRef: Borrow<RightType>>(
        &mut self,
        right: RightRef,
    ) -> Option<HashSet<Rc<LeftType>>> {
        let right = right.borrow();
        match self.right_map_rc.remove(right) {
            Some(left_set) => {
                left_set.iter().for_each(|left| {
                    let is_empty = self.get_mut_left(left).map(|hashet_right| {
                        hashet_right.remove(right);

                        hashet_right.is_empty()
                    });

                    self.len -= 1;

                    if is_empty.is_some_and(|b| b) {
                        self.left_map_rc.remove(left);
                    }
                });

                Some(left_set)
            },
            None => None,
        }
    }

    // Setter
    /// Set a left_value by the right key
    ///
    /// The argument passed is an [Rc] for the `right_key`, since it might be inserted as a value
    /// for all the left_values that are in `left_values` but not present in the [`BiMultiMap`]
    pub fn set_right(&mut self, right_key: Rc<RightType>, left_values: HashSet<LeftType>) {
        // These 2 declarations, demonstrate perfectly what this function does.
        //
        // We have on one side what Ill call `bmm_left_values` (BiMultiMapLeftValues) and
        // `left_values`, the values passed in parameter.
        //
        // We therefore do 3 checks:
        // 1. If a value of `bmm_left_values` is not in `left_values` => We want to remove it from
        //   `bmm_left_values`. This is `to_be_removed`.
        // 2. If a value is both in `bmm_left_values` and `left_values` => Value is alreay present
        //    in both. No need to modify anything.
        //  3. If a value of `left_values` is not in `bmm_left_values` => We want to add it to
        //     `bmm_left_values`. This is `to_add_as_value_of_left`.
        let mut to_add_as_value_of_left = HashSet::new();
        let mut to_be_removed = HashSet::new();

        match self.get_mut_right(&right_key) {
            Some(left_set) => {
                // Check for value to remove
                for left_value in left_set.iter() {
                    if !left_values.contains(&**left_value) {
                        // `Rc`, so cheap clone
                        to_be_removed.insert(left_value.clone());
                    }
                }

                // Check for value to add
                for left_value in left_values {
                    if !left_set.contains(&left_value) {
                        let left_rc = Rc::new(left_value);
                        to_add_as_value_of_left.insert(left_rc.clone());
                        left_set.insert(left_rc);
                    }
                }
            },
            None => {
                let rc_values: HashSet<_> = left_values.into_iter().map(Rc::new).collect();
                // Ok to clone since it's an `HashSet` of `Rc`
                let rc_values_cloned = rc_values.clone();
                self.right_map_rc.insert(right_key.clone(), rc_values);
                to_add_as_value_of_left.extend(rc_values_cloned);
            },
        }

        for left_key in to_add_as_value_of_left {
            self.left_map_rc
                .entry(left_key.clone())
                .and_modify(|right_set| {
                    right_set.insert(right_key.clone());
                })
                .or_insert_with(|| HashSet::from_iter([right_key.clone()]));
        }

        for left_key in to_be_removed {
            if let Some(left_set_mut) = self.get_mut_right(&right_key) {
                left_set_mut.remove(&*left_key);
            }

            self.left_map_rc
                .entry(left_key.clone())
                .and_modify(|right_set| {
                    right_set.remove(&right_key);
                });

            if self
                .left_map_rc
                .get(&left_key)
                .is_some_and(|right_set| right_set.is_empty())
            {
                self.left_map_rc.remove(&left_key);
            }
        }
    }

    pub fn right_values(&self) -> Keys<Rc<RightType>, HashSet<Rc<LeftType>>> {
        self.right_map_rc.keys()
    }

    // pub fn right_entry(
    //     &mut self,
    //     right_entry: Rc<RightType>,
    // ) -> std::collections::hash_map::Entry<'_, std::rc::Rc<RightType>, HashSet<std::rc::Rc<LeftType>>>
    // {
    //     self.right_map_rc.entry(right_entry)
    // }
}
