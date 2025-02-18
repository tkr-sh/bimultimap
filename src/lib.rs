#![feature(coroutines)]
#![feature(gen_blocks)]
#![feature(impl_trait_in_assoc_type)]

mod left;
mod right;
#[cfg(feature = "serde")]
mod serde;

use std::{borrow::Borrow, hash::Hash};

#[cfg(feature = "hashmap")]
pub use std::collections::{HashMap, HashSet, hash_map::Entry};
#[cfg(feature = "hashbrown")]
pub use hashbrown::{HashMap, HashSet, hash_map::Entry};

/// The reference counting type returned by the map
#[cfg(feature = "thread-safe")]
pub type Rc<T> = std::sync::Arc<T>;
#[cfg(not(feature = "thread-safe"))]
pub type Rc<T> = std::rc::Rc<T>;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct BiMultiMap<L: Hash + Eq, R: Hash + Eq> {
    left_map_rc:  HashMap<Rc<L>, HashSet<Rc<R>>>,
    right_map_rc: HashMap<Rc<R>, HashSet<Rc<L>>>,
}

impl<L, R> IntoIterator for BiMultiMap<L, R>
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
                    yield (left.clone(), right);
                }
            }
        }
    }
}

impl<L, R> FromIterator<(L, R)> for BiMultiMap<L, R>
where
    L: Hash + Eq,
    R: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = (L, R)>>(iter: T) -> Self {
        let mut map = BiMultiMap::new();
        iter.into_iter()
            .for_each(|(left, right)| map.insert(left, right));
        map
    }
}

impl<L: Hash + Eq, R: Hash + Eq> BiMultiMap<L, R> {
    pub fn new() -> Self {
        BiMultiMap {
            left_map_rc:  HashMap::new(),
            right_map_rc: HashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Rc<L>, &Rc<R>)> {
        gen {
            for (left, rights) in self.left_map_rc.iter() {
                for right in rights {
                    yield (left, right);
                }
            }
        }
    }

    pub fn iter_ref(&self) -> impl Iterator<Item = (&L, &R)> {
        gen {
            for (left, rights) in self.left_map_rc.iter() {
                for right in rights {
                    yield (left.borrow(), right.borrow());
                }
            }
        }
    }

    /// Inserts a (L, R) in the [BiMultiMap]
    pub fn insert(&mut self, left: L, right: R) {
        let left_rc = Rc::new(left);
        let right_rc = Rc::new(right);

        self.right_map_rc
            .entry(right_rc.clone())
            .and_modify(|left_set| {
                left_set.insert(left_rc.clone());
            })
            .or_insert_with(|| HashSet::from_iter([left_rc.clone()]));

        self.left_map_rc
            .entry(left_rc.clone())
            .and_modify(|right_set| {
                right_set.insert(right_rc.clone());
            })
            .or_insert_with(|| HashSet::from_iter([right_rc.clone()]));
    }

    /// Remove an existing mapping between Left and Right.
    ///
    /// Returns whether the mapping was removed.
    pub fn remove<LeftRef: Borrow<L>, RightRef: Borrow<R>>(
        &mut self,
        left: LeftRef,
        right: RightRef,
    ) -> bool {
        let left = left.borrow();
        let right = right.borrow();

        let can_be_removed = self
            .get_left(left)
            .is_some_and(|right_set| right_set.contains(right)) &&
            self.get_right(right)
                .is_some_and(|left_set| left_set.contains(left));

        if can_be_removed {
            let should_remove_left = self.get_mut_left(left).map(|right_set| {
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
}

