#![feature(coroutines)]
#![feature(gen_blocks)]
#![feature(impl_trait_in_assoc_type)]

use std::{hash::Hash, ops::Deref, borrow::Borrow};

#[cfg(feature = "hashbrown")]
use hashbrown::{HashMap, HashSet};

type Rc<T> = std::sync::Arc<T>;

#[derive(Default)]
pub struct BiMultiMap<'a, L: Hash + Eq, R: Hash + Eq> {
    left_map_rc: HashMap<Rc<L>, HashSet<Rc<R>>>,
    right_map_rc: HashMap<Rc<R>, HashSet<Rc<L>>>,
    left_map_ref: HashMap<&'a L, HashSet<&'a R>>,
    right_map_ref: HashMap<&'a R, HashSet<&'a L>>,
}

impl<'a, L, R> IntoIterator for  BiMultiMap<'a, L, R> 
    where
        L: Hash + Eq + Clone + 'a,
        R: Hash + Eq + Clone + 'a,
{
    type Item = (Rc<L>, Rc<R>);
    type IntoIter = impl Iterator<Item = (Rc<L>, Rc<R>)>;

    fn into_iter(self) -> Self::IntoIter {
        gen move {
            for (left, rights) in self.left_map_rc {
                for right in rights {
                    yield (
                        left.clone(), right
                    );
                }
            }
        }.into_iter()
    }
}

impl<'a, L: Hash + Eq, R: Hash + Eq> BiMultiMap<'a, L, R> {
    pub fn new() -> BiMultiMap<'a, L, R> {
        BiMultiMap {
            left_map_ref: HashMap::new(),
            right_map_ref: HashMap::new(),
            left_map_rc: HashMap::new(),
            right_map_rc: HashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&L, &R)> {
        gen {
            for (left, rights) in self.left_map_rc.iter() {
                for right in rights {
                    yield (&*left.borrow(), &*right.borrow());
                }
            }
        }.into_iter()
    }

    /// Inserts a (L, R) in the [BiMultiMap]
    pub fn insert<'l>(&'l mut self, left: L, right: R) {
        // let left_ref = self.left_values.get_or_insert(left);
        // // INSERT AND AFTER THAT GET REFERENCE OF SELF
        // // let uwu = if let Some(uwu) = self.right_values.get(&right) {
        // //     uwu
        // // } else {
        // //     unsafe { self.right_values.insert_unique_unchecked(right) }
        // //
        // // };
        // // let iter = &self.right_values.iter();
        // self.right_values.insert(right);
        // let right_keys = self.right_map.keys().collect::<HashSet<_>>();
        // if let Some(right_value_ref) = self.right_values.iter().find(|right_value| !right_keys.contains(right_value)) {
        //     drop(right_keys);
        //     self.right_map
        //         .entry(right_value_ref)
        //         .and_modify(|left_set| {
        //             left_set.insert(left_ref);
        //         })
        //     .or_insert_with(|| HashSet::from_iter([left_ref]));
        // }

        // let right_ref = self.right_values.difference(iter.collect());

        // if let Some(ok) = right_ref {
        //
        // }

        //     .and_modify(|left_set| {
        //         left_set.insert(left_ref);
        //     })
        //     .or_insert_with(|| HashSet::from_iter([left_ref]));

        // self.left_map
        //     .entry(left_ref)
        //     .and_modify(|right_set| {
        //         right_set.insert(right_ref);
        //     })
        //     .or_insert_with(|| HashSet::from_iter([right_ref]));
    }

    pub fn get_left(&self, left: &L) -> Option<&HashSet<&R>> {
        self.left_map_ref.get(
            left
        )
    }

    // pub fn get_right(&self, right: &R) -> Option<&HashSet<&L>> {
    //     self.right_map.get(right)
    // }

    pub fn get_one_left(&self, left: &L) -> Option<&R> {
        self.get_left(left).and_then(|e| e.iter().next()).map(Deref::deref)
        // self.get_left(left).and_then(|rights| rights.iter().next()).map(|v| *v)
    }
    // pub fn get_one_right(&self, right: &R) -> Option<&L> {
    //     self.get_right(right).and_then(|lefts| lefts.iter().next()).map(|v| *v)
    // }

    // fn get_mut_left(&mut self, left: &L) -> Option<&mut HashSet<&'a R>> {
    //     // self.left_map.get_mut(left)
    // }
    // fn get_mut_right(&mut self, right: &R) -> Option<&mut HashSet<&'a L>> {
    //     self.right_map.get_mut(right)
    // }

    pub fn get_left_vec(&self, left: &L) -> Option<Vec<&R>> {
        // self.left_map.get(
        //     &self.hash_one(left)
        // ).map(|map| map.iter().map(|c| self.right_by_hash_unchecked(*c)).collect())
        todo!()
    }
    // pub fn get_right_vec(&self, right: &R) -> Option<Vec<&L>> {
    //     self.get_right(right)
    //         .map(|set| set.iter().map(Deref::deref).collect::<Vec<_>>())
    // }

    /// Remove an existing mapping between Left and Right.
    ///
    /// Returns whether the mapping was removed.
    pub fn remove<'x, 'y>(&mut self, left: &'x L, right: &'y R) -> bool
    where 'x: 'a, 'y: 'a{
        // let can_be_removed = (self.get_left(&left).is_some_and(|right_set| right_set.contains(right))
        // ) && (self.get_right(&right) .is_some_and(|left_set| left_set.contains(left)));
        //
        // if can_be_removed {
        //     let should_remove_left = self.get_mut_left(&left).map(|right_set| {
        //         right_set.remove(&right);
        //         right_set.is_empty()
        //     });
        //
        //     if should_remove_left == Some(true) {
        //         self.left_map.remove(&left);
        //         self.left_values.remove(left);
        //     }
        //
        //     let should_remove_right = self.get_mut_right(&right).map(|left_set| {
        //         left_set.remove(&left);
        //         left_set.is_empty()
        //     });
        //
        //     if should_remove_right == Some(true) {
        //         self.right_map.remove(&right);
        //         self.right_values.remove(right);
        //     }
        //
        //     true
        // } else {
        //     false
        // }
        true
    }

    pub fn remove_left(&mut self, left: &'a L) -> Option<HashSet<&'a R>> {
        // match self.left_map.remove(left) {
        //     Some(right_set) => {
        //         right_set.iter().for_each(|right| {
        //             let is_empty = self.get_mut_right(right).map(|hashet_left| {
        //                 hashet_left.remove(&left);
        //
        //                 hashet_left.is_empty()
        //             });
        //
        //
        //             if is_empty.is_some_and(|b| b) {
        //                 self.right_map.remove(right);
        //                 self.right_values.remove(*right);
        //             }
        //         });
        //         
        //         self.left_values.remove(left);
        //
        //         Some(right_set)
        //     }
        //     None => None,
        // }
        None
    }
}
