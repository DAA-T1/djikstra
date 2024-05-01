//! Memory safe minimum priority queue implementations.
//!
use std::{collections::HashMap, hash::Hash};

/// Non-performant and easy min priority queue implementation.
///
/// Uses a HashMap under the hood with a generic key and value of type ```usize```.
/// Key contains the associated weight of its corresponding element in the priority queue.
/// A key cannot have a value below 0.
pub struct PriorityQueue<T>
where
    T: Ord,
{
    pub map: HashMap<T, usize>,
}

impl<T> Default for PriorityQueue<T>
where
    T: Ord + Hash + Clone,
{
    fn default() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl<T> PriorityQueue<T>
where
    T: Ord + Hash + Clone,
{
    /// Create a new PriorityQueue with no elements.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new PriorityQueue from an iterator of elements without keys.
    /// Keys will be set to usize::MAX as default
    pub fn from_keys<I>(input: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            map: HashMap::from_iter(input.into_iter().map(|item| (item, usize::MAX))),
        }
    }

    /// Create a new PriorityQueue from a iterator of tuples containing elements and their respective keys.
    ///
    /// Example input:
    /// ```
    /// let valid_vec_w_keys = vec![(-1, 1), (3, 3), (2, 2), (4, 4)];
    /// ```
    pub fn from_keys_values<I>(input: I) -> Self
    where
        I: IntoIterator<Item = (T, usize)>,
    {
        Self {
            map: HashMap::from_iter(input),
        }
    }
    /// Insert a new element with its key into the priority queue.
    pub fn insert(&mut self, element: T, key: usize) {
        self.map.insert(element, key);
    }

    /// Change the key for an element in the priority queue.
    pub fn change_key(&mut self, element: &T, key: usize) {
        if let Some(obj) = self.map.get_mut(element) {
            *obj = key;
        }
    }

    /// Extract the element with the smallest key from the queue.
    /// Returns the element and its associated key as a tuple.
    pub fn extract_min(&mut self) -> Option<(T, usize)> {
        // the below code has to be implemented because of
        // internal reference in min_by and read from here
        // https://github.com/rust-lang/rust/issues/27724#issuecomment-161772708
        let mut min_value: Option<usize> = None;
        let mut min_key: Option<T> = None;

        for (key, &value) in self.map.iter() {
            if let Some(m_value) = min_value {
                if m_value > value {
                    min_value = Some(value);
                    min_key = Some(key.clone());
                }
            } else {
                min_value = Some(value);
                min_key = Some(key.clone());
            }
        }

        if let Some(min_key) = min_key {
            self.map.remove_entry(&min_key)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pq::PriorityQueue;

    #[test]
    fn removes_minimum() {
        let numbers = vec![(-1, 1), (3, 3), (2, 2), (4, 4)];
        let mut pq = PriorityQueue::from_keys_values(numbers);
        assert_eq!(pq.extract_min(), Some((-1, 1)));
        assert_eq!(pq.extract_min(), Some((2, 2)))
    }

    #[test]
    fn changes_key() {
        let numbers = vec![(0, 0), (1, usize::MAX), (2, usize::MAX), (3, usize::MAX)];
        let mut pq = PriorityQueue::from_keys_values(numbers);
        // check for key increase
        pq.change_key(&1, 4);
        pq.change_key(&2, 1);
        pq.extract_min();
        assert_eq!(pq.extract_min(), Some((2, 1)));

        // check for key decrease
    }

    #[test]
    fn returns_none_when_empty() {
        let mut pq: PriorityQueue<usize> = PriorityQueue::new();
        let min = pq.extract_min();
        assert_eq!(min, None)
    }
}
