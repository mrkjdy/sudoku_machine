use super::bitset::{BitSet16, BitSet16Iter};

use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, Default)]
pub struct ElementSet(BitSet16);

impl PartialEq for ElementSet {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
    }
}

impl Eq for ElementSet {}

impl PartialOrd for ElementSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.0.len().cmp(&self.0.len()))
    }
}

impl Ord for ElementSet {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.len().cmp(&self.0.len())
    }
}

impl ElementSet {
    pub const CLASSIC: Self = Self(BitSet16(0b111111111));

    pub fn remove(&mut self, num: u8) {
        self.0.remove(num)
    }

    pub fn insert(&mut self, num: u8) {
        self.0.insert(num)
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self(self.0.intersection(&other.0))
    }

    pub fn iter(&self) -> BitSet16Iter {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> u8 {
        self.0.len()
    }

    pub fn has(&self, num: u8) -> bool {
        self.0.has(num)
    }

    pub fn pop(&mut self) -> Option<u8> {
        self.0.pop()
    }
}

impl<I> From<I> for ElementSet
where
    I: Iterator<Item = u8>,
{
    fn from(iter: I) -> Self {
        Self(BitSet16::from(iter))
    }
}
