use arrayvec::ArrayVec;
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct ArrayPriorityQueue<P: Ord + Debug, const N: usize> {
    /// A vector containing the indices of elements in the priority queue.
    heap: ArrayVec<usize, N>,
    /// A vector that maps indices to their corresponding priority values.
    map: ArrayVec<Option<(usize, P)>, N>,
}

impl<P: Ord + Debug, const N: usize> Default for ArrayPriorityQueue<P, N> {
    /// Creates a new, empty, fixed-size priority queue based on the provided capacity in the type
    /// annotation.
    fn default() -> Self {
        Self {
            heap: ArrayVec::new(),
            map: ArrayVec::new(), // will be grown with None via init_map_none
        }
    }
}

impl<P: Ord + Debug, const N: usize> ArrayPriorityQueue<P, N> {
    /// Create a new, empty, fixed-size priority queue based on the provided capacity in the type
    /// annotation.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Ensure the map has at least `required_len` entries, filling with None.
    pub fn init_map_none(&mut self, required_len: usize) {
        debug_assert!(
            required_len <= N,
            "required_len {required_len} exceeds fixed capacity {N}",
        );
        while self.map.len() < required_len {
            self.map.push(None);
        }
    }

    /// Fill the priority queue from an iterator.
    pub fn fill_from_iter<T: IntoIterator<Item = (usize, P)>>(&mut self, iter: T) {
        for index_priority_pair in iter {
            self.insert(index_priority_pair);
        }
    }

    /// Fill the priority queue from an iterator without checking if the map is large enough.
    pub fn fill_from_iter_unsafe<T: IntoIterator<Item = (usize, P)>>(&mut self, iter: T) {
        for index_priority_pair in iter {
            self.insert_unsafe(index_priority_pair);
        }
    }

    /// Get the index of the parent of the item at the given index
    #[inline]
    fn get_parent_index(i: usize) -> usize {
        (i - 1) / 2
    }

    /// Get the index of the left child of the item at the given index
    #[inline]
    fn get_left_child_index(i: usize) -> usize {
        2 * i + 1
    }

    /// Get the index of the right child of the item at the given index
    #[inline]
    fn get_right_child_index(i: usize) -> usize {
        2 * i + 2
    }

    /// Swap the items at the given heap indexes
    fn swap(&mut self, heap_index_a: usize, heap_index_b: usize) {
        // Get the indexes of the cells that need to be updated
        let map_index_a = self.heap[heap_index_a];
        let map_index_b = self.heap[heap_index_b];
        // Swap the cell index positions in the heap
        self.heap.swap(heap_index_a, heap_index_b);
        // Swap the heap indexes in the map
        let (_, priority_a) = self.map[map_index_a].take().unwrap();
        let (_, priority_b) = self.map[map_index_b].take().unwrap();
        self.map[map_index_a] = Some((heap_index_b, priority_a));
        self.map[map_index_b] = Some((heap_index_a, priority_b));
    }

    /// Get the priority of the item at the given index without checking if the key could be out of
    /// bounds. This function is unsafe because it assumes that the map is large enough to contain
    /// the index.
    pub fn get_priority_unsafe(&self, key: usize) -> Option<&P> {
        self.map[key].as_ref().map(|(_, p)| p)
    }

    /// Get the priority of the key at the given index
    pub fn get_priority(&self, key: usize) -> Option<&P> {
        if key < self.map.len() {
            self.get_priority_unsafe(key)
        } else {
            None
        }
    }

    /// Get the map index of the item at the given heap index.
    /// This function is unsafe because it assumes that the heap index is within bounds.
    fn get_map_index_unsafe(&self, heap_index: usize) -> usize {
        self.heap[heap_index]
    }

    /// Get the priority of the item at the given heap index
    fn get_priority_heap_index_unsafe(&self, heap_index: usize) -> Option<&P> {
        self.get_priority_unsafe(self.get_map_index_unsafe(heap_index))
    }

    /// Try to move the item at the given heap index up the heap until it is in the correct
    /// position.
    fn heapify_up(&mut self, heap_index: usize) {
        if heap_index == 0 {
            return;
        }
        let mut current_heap_index = heap_index;
        let mut parent_heap_index = Self::get_parent_index(current_heap_index);
        while current_heap_index > 0
            && self
                .get_priority_heap_index_unsafe(current_heap_index)
                .gt(&self.get_priority_heap_index_unsafe(parent_heap_index))
        {
            self.swap(current_heap_index, parent_heap_index);
            current_heap_index = parent_heap_index;
            if parent_heap_index > 0 {
                parent_heap_index = Self::get_parent_index(current_heap_index);
            }
        }
    }

    /// Try to move the item at the given heap index down the heap until it is in the correct
    /// position.
    fn heapify_down(&mut self, heap_index: usize) {
        let mut current_heap_index = heap_index;
        let mut left_child_heap_index = Self::get_left_child_index(current_heap_index);
        let mut right_child_heap_index = Self::get_right_child_index(current_heap_index);
        while left_child_heap_index < self.heap.len() {
            let largest_child_index = if right_child_heap_index < self.heap.len()
                && self
                    .get_priority_heap_index_unsafe(right_child_heap_index)
                    .gt(&self.get_priority_heap_index_unsafe(left_child_heap_index))
            {
                right_child_heap_index
            } else {
                left_child_heap_index
            };
            if self
                .get_priority_heap_index_unsafe(current_heap_index)
                .ge(&self.get_priority_heap_index_unsafe(largest_child_index))
            {
                break;
            }
            self.swap(current_heap_index, largest_child_index);
            current_heap_index = largest_child_index;
            left_child_heap_index = Self::get_left_child_index(current_heap_index);
            right_child_heap_index = Self::get_right_child_index(current_heap_index);
        }
    }

    /// Insert an item into the priority queue without checking if the map is large enough.
    /// This function is unsafe because it assumes that the map is large enough to contain the
    /// index.
    pub fn insert_unsafe(&mut self, (map_index, new_priority): (usize, P)) {
        debug_assert!(
            map_index < N,
            "map_index {map_index} exceeds fixed capacity {N}",
        );
        if map_index >= self.map.len() {
            self.init_map_none(map_index + 1);
        }
        let slot = &mut self.map[map_index];
        if let Some((heap_index, old_priority)) = slot.take() {
            *slot = Some((heap_index, new_priority));
            match self
                .get_priority_heap_index_unsafe(heap_index)
                .unwrap()
                .cmp(&old_priority)
            {
                Ordering::Greater => self.heapify_up(heap_index),
                Ordering::Less => self.heapify_down(heap_index),
                Ordering::Equal => { /* no-op: priority unchanged */ }
            }
        } else {
            let heap_index = self.heap.len();
            self.heap.push(map_index);
            *slot = Some((heap_index, new_priority));
            self.heapify_up(heap_index);
        }
    }

    /// Remove the item with the highest priority from the priority queue
    pub fn pop(&mut self) -> Option<(usize, P)> {
        if self.heap.is_empty() {
            return None;
        }
        let last_index = self.heap.len() - 1;
        if last_index > 0 {
            self.swap(0, last_index);
        }
        let index = self.heap.pop().unwrap();
        let (.., priority) = self.map[index].take().unwrap();
        if !self.heap.is_empty() {
            self.heapify_down(0);
        }
        Some((index, priority))
    }

    /// Insert an item into the priority queue
    pub fn insert(&mut self, index_priority_pair: (usize, P)) {
        // Extend the map if needed
        let required_len = index_priority_pair.0 + 1;
        self.init_map_none(required_len);
        self.insert_unsafe(index_priority_pair);
    }

    /// Get the number of items in the priority queue
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Check if the priority queue is empty
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Peek at the item with the highest priority
    pub fn peek(&self) -> Option<(usize, &P)> {
        if self.is_empty() {
            return None;
        }
        let map_index = self.heap[0];
        let priority = &self.map[map_index].as_ref().unwrap().1;
        Some((map_index, priority))
    }

    /// Delete an item from the priority queue
    pub fn delete(&mut self, map_index: usize) {
        if let Some((heap_index, _)) = self.map[map_index].take() {
            // Remove last element from heap
            let maybe_last = self.heap.pop();
            if let Some(last_map_index) = maybe_last {
                if last_map_index != map_index {
                    // Move last element into the freed slot
                    self.heap[heap_index] = last_map_index;

                    // Update moved element's map entry to its new heap index
                    let slot = &mut self.map[last_map_index];
                    if let Some((_, p)) = slot.take() {
                        *slot = Some((heap_index, p));
                    }

                    // Restore heap property (up or down depending on relation to parent)
                    if heap_index > 0 {
                        let parent = Self::get_parent_index(heap_index);
                        if self
                            .get_priority_heap_index_unsafe(heap_index)
                            .gt(&self.get_priority_heap_index_unsafe(parent))
                        {
                            self.heapify_up(heap_index);
                            return;
                        }
                    }
                    self.heapify_down(heap_index);
                }
            }
        }
    }

    /// Create a new `PriorityQueue` from an iterator of key-priority pairs.
    /// This function is unsafe because it assumes that the iterator will not contain keys larger
    /// than the size hint from the iterator.
    pub fn from_iter_unsafe<I: Iterator<Item = (usize, P)>>(iter: I) -> Self {
        let mut pq = Self::default();
        pq.init_map_none(N);
        pq.fill_from_iter_unsafe(iter);
        pq
    }
}

impl<P, const N: usize> FromIterator<(usize, P)> for ArrayPriorityQueue<P, N>
where
    P: Ord + Debug,
{
    /// Create a new `PriorityQueue` from an iterator of key-priority pairs
    fn from_iter<I: IntoIterator<Item = (usize, P)>>(iter: I) -> Self
    where
        I::IntoIter: Iterator,
    {
        let mut pq = Self::default();
        pq.fill_from_iter(iter);
        pq
    }
}

impl<P, I, const N: usize> From<I> for ArrayPriorityQueue<P, N>
where
    P: Ord + Debug,
    I: Iterator<Item = (usize, P)>,
{
    fn from(iter: I) -> Self {
        iter.collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let pq: ArrayPriorityQueue<i32, 10> = ArrayPriorityQueue::default();
        assert!(pq.is_empty());
        assert_eq!(pq.heap.len(), 0);
        assert_eq!(pq.map.len(), 0);
    }

    #[test]
    fn test_with_capacity() {
        let pq: ArrayPriorityQueue<i32, 5> = ArrayPriorityQueue::default();
        assert!(pq.is_empty());
        assert_eq!(pq.heap.len(), 0);
        assert_eq!(pq.map.len(), 0);
    }

    #[test]
    fn test_insert_and_pop() {
        let mut pq: ArrayPriorityQueue<i32, 10> = ArrayPriorityQueue::default();
        pq.insert((1, 10));
        pq.insert((2, 5));
        pq.insert((3, 20));
        assert_eq!(pq.pop(), Some((3, 20)));
        assert_eq!(pq.pop(), Some((1, 10)));
        assert_eq!(pq.pop(), Some((2, 5)));
        assert!(pq.is_empty());
    }

    #[test]
    fn test_heapify_up() {
        let mut pq: ArrayPriorityQueue<i32, 10> = ArrayPriorityQueue::default();
        pq.insert((2, 20));
        pq.insert((1, 10));
        pq.insert((3, 5));
        assert_eq!(pq.pop(), Some((2, 20)));
        assert_eq!(pq.pop(), Some((1, 10)));
        assert_eq!(pq.pop(), Some((3, 5)));
    }

    #[test]
    fn test_heapify_down() {
        let mut pq: ArrayPriorityQueue<i32, 10> = ArrayPriorityQueue::default();
        pq.insert((1, 10));
        pq.insert((2, 5));
        pq.insert((3, 20));
        pq.pop();
        pq.insert((4, 25));
        assert_eq!(pq.pop(), Some((4, 25)));
        assert_eq!(pq.pop(), Some((1, 10)));
        assert_eq!(pq.pop(), Some((2, 5)));
    }

    #[test]
    fn test_peek() {
        let mut pq: ArrayPriorityQueue<i32, 10> = ArrayPriorityQueue::default();
        pq.insert((1, 10));
        pq.insert((2, 5));
        assert_eq!(pq.peek(), Some((1, &10)));
    }

    #[test]
    fn test_len() {
        let mut pq: ArrayPriorityQueue<i32, 10> = ArrayPriorityQueue::default();
        assert_eq!(pq.len(), 0);
        pq.insert((1, 10));
        assert_eq!(pq.len(), 1);
        pq.insert((2, 5));
        assert_eq!(pq.len(), 2);
        pq.pop();
        assert_eq!(pq.len(), 1);
    }

    #[test]
    fn test_from_iter() {
        let items = vec![(3, 20), (1, 10), (2, 5)];
        let mut pq: ArrayPriorityQueue<i32, 10> = ArrayPriorityQueue::from(items.into_iter());
        assert_eq!(pq.pop(), Some((3, 20)));
        assert_eq!(pq.pop(), Some((1, 10)));
        assert_eq!(pq.pop(), Some((2, 5)));
    }
}
