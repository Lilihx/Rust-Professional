/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;
use std::fmt::Debug;

#[derive(Debug)]
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
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count = self.count + 1;
        self.items.push(value);
        let mut child_idx = self.len();
        loop {
            if child_idx == 1 {
                break;
            }
            let parent_idx = self.parent_idx(child_idx);
            if (self.comparator)(&self.items[child_idx], &self.items[parent_idx]) {
                self.items.swap(child_idx, parent_idx);
                child_idx = parent_idx;
            } else {
                break;
            }
        }
    }
    // 2 4 9 11
    // 0 4 11 9
    pub fn peek(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        self.items.swap(1, self.count);
        let mut parent_idx = 1;
        self.count = self.count - 1;
        loop {
            if !self.children_present(parent_idx) {
                break;
            }
            let child_idx = self.smallest_child_idx(parent_idx);
            if (self.comparator)(&self.items[child_idx], &self.items[parent_idx]) {
                self.items.swap(parent_idx, child_idx);
                parent_idx = child_idx;
            } else {
                break;
            }
        }
        Some(self.items.remove(self.count + 1))
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_child_idx = self.left_child_idx(idx);
        let right_child_idx = self.right_child_idx(idx);
        if right_child_idx >= self.count {
            left_child_idx
        } else if (self.comparator)(&self.items[left_child_idx], &self.items[right_child_idx]) {
            left_child_idx
        } else {
            right_child_idx
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
        self.peek()
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
        println!("{:?}", heap);
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
