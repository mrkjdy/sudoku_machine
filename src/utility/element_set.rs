use super::bitset::{BitSet16, BitSet16Iter};

use std::cmp::Ordering;

/// A wrapper around a `BitSet16`, where the `Eq`, `PartialEq`, `PartialOrd`, and `Ord` traits have
/// been implemented based on the cardinality (length) of the set. This is useful for use in when
/// you need to compare sets based on their size alone, such as in a MRV (Minimum Remaining Values)
/// priority queue.
#[derive(Copy, Clone, Debug, Default)]
pub struct ElementSet(BitSet16);

impl PartialEq for ElementSet {
    /// Checks if two element sets are equal by comparing their cardinality (length).
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
    }
}

impl Eq for ElementSet {}

impl PartialOrd for ElementSet {
    /// Compares two element sets by their cardinality (length).
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ElementSet {
    /// Compares two element sets by their cardinality (length).
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.len().cmp(&self.0.len())
    }
}

impl ElementSet {
    /// Creates a new `ElementSet` containing all numbers from 1 to 9.
    pub const CLASSIC: Self = Self(BitSet16(0b1_1111_1111));

    /// Removes a number from the set.
    pub fn remove(&mut self, num: u8) {
        self.0.remove(num);
    }

    /// Inserts a number into the set.
    pub fn insert(&mut self, num: u8) {
        self.0.insert(num);
    }

    /// Returns the intersection of two element sets. (the set of numbers that are in both sets)
    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        Self(self.0.intersection(&other.0))
    }

    /// Returns an iterator over the numbers in the set.
    #[must_use]
    pub fn iter(&self) -> BitSet16Iter<'_> {
        self.0.iter()
    }

    /// Returns true if the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the length of the set.
    #[must_use]
    pub fn len(&self) -> u8 {
        self.0.len()
    }

    /// Returns true if the set contains the given number.
    #[must_use]
    pub fn has(&self, num: u8) -> bool {
        self.0.has(num)
    }

    /// Removes and returns the smallest number in the set.
    pub fn pop(&mut self) -> Option<u8> {
        self.0.pop()
    }
}

impl<I> From<I> for ElementSet
where
    I: Iterator<Item = u8>,
{
    /// Creates a new `ElementSet` from an iterator of numbers.
    fn from(iter: I) -> Self {
        Self(BitSet16::from(iter))
    }
}

impl<'a> IntoIterator for &'a ElementSet {
    type Item = u8;
    type IntoIter = BitSet16Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
