use std::cmp::Ordering;

use super::MinPQ;

#[derive(Debug)]
pub struct UnorderedMinPQ<T> {
    vec: Vec<T>,
}

impl<T: PartialOrd> MinPQ<T> for UnorderedMinPQ<T> {
    fn new() -> Self {
        UnorderedMinPQ { vec: vec![] }
    }

    fn insert(&mut self, key: T) {
        self.vec.push(key);
    }

    fn min(&self) -> &T {
        self.vec
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap()
    }

    fn delete_min(&mut self) -> T {
        let min_index = self
            .vec
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(index, item)| index)
            .unwrap();

        self.vec.remove(min_index)
    }

    fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    fn size(&self) -> usize {
        self.vec.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unordered_min_pq() {
        let mut pq = UnorderedMinPQ::new();

        pq.insert(6);
        pq.insert(2);
        pq.insert(4);
        pq.insert(4);
        pq.insert(3);
        pq.insert(9);

        assert_eq!(pq.delete_min(), 2);
        assert_eq!(pq.delete_min(), 3);
        assert_eq!(pq.delete_min(), 4);

        pq.insert(1);
        pq.insert(1);
        assert_eq!(pq.delete_min(), 1);
        assert_eq!(pq.delete_min(), 1);
        assert_eq!(pq.delete_min(), 4);
        assert_eq!(pq.delete_min(), 6);
        assert_eq!(pq.delete_min(), 9);
    }
}
