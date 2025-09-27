/// A fixed-size set of bits, stored in a `u16` with a maximum length of 16.
///
/// This struct is useful for efficiently representing a set of numbers between
/// 1 and 16, inclusive. Each bit in the `u16` represents the presence or absence
/// of a number in the set.
#[derive(Copy, Clone, Debug, Default)]
pub struct BitSet16(pub(super) u16);

impl<I> From<I> for BitSet16
where
    I: Iterator<Item = u8>,
{
    /// Creates a new `BitSet16` from a range of numbers.
    ///
    /// This function will set the bits corresponding to each number in the range.
    ///
    /// # Arguments
    ///
    /// * `range` - A range of numbers to insert into the bitset.
    ///
    fn from(iter: I) -> Self {
        Self(iter.fold(0, |acc, num| {
            acc | BitSet16::mask(BitSet16::num_to_bit(num))
        }))
    }
}

impl BitSet16 {
    /// Converts a number to the corresponding bit position in the bitset.
    ///
    /// # Arguments
    ///
    /// * `num` - A number to convert to a bit position. This should be in the range 1..=16.
    ///
    fn num_to_bit(num: u8) -> u8 {
        num - 1
    }

    fn bit_to_num(bit: u8) -> u8 {
        bit + 1
    }

    /// Creates a bitmask for a given bit position.
    ///
    /// # Arguments
    ///
    /// * `bit` - The position for which to create a bitmask.
    ///
    fn mask(bit: u8) -> u16 {
        1 << bit
    }

    /// Returns the number of elements in the bitset.
    ///
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn len(&self) -> u8 {
        // There are only up to 16 1s in a u16, so no truncation can occur here.
        self.0.count_ones() as u8
    }

    /// Returns an iterator over the numbers in the bitset.
    ///
    #[must_use]
    pub fn iter(&self) -> BitSet16Iter<'_> {
        BitSet16Iter { set: self, bit: 0 }
    }

    /// Gets the value of the bit at the specified position.
    ///
    /// # Arguments
    ///
    /// * `bit` - The position of the bit to get.
    ///
    fn get_bit(self, bit: u8) -> u16 {
        self.0 & Self::mask(bit)
    }

    /// Determines if the bit at the specified position is one.
    ///
    /// # Arguments
    ///
    /// * `bit` - The position of the bit to check.
    ///
    fn bit_is_one(self, bit: u8) -> bool {
        self.get_bit(bit) != 0
    }

    /// Checks if the bitset contains a specific number.
    ///
    /// # Arguments
    ///
    /// * `num` - The number to check for presence in the bitset. Should be in the range 1..=16.
    ///
    #[must_use]
    pub fn has(&self, num: u8) -> bool {
        self.bit_is_one(Self::num_to_bit(num))
    }

    /// Sets the bit at the specified position to one.
    ///
    /// # Arguments
    ///
    /// * `bit` - The position of the bit to be set.
    ///
    fn set_bit(&mut self, bit: u8) {
        self.0 |= Self::mask(bit);
    }

    /// Inserts a number into the bitset, setting the corresponding bit to one.
    ///
    /// If the bit was previously zero, the length of the bitset is increased.
    ///
    /// # Arguments
    ///
    /// * `num` - The number to insert into the bitset. Should be in the range 1..=16.
    ///
    pub fn insert(&mut self, num: u8) {
        self.set_bit(Self::num_to_bit(num));
    }

    /// Clears the bit at the specified position, setting it to zero.
    ///
    /// # Arguments
    ///
    /// * `bit` - The position of the bit to clear.
    ///
    fn clear_bit(&mut self, bit: u8) {
        self.0 &= !Self::mask(bit);
    }

    /// Removes a number from the bitset, setting the corresponding bit to zero.
    ///
    /// If the bit was previously one, the length of the bitset is decreased.
    ///
    /// # Arguments
    ///
    /// * `num` - The number to remove from the bitset. Should be in the range 1..=16.
    ///
    pub fn remove(&mut self, num: u8) {
        self.clear_bit(Self::num_to_bit(num));
    }

    /// Computes the intersection of two `BitSet16` instances, returning a new bitset.
    ///
    /// The intersection is a new bitset containing only the numbers present in both
    /// `self` and `other`.
    ///
    /// # Arguments
    ///
    /// * `other` - The other `BitSet16` to intersect with.
    ///
    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.is_empty() {
            return None;
        }
        let bit = self.0.trailing_zeros() as u8;
        self.clear_bit(bit);
        Some(Self::bit_to_num(bit))
    }
}

pub struct BitSet16Iter<'a> {
    set: &'a BitSet16,
    bit: u8,
}

impl Iterator for BitSet16Iter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.bit < 16 {
            let bit_is_one = self.set.bit_is_one(self.bit);
            self.bit += 1;
            if bit_is_one {
                return Some(self.bit);
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.set.len() as usize;
        (length, Some(length))
    }
}

impl<'a> IntoIterator for &'a BitSet16 {
    type Item = u8;
    type IntoIter = BitSet16Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let bitset = BitSet16::default();
        assert_eq!(bitset.0, 0);
    }

    #[test]
    fn test_from_range() {
        let bitset = BitSet16::from(1..4);
        assert!(bitset.has(1));
        assert!(bitset.has(2));
        assert!(bitset.has(3));
        assert!(!bitset.has(4));
    }

    #[test]
    fn test_len() {
        let bitset = BitSet16::from(1..4);
        assert_eq!(bitset.len(), 3);
    }

    #[test]
    fn test_iter() {
        let bitset = BitSet16::from(1..4);
        let elements: Vec<u8> = bitset.iter().collect();
        assert_eq!(elements, vec![1, 2, 3]);
    }

    #[test]
    fn test_get_bit() {
        let bitset = BitSet16::from(1..2);
        assert_eq!(bitset.get_bit(0), 1);
        assert_eq!(bitset.get_bit(1), 0);
    }

    #[test]
    fn test_bit_is_one() {
        let bitset = BitSet16::from(1..2);
        assert!(bitset.bit_is_one(0));
        assert!(!bitset.bit_is_one(1));
    }

    #[test]
    fn test_has() {
        let bitset = BitSet16::from(1..2);
        assert!(bitset.has(1));
        assert!(!bitset.has(2));
    }

    #[test]
    fn test_insert() {
        let mut bitset = BitSet16::default();
        bitset.insert(1);
        assert!(bitset.has(1));
    }

    #[test]
    fn test_remove() {
        let mut bitset = BitSet16::from(1..2);
        bitset.remove(1);
        assert!(!bitset.has(1));
    }

    #[test]
    fn test_intersection() {
        let bitset1 = BitSet16::from(1..3);
        let bitset2 = BitSet16::from(2..4);
        let intersection = bitset1.intersection(&bitset2);
        assert!(!intersection.has(1));
        assert!(intersection.has(2));
        assert!(!intersection.has(3));
    }

    #[test]
    fn test_pop() {
        let mut bitset = BitSet16::default();
        assert_eq!(bitset.pop(), None);
        bitset.insert(2);
        bitset.insert(3);
        bitset.insert(1);
        assert_eq!(bitset.pop(), Some(1));
        assert_eq!(bitset.pop(), Some(2));
        assert_eq!(bitset.pop(), Some(3));
        assert_eq!(bitset.pop(), None);
    }
}
