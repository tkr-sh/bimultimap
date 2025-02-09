#![feature(coroutines)]
#![feature(gen_blocks)]
#![feature(impl_trait_in_assoc_type)]

use std::{hash::Hash, ops::Deref, borrow::Borrow};

#[cfg(feature = "hashbrown")]
use hashbrown::{HashMap, HashSet};

/// The reference counting type returned by the map
#[cfg(feature = "thread-safe")]
pub type Rc<T> = std::sync::Arc<T>;
#[cfg(not(feature = "thread-safe"))]
pub type Rc<T> = std::rc::Rc<T>;

#[derive(Default)]
pub struct BiMultiMap<L: Hash + Eq, R: Hash + Eq> {
    left_map_rc: HashMap<Rc<L>, HashSet<Rc<R>>>,
    right_map_rc: HashMap<Rc<R>, HashSet<Rc<L>>>,
}

impl<L, R> IntoIterator for  BiMultiMap<L, R> 
    where
        L: Hash + Eq + Clone,
        R: Hash + Eq + Clone,
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

impl<L: Hash + Eq, R: Hash + Eq> BiMultiMap<L, R> {
    pub fn new() -> BiMultiMap<L, R> {
        BiMultiMap {
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
        let left_rc = Rc::new(left);
        let right_rc = Rc::new(right);

        self
            .right_map_rc
            .entry(right_rc.clone())
            .and_modify(|left_set| {
                left_set.insert(left_rc.clone());
            })
            .or_insert_with(|| HashSet::from_iter([left_rc.clone()]));

        self
            .left_map_rc
            .entry(left_rc.clone())
            .and_modify(|right_set| {
                right_set.insert(right_rc.clone());
            })
            .or_insert_with(|| HashSet::from_iter([right_rc.clone()]));
    }

    pub fn get_left(&self, left: &L) -> Option<&HashSet<Rc<R>>> {
        self.left_map_rc.get(left)
    }
    pub fn get_right(&self, right: &R) -> Option<&HashSet<Rc<L>>> {
        self.right_map_rc.get(right)
    }

    pub fn get_one_left(&self, left: &L) -> Option<&R> {
        self.get_left(left).and_then(|e| e.iter().next()).map(Deref::deref)
        // self.get_left(left).and_then(|rights| rights.iter().next()).map(|v| *v)
    }
    // pub fn get_one_right(&self, right: &R) -> Option<&L> {
    //     self.get_right(right).and_then(|lefts| lefts.iter().next()).map(|v| *v)
    // }

    fn get_mut_left(&mut self, left: &L) -> Option<&mut HashSet<Rc<R>>> {
        self.left_map_rc.get_mut(left)
    }
    fn get_mut_right(&mut self, right: &R) -> Option<&mut HashSet<Rc<L>>> {
        self.right_map_rc.get_mut(right)
    }

    pub fn get_left_vec(&self, left: &L) -> Option<Vec<&R>> {
        self.left_map_rc.get(left).map(|map| map.iter().map(Deref::deref).collect())
    }
    // pub fn get_right_vec(&self, right: &R) -> Option<Vec<&L>> {
    //     self.get_right(right)
    //         .map(|set| set.iter().map(Deref::deref).collect::<Vec<_>>())
    // }

    /// Remove an existing mapping between Left and Right.
    ///
    /// Returns whether the mapping was removed.
    pub fn remove<LeftRef: Borrow<L>, RightRef: Borrow<R>>(&mut self, left: LeftRef, right: RightRef) -> bool {
        let left = left.borrow();
        let right = right.borrow();

        let can_be_removed =
                self.get_left(&left).is_some_and(|right_set| right_set.contains(right)) &&
                self.get_right(right).is_some_and(|left_set| left_set.contains(left));

        if can_be_removed {
            let should_remove_left = self.get_mut_left(&left).map(|right_set| {
                right_set.remove(right);
                right_set.is_empty()
            });

            if should_remove_left == Some(true) {
                self.left_map_rc.remove(left);
            }

            let should_remove_right = self.get_mut_right(right).map(|left_set| {
                left_set.remove(left);
                left_set.is_empty()
            });

            if should_remove_right == Some(true) {
                self.right_map_rc.remove(right);
            }

            true
        } else {
            false
        }
    }

    pub fn remove_left(&mut self, left: &L) -> Option<HashSet<Rc<R>>> {
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
            }
            None => None,
        }
    }
}
