/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn adjust_from_bottom(&mut self) {
        // We get a new insertion at the rear
        let mut idx = self.count - 1;
        let mut parent_idx = self.parent_idx(idx);
        while idx != 0 && (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
            self.items.swap(idx, parent_idx);
            idx = parent_idx;
            parent_idx = self.parent_idx(idx);
        }
    }

    fn adjust_from_root(&mut self) {
        // the head is removed and the previous rear element has replaced the head
        let mut idx = 0;
        while self.children_present(idx) {
            let smallest_idx = self.smallest_child_idx(idx);
            if smallest_idx == idx {
                return;
            }
            self.items.swap(idx, smallest_idx);
            idx = smallest_idx;
        }
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        self.adjust_from_bottom();
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0{
            return None;
        }
        if self.count == 1 {
            // quick path
            self.count = 0;
            return self.items.pop();
        }
        // normal path
        let rear = self.count - 1;
        self.items.swap(0, rear);
        let val = self.items.pop();
        self.count -= 1;
        self.adjust_from_root();
        val
    }

    fn parent_idx(&self, idx: usize) -> usize {
        if idx == 0 {
            0
        } else {
            (idx - 1)/ 2
        }
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) < self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if !self.children_present(idx) {
            return idx;
        }
		let me = &self.items[idx];
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        let left;
        let right;
        if right_idx >= self.count {
            // Right child not exist
            left = &self.items[left_idx];
            if (self.comparator)(me, left) {
                return idx;
            } else {
                return left_idx;
            }
        } else {
            // both left and right child exit
            left = &self.items[left_idx];
            right = &self.items[right_idx];
            if (self.comparator)(me, left) && (self.comparator)(me, right) {
                return idx;
            } else if (self.comparator)(left, right) {
                return left_idx;
            } else {
                return right_idx;
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		self.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}