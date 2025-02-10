use std::fmt::Debug;

#[derive(Clone, Debug, Default)]
pub struct PriorityQueue<P: Ord + Debug> {
    heap: Vec<usize>,
    map: Vec<Option<(usize, P)>>,
}

impl<P: Ord + Debug> PriorityQueue<P> {
    /// Create a new empty PriorityQueue
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: Vec::with_capacity(capacity),
            map: Vec::with_capacity(capacity),
        }
    }

    /// Initialize the map with None values up to the required length.
    pub fn init_map_none(&mut self, required_len: usize) {
        let current_len = self.map.len();
        self.map.extend((current_len..required_len).map(|_| None));
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
    fn get_parent_index(i: usize) -> usize {
        (i - 1) / 2
    }

    /// Get the index of the left child of the item at the given index
    fn get_left_child_index(i: usize) -> usize {
        2 * i + 1
    }

    /// Get the index of the right child of the item at the given index
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
        if let Some((heap_index, old_priority)) = self.map[map_index].take() {
            let is_gt_old = new_priority.gt(&old_priority);
            self.map[map_index] = Some((heap_index, new_priority));
            if is_gt_old {
                self.heapify_up(heap_index);
            } else {
                self.heapify_down(heap_index);
            }
        } else {
            let heap_index = self.heap.len();
            self.heap.push(map_index);
            self.map[map_index] = Some((heap_index, new_priority));
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
        let (_, priority) = self.map[index].take().unwrap();
        self.heapify_down(0);
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
            // Pop the last value off of the heap
            if let Some(last_index) = self.heap.pop() {
                if last_index != map_index {
                    self.heap[heap_index] = last_index;
                    self.heapify_down(heap_index);
                }
            }
        }
    }

    /// Create a new PriorityQueue from an iterator of key-priority pairs.
    /// This function is unsafe because it assumes that the iterator will not contain keys larger
    /// than the size hint from the iterator.
    pub fn from_iter_unsafe<I: Iterator<Item = (usize, P)>>(iter: I) -> Self {
        let (lower, upper_opt) = iter.size_hint();
        let capacity = if let Some(upper) = upper_opt {
            upper
        } else {
            lower
        };
        let mut priority_queue = PriorityQueue::with_capacity(capacity);
        priority_queue.init_map_none(capacity);
        priority_queue.fill_from_iter_unsafe(iter);
        priority_queue
    }
}

impl<P> FromIterator<(usize, P)> for PriorityQueue<P>
where
    P: Ord + Debug,
{
    /// Create a new PriorityQueue from an iterator of key-priority pairs
    fn from_iter<I: IntoIterator<Item = (usize, P)>>(iter: I) -> Self
    where
        I::IntoIter: Iterator,
    {
        let iter = iter.into_iter();
        let (lower, upper_opt) = iter.size_hint();
        let capacity = if let Some(upper) = upper_opt {
            upper
        } else {
            lower
        };
        let mut priority_queue = PriorityQueue::with_capacity(capacity);
        // Have to use the checked fill_from_iter because the iterator may contain keys larger than
        // the size hint.
        priority_queue.fill_from_iter(iter);
        priority_queue
    }
}

impl<P, I> From<I> for PriorityQueue<P>
where
    P: Ord + Debug,
    I: Iterator<Item = (usize, P)>,
{
    fn from(iter: I) -> Self {
        PriorityQueue::from_iter(iter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let pq: PriorityQueue<i32> = PriorityQueue::default();
        assert!(pq.is_empty());
        assert_eq!(pq.heap.len(), 0);
        assert_eq!(pq.map.len(), 0);
    }

    #[test]
    fn test_with_capacity() {
        let pq: PriorityQueue<i32> = PriorityQueue::with_capacity(5);
        assert!(pq.is_empty());
        assert_eq!(pq.heap.len(), 0);
        assert_eq!(pq.map.len(), 0);
    }

    #[test]
    fn test_insert_and_pop() {
        let mut pq = PriorityQueue::default();
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
        let mut pq = PriorityQueue::default();
        pq.insert((2, 20));
        pq.insert((1, 10));
        pq.insert((3, 5));
        assert_eq!(pq.pop(), Some((2, 20)));
        assert_eq!(pq.pop(), Some((1, 10)));
        assert_eq!(pq.pop(), Some((3, 5)));
    }

    #[test]
    fn test_heapify_down() {
        let mut pq = PriorityQueue::default();
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
        let mut pq = PriorityQueue::default();
        pq.insert((1, 10));
        pq.insert((2, 5));
        assert_eq!(pq.peek(), Some((1, &10)));
    }

    #[test]
    fn test_len() {
        let mut pq = PriorityQueue::default();
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
        let mut pq: PriorityQueue<i32> = PriorityQueue::from(items.into_iter());
        assert_eq!(pq.pop(), Some((3, 20)));
        assert_eq!(pq.pop(), Some((1, 10)));
        assert_eq!(pq.pop(), Some((2, 5)));
    }
}
