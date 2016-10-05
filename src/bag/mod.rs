use std::collections::hash_map::{self as hm, HashMap};
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::hash::Hash;
use std::ops::AddAssign;
use std::fmt;
use std::iter::{Iterator, IntoIterator, FromIterator};

#[derive(PartialEq, Clone)]
/// An Unordered MultiSet.
/// We can view `Bag` as `Unigram`, a special case of the `n-gram`, with n=1.
pub struct Bag<E>(HashMap<E, usize>) where E: Eq + Hash;

impl<E> fmt::Debug for Bag<E>
    where E: Eq + Hash + fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bag {:?}", self.0)
    }
}

#[macro_export]
macro_rules! bagof {
    () => { $crate::Bag::new() };
    ( $( $item: expr ),* ) => {
        {
            let mut bag = $crate::Bag::new();
            $( bag.insert($item); )*
            bag
        }
    };
}

#[macro_export]
macro_rules! bigram {
    () => { $crate::Bag::new() };
    ( $( $item: expr ),* ) => {
        {
            let mut bag = $crate::Bag::new();
            let vec = vec![$( $item ),*];
            for w in vec.windows(2) {
                bag.insert((w[0], w[1]));
            }
            bag
        }
    };
}

impl<E: Eq + Hash> Bag<E> {
    /// Creates a new empty `Bag`.
    pub fn new() -> Self {
        Bag(HashMap::new())
    }

    /// Counts all the elements, including each duplicate.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use(bagof)] extern crate bag; fn main() {
    /// let ms = bagof!(1,1,2);
    /// assert_eq!(3, ms.len());
    /// let ms = bagof!(1,1,2,2);
    /// assert_eq!(4, ms.len());
    /// # }
    /// ```
    pub fn len(&self) -> usize {
        self.0.values().fold(0, |a, &b| a + b)
    }

    /// Counts the occurrences of `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use(bagof)] extern crate bag; fn main() {
    /// let bag = bagof!(0,0,1,0,1);
    /// assert_eq!(3, bag.occurrence(0));
    /// assert_eq!(2, bag.occurrence(1));
    /// assert_eq!(0, bag.occurrence(2));
    /// # }
    /// ```
    pub fn occurrence(&self, elem: E) -> usize {
        self.0.get(&elem).map_or(0, |&x| x)
    }

    /// Insert an element.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use(bagof)] extern crate bag; fn main() {
    /// let mut bag = bagof!();
    /// assert_eq!(0, bag.occurrence(1));
    /// bag.insert(1);
    /// assert_eq!(1, bag.occurrence(1));
    /// # }
    /// ```
    pub fn insert(&mut self, elem: E) {
        match self.0.entry(elem) {
            Vacant(view) => {
                view.insert(1);
            }
            Occupied(mut view) => {
                view.get_mut().add_assign(1);
            }
        }
    }
}

pub struct Frequency<'a, E>
    where E: 'a + Eq + Hash
{
    it: hm::Iter<'a, E, usize>,
}

pub struct Distinct<'a, E>
    where E: 'a + Eq + Hash
{
    it: hm::Keys<'a, E, usize>,
}

impl<'a, E> Bag<E>
    where E: Eq + Hash
{
    pub fn iter(&'a self) -> Frequency<'a, E> {
        self.frequency()
    }
    pub fn frequency(&'a self) -> Frequency<'a, E> {
        Frequency { it: self.0.iter() }
    }
    pub fn distinct(&'a self) -> Distinct<'a, E> {
        Distinct { it: self.0.keys() }
    }
}

macro_rules! iterator {
    ( struct $name:ident -> $item: ty ) => {
        impl<'a, E> Iterator for $name<'a, E>
            where E: Eq + Hash
        {
            type Item = $item;
            fn next(&mut self) -> Option<Self::Item> {
                self.it.next()
            }
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.it.size_hint()
            }
        }
    }
}

iterator!( struct Frequency -> (&'a E, &'a usize) );
iterator!( struct Distinct  ->  &'a E );

impl<E> IntoIterator for Bag<E>
    where E: Eq + Hash
{
    type Item = (E, usize);
    type IntoIter = hm::IntoIter<E, usize>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<E> FromIterator<E> for Bag<E>
    where E: Eq + Hash
{
    fn from_iter<I: IntoIterator<Item = E>>(it: I) -> Self {
        let mut bag = Bag::new();
        for e in it.into_iter() {
            bag.insert(e);
        }
        bag
    }
}

#[cfg(test)]
mod tests {
    macro_rules! check_bagof {
        () => {};
        ( $( $item:expr),* ) => {
            {
                let bag = bagof!( $( $item ),* );
                for (&e, &c) in bag.iter() {
                    assert!(bag.occurrence(e) > 0);
                    assert!(bag.occurrence(e) == c);
                }
                let bag = bigram!( $( $item ),* );
                println!("{:?}", bag);
            }
        }
    }

    #[test]
    fn bagof_chars() {
        check_bagof!();
        check_bagof!(1,2,1,2,3,3,0,3,4);
        check_bagof!('a','b','r','a','c','a','d','a','b','r','a');
        check_bagof!("I","am","18","years","old",".");
    }
}
