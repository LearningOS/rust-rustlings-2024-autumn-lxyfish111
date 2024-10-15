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
    T: Default + std::fmt::Debug,
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
        self.items.push(value);
        self.count += 1;
        self.sift_up(self.count);
        // println!("items:{:?}", self.items);
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
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        if right <= self.count && ((self.comparator)(&self.items[right], &self.items[left])) {
            right
        } else {
            left
        }
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 1 && ((self.comparator)(&self.items[idx], &self.items[self.parent_idx(idx)])) {
            let idx1 = self.parent_idx(idx);
            self.items.swap(idx, idx1);
            idx = self.parent_idx(idx);
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        // println!("arr item1:{:?}", self.items);
        while self.children_present(idx) {
            let smaller_child = self.smallest_child_idx(idx);
            // println!("smaller:{}", smaller_child.clone());
            if (self.comparator)(&self.items[smaller_child], &self.items[idx]) {
                // println!("1");
                self.items.swap(idx, smaller_child);
                idx = smaller_child;
                // println!("arr item2:{:?}", self.items);
            } else {
                // println!("2");
                break;
            }
        }
    } 
}

impl<T> Heap<T>
where
    T: Default + Ord + std::fmt::Debug,
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
    T: Default + Clone + std::fmt::Debug + std::fmt::Display,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // println!("item next:{:?}", self.items);
        if self.is_empty() {
            None
        } else {
            let item = self.items[1].clone();
            self.items[1] = self.items.remove(self.count);
            self.count -= 1;
            self.sift_down(1);
            // println!("item value:{}", item.clone());
            Some(item)
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::fmt::Debug,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::fmt::Debug,
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