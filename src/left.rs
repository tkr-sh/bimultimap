use {
    crate::{BiMultiMap, HashSet, Rc},
    std::{borrow::Borrow, hash::Hash, ops::Deref},
};

impl<LeftType: Hash + Eq, RightType: Hash + Eq> BiMultiMap<LeftType, RightType> {
    pub fn get_left(&self, left: &LeftType) -> Option<&HashSet<Rc<RightType>>> {
        self.left_map_rc.get(left)
    }

    pub fn get_one_left(&self, left: &LeftType) -> Option<&RightType> {
        self.get_left(left)
            .and_then(|e| e.iter().next())
            .map(Deref::deref)
    }

    pub(crate) fn get_mut_left(&mut self, left: &LeftType) -> Option<&mut HashSet<Rc<RightType>>> {
        self.left_map_rc.get_mut(left)
    }

    pub fn get_left_vec(&self, left: &LeftType) -> Option<Vec<&RightType>> {
        self.left_map_rc
            .get(left)
            .map(|map| map.iter().map(Deref::deref).collect())
    }

    pub fn remove_left<LeftRef: Borrow<LeftType>>(
        &mut self,
        left: LeftRef,
    ) -> Option<HashSet<Rc<RightType>>> {
        let left = left.borrow();
        match self.left_map_rc.remove(left) {
            Some(right_set) => {
                right_set.iter().for_each(|right| {
                    let is_empty = self.get_mut_right(right).map(|hashet_left| {
                        hashet_left.remove(left);

                        hashet_left.is_empty()
                    });

                    if is_empty.is_some_and(|b| b) {
                        self.right_map_rc.remove(right);
                    }
                });

                Some(right_set)
            },
            None => None,
        }
    }

    // Setter
    /// Set a right_value by the left key
    ///
    /// The argument passed is an [Rc] for the `left_key`, since it might be inserted as a value
    /// for all the right_values that are in `right_values` but not present in the [`BiMultiMap`]
    pub fn set_left(&mut self, left_key: Rc<LeftType>, right_values: HashSet<RightType>) {
        // These 2 declarations, demonstrate perfectly what this function does.
        //
        // We have on one side what Ill call `bmm_right_values` (BiMultiMapRightValues) and
        // `right_values`, the values passed in parameter.
        //
        // We therefore do 3 checks:
        // 1. If a value of `bmm_right_values` is not in `right_values` => We want to remove it from
        //   `bmm_right_values`. This is `to_be_removed`.
        // 2. If a value is both in `bmm_right_values` and `right_values` => Value is alreay present
        //    in both. No need to modify anything.
        //  3. If a value of `right_values` is not in `bmm_right_values` => We want to add it to
        //     `bmm_right_values`. This is `to_add_as_value_of_right`.
        let mut to_add_as_value_of_right = HashSet::new();
        let mut to_be_removed = HashSet::new();

        match self.get_mut_left(&left_key) {
            Some(right_set) => {
                // Check for value to remove
                for right_value in right_set.iter() {
                    if !right_values.contains(&**right_value) {
                        // `Rc`, so cheap clone
                        to_be_removed.insert(right_value.clone());
                    }
                }

                // Check for value to add
                for right_value in right_values {
                    if !right_set.contains(&right_value) {
                        let right_rc = Rc::new(right_value);
                        to_add_as_value_of_right.insert(right_rc.clone());
                        right_set.insert(right_rc);
                    }
                }
            },
            None => {
                let rc_values: HashSet<_> = right_values.into_iter().map(Rc::new).collect();
                // Ok to clone since it's an `HashSet` of `Rc`
                let rc_values_cloned = rc_values.clone();
                self.left_map_rc.insert(left_key.clone(), rc_values);
                to_add_as_value_of_right.extend(rc_values_cloned);
            },
        }

        for right_key in to_add_as_value_of_right {
            self.right_map_rc
                .entry(right_key.clone())
                .and_modify(|left_set| {
                    left_set.insert(left_key.clone());
                })
                .or_insert_with(|| HashSet::from_iter([left_key.clone()]));
        }

        for right_key in to_be_removed {
            if let Some(right_set_mut) = self.get_mut_left(&left_key) {
                right_set_mut.remove(&*right_key);
            }

            self.right_map_rc
                .entry(right_key.clone())
                .and_modify(|left_set| {
                    left_set.remove(&left_key);
                });

            if self
                .right_map_rc
                .get(&right_key)
                .is_some_and(|left_set| left_set.is_empty())
            {
                self.right_map_rc.remove(&right_key);
            }
        }
    }

    pub fn left_entry(
        &mut self,
        left_entry: Rc<LeftType>,
    ) -> std::collections::hash_map::Entry<'_, std::rc::Rc<LeftType>, HashSet<std::rc::Rc<RightType>>>
    {
        self.left_map_rc.entry(left_entry)
    }
}
