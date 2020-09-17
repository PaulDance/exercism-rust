//! Basically a HashSet re-implementation. There are a few untested functionalities that were
//! added for completion's sake.

use std::collections::{hash_map::DefaultHasher, VecDeque};
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

/// A collection behaving like a set with faster access using hashing.
#[derive(Debug, Clone)]
pub struct CustomSet<T: Hash> {
    /// The map storing the set values.
    ///
    /// Its indices are the result of hashing its elements modulo its length. `VecDeque` is used in
    /// order to have a faster re-hashing operation.
    map: Vec<VecDeque<T>>,
    /// The number of elements currently stored in the set.
    len: usize,
    /// The maximum number of elements the set can receive before extending and re-hashing.
    capacity: usize,
}

impl<T: Hash + PartialEq + Clone> CustomSet<T> {
    /// The minimum and default capacity of a set if not specified.
    const DEFAULT_CAPACITY: usize = 5;
    /// The amount by which the capacity of a set grows when filled up.
    const ADDED_CAPACITY: usize = 10;
    /// The ratio of the map size to the collection capacity.
    const SIZE_CAP_RATIO: f64 = 0.3;

    /// Builds a new set by cloning the elements from the given `input` slice.
    pub fn new(input: &[T]) -> Self {
        let mut set = Self::with_capacity(input.len());
        set.extend(input.iter().map(|x| x.clone()));
        set
    }

    /// Builds a new empty set with the default capacity.
    pub fn new_empty() -> Self {
        Self::with_capacity(Self::DEFAULT_CAPACITY)
    }

    /// Builds a new empty set with the given `capacity`.
    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = capacity.max(Self::DEFAULT_CAPACITY);
        let map_size = (capacity as f64 * Self::SIZE_CAP_RATIO).round() as usize;
        let mut map = Vec::with_capacity(map_size);
        map.resize(map_size, VecDeque::new());

        Self {
            map,
            capacity,
            len: 0,
        }
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` iff the set contains no element.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Adds an element to the set if does not contain it already.
    pub fn add(&mut self, element: T) {
        // Extend and re-hash if capacity is reached.
        if self.len == self.capacity {
            let add_map_size =
                (Self::ADDED_CAPACITY as f64 * Self::SIZE_CAP_RATIO).round() as usize;
            self.map.extend((0..add_map_size).map(|_| VecDeque::new()));
            self.capacity += add_map_size;
            self.rehash();
        }

        let index = self.index_of(&element);

        // Push if not already stored.
        if !self.map[index].contains(&element) {
            self.map[index].push_back(element);
            self.len += 1;
        }
    }

    /// Removes the given element from the set if it is present, does nothing otherwise.
    pub fn remove(&mut self, element: &T) {
        let i = self.index_of(element);

        if let Some(j) = self.map[i].iter().position(|x| x == element) {
            self.map[i].swap_remove_back(j);
        }
    }

    /// Returns `true` iff the given `element` is present in the set.
    pub fn contains(&self, element: &T) -> bool {
        self.map[self.index_of(element)]
            .iter()
            .any(|x| x == element)
    }

    /// Returns `true` iff all the elements of the set are contained in `other`.
    pub fn is_subset(&self, other: &Self) -> bool {
        self.iter().all(|x| other.contains(x))
    }

    /// Returns `true` iff the set and `other` contain no element in common.
    pub fn is_disjoint(&self, other: &Self) -> bool {
        !(self.iter().any(|x| other.contains(x)) || other.iter().any(|x| self.contains(x)))
    }

    /// Returns a new set by cloning all the elements contained in both `self` and `other`.
    pub fn intersection(&self, other: &Self) -> Self {
        self.iter()
            .filter(|x| other.contains(x))
            .map(|x| x.clone())
            .collect()
    }

    /// Returns a new set by cloning all the elements contained in `self` but not in `other`.
    pub fn difference(&self, other: &Self) -> Self {
        self.iter()
            .filter(|x| !other.contains(x))
            .map(|x| x.clone())
            .collect()
    }

    /// Returns a new set by cloning all the elements contained in either `self` or `other`.
    pub fn union(&self, other: &Self) -> Self {
        let mut set = self.clone();
        set.extend(other.iter().map(|x| x.clone()));
        set
    }

    /// Returns an iterator on the set's elements by reference.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.map.iter().flat_map(|dq| dq.iter())
    }

    /// Consumes the set and returns an iterator on the set's elements by value.
    pub fn into_iter(self) -> impl IntoIterator<Item = T> {
        self.map.into_iter().flat_map(|dq| dq.into_iter())
    }

    /// Re-hashes all the elements of the set in order to put them in their correct place.
    fn rehash(&mut self) {
        for i in 0..self.map.len() {
            for _ in 0..self.map[i].len() {
                if let Some(element) = self.map[i].pop_front() {
                    let index = self.index_of(&element);
                    self.map[index].push_back(element);
                }
            }
        }
    }

    /// Returns the index of the map to use in order to access the deque where is stored the given
    /// `element` by hashing it.
    fn index_of(&self, element: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);
        hasher.finish() as usize % self.map.len()
    }
}

impl<T: Hash + PartialEq + Clone> PartialEq for CustomSet<T> {
    /// Returns `true` iff both sets are subsets of each other.
    fn eq(&self, other: &Self) -> bool {
        self.is_subset(other) && other.is_subset(self)
    }
}

impl<T: Hash + PartialEq + Clone> FromIterator<T> for CustomSet<T> {
    /// Builds a new set by consuming all the elements of the given `iter`.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new_empty();
        set.extend(iter);
        set
    }
}

impl<T: Hash + PartialEq + Clone> Extend<T> for CustomSet<T> {
    /// Extends the set by consuming and adding to it all the elements of the given `iter`.
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.add(x);
        }
    }
}
