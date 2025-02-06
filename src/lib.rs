#![feature(coroutines)]
#![feature(gen_blocks)]
#![feature(impl_trait_in_assoc_type)]

use std::{hash::Hash, ops::Deref};

#[cfg(feature = "hashbrown")]
use hashbrown::{HashMap, HashSet};

#[derive(Default)]
pub struct BiMultiMap<'a, L: Hash + Eq + 'a, R: Hash + Eq + 'a> {
    left_values: HashSet<L>,
    right_values: HashSet<R>,
    left_map: HashMap<&'a L, HashSet<&'a R>>,
    right_map: HashMap<&'a R, HashSet<&'a L>>,
}

impl<'a, L, R> IntoIterator for  BiMultiMap<'a, L, R> 
    where
        L: Hash + Eq + 'a + Clone,
        R: Hash + Eq + 'a + Clone,
{
    type Item = (L, R);
    type IntoIter = impl Iterator<Item = (L, R)>;

    fn into_iter(self) -> Self::IntoIter {
        gen {
            for (left, rights) in self.left_map {
                for right in rights {
                    yield (left.clone(), right.clone());
                }
            }
        }.into_iter()
    }
}

impl<'a, L: Hash + Eq + 'a, R: Hash + Eq + 'a> BiMultiMap<'a, L, R> {
    pub fn new() -> BiMultiMap<'a, L, R> {
        BiMultiMap {
            left_values: HashSet::new(),
            right_values: HashSet::new(),
            left_map: HashMap::new(),
            right_map: HashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'a L, &'a R)> {
        gen {
            for (left, rights) in self.left_map.iter() {
                for right in rights {
                    yield (*left, *right);
                }
            }
        }.into_iter()
    }

    /// Inserts a (L, R) in the [BiMultiMap]
    pub fn insert(&'a mut self, left: L, right: R) {
        let left_ref = self.left_values.get_or_insert(left);
        let right_ref = self.right_values.get_or_insert(right);

        self.right_map
            .entry(right_ref)
            .and_modify(|left_set| {
                left_set.insert(left_ref);
            })
            .or_insert_with(|| HashSet::from_iter([left_ref]));

        self.left_map
            .entry(left_ref)
            .and_modify(|right_set| {
                right_set.insert(right_ref);
            })
            .or_insert_with(|| HashSet::from_iter([right_ref]));
    }

    pub fn get_left(&self, left: &L) -> Option<&HashSet<&R>> {
        self.left_map.get(left)
    }
    pub fn get_right(&self, right: &R) -> Option<&HashSet<&L>> {
        self.right_map.get(right)
    }

    pub fn get_one_left(&self, left: &L) -> Option<&R> {
        self.get_left(left).and_then(|rights| rights.iter().next()).map(|v| *v)
    }
    pub fn get_one_right(&self, right: &R) -> Option<&L> {
        self.get_right(right).and_then(|lefts| lefts.iter().next()).map(|v| *v)
    }

    fn get_mut_left(&mut self, left: &L) -> Option<&mut HashSet<&'a R>> {
        self.left_map.get_mut(left)
    }
    fn get_mut_right(&mut self, right: &R) -> Option<&mut HashSet<&'a L>> {
        self.right_map.get_mut(right)
    }

    pub fn get_left_vec(&self, left: &L) -> Option<Vec<&R>> {
        self.get_left(left)
            .map(|set| set.iter().map(Deref::deref).collect::<Vec<_>>())
    }
    pub fn get_right_vec(&self, right: &R) -> Option<Vec<&L>> {
        self.get_right(right)
            .map(|set| set.iter().map(Deref::deref).collect::<Vec<_>>())
    }

    /// Remove an existing mapping between Left and Right.
    ///
    /// Returns whether the mapping was removed.
    pub fn remove<'x, 'y>(&mut self, left: &'x L, right: &'y R) -> bool
    where 'x: 'a, 'y: 'a{
        let can_be_removed = (self.get_left(&left).is_some_and(|right_set| right_set.contains(right))
        ) && (self.get_right(&right) .is_some_and(|left_set| left_set.contains(left)));

        if can_be_removed {
            let should_remove_left = self.get_mut_left(&left).map(|right_set| {
                right_set.remove(&right);
                right_set.is_empty()
            });

            if should_remove_left == Some(true) {
                self.left_map.remove(&left);
                self.left_values.remove(left);
            }

            let should_remove_right = self.get_mut_right(&right).map(|left_set| {
                left_set.remove(&left);
                left_set.is_empty()
            });

            if should_remove_right == Some(true) {
                self.right_map.remove(&right);
                self.right_values.remove(right);
            }

            true
        } else {
            false
        }
    }

    pub fn remove_left(&mut self, left: &'a L) -> Option<HashSet<&'a R>> {
        match self.left_map.remove(left) {
            Some(right_set) => {
                right_set.iter().for_each(|right| {
                    let is_empty = self.get_mut_right(right).map(|hashet_left| {
                        hashet_left.remove(&left);

                        hashet_left.is_empty()
                    });


                    if is_empty.is_some_and(|b| b) {
                        self.right_map.remove(right);
                        self.right_values.remove(*right);
                    }
                });
                
                self.left_values.remove(left);

                Some(right_set)
            }
            None => None,
        }
    }
}
