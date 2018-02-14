#![feature(conservative_impl_trait)]

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::hash::Hash;

/// An Unordered `MultiSet`.
#[derive(Debug, Clone, PartialEq)]
pub struct Bag<T: Eq + Hash>(HashMap<T, usize>);

impl<T: Eq + Hash> Default for Bag<T> {
    fn default() -> Self {
        Bag(HashMap::new())
    }
}

#[macro_export]
macro_rules! bagof {
    () => { $crate::Bag::new() };
    ( $( $item: expr ),* ) => {
        {
            let mut bag = $crate::Bag::new();
            $( bag.put($item); )*
            bag
        }
    };
}

impl<T: Eq + Hash> Bag<T> {
    /// Creates a new empty `Bag`.
    pub fn new() -> Self {
        Bag(HashMap::new())
    }

    /// Counts all the elements, including each duplicate.
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use(bagof)]
    /// extern crate bag;
    /// fn main() {
    ///     let bag = bagof!(1, 1, 2);
    ///     assert_eq!(3, bag.len());
    ///     let bag = bagof!(1, 1, 2, 2);
    ///     assert_eq!(4, bag.len());
    /// }
    /// ```
    pub fn len(&self) -> usize {
        self.0.values().sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Insert an element.
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use(bagof)]
    /// extern crate bag;
    /// fn main() {
    ///     let mut bag = bagof!();
    ///     assert_eq!(0, bag.occurrence(&1));
    ///     bag.put(1);
    ///     assert_eq!(1, bag.occurrence(&1));
    /// }
    /// ```
    pub fn put(&mut self, elem: T) {
        match self.0.entry(elem) {
            Vacant(view) => {
                view.insert(1);
            }
            Occupied(mut view) => {
                *view.get_mut() += 1;
            }
        }
    }

    /// Counts the occurrences of `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use(bagof)]
    /// extern crate bag;
    /// fn main() {
    ///     let bag = bagof!(0,0,1,0,1);
    ///     assert_eq!(3, bag.occurrence(&0));
    ///     assert_eq!(2, bag.occurrence(&1));
    ///     assert_eq!(0, bag.occurrence(&2));
    /// }
    /// ```
    pub fn occurrence(&self, elem: &T) -> usize {
        self.0.get(elem).map_or(0, |&x| x)
    }

    pub fn frequency<'a>(&'a self) -> impl Iterator<Item = (&T, &usize)> + 'a {
        self.0.iter()
    }

    pub fn distinct<'a>(&'a self) -> impl Iterator<Item = &T> + 'a {
        self.0.keys()
    }
}

#[cfg(test)]
mod tests {
    macro_rules! check_bagof {
        ( $( $item:expr),* ) => {
            {
                let bag = bagof!( $( $item ),* );
                for (&e, &c) in bag.frequency() {
                    assert!(bag.occurrence(&e) > 0);
                    assert!(bag.occurrence(&e) == c);
                }
            }
        }
    }

    #[test]
    fn bagof_chars() {
        check_bagof!(1, 2, 1, 2, 3, 3, 0, 3, 4);
        check_bagof!('a', 'b', 'r', 'a', 'c', 'a', 'd', 'a', 'b', 'r', 'a');
        check_bagof!("I", "am", "18", "years", "old", ".");
    }
}
