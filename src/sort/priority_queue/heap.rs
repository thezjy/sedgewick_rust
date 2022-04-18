use super::MinPQ;

#[derive(Debug)]
pub struct HeapMinPQ<T> {
    vec: Vec<T>,
}

impl<T: PartialOrd> HeapMinPQ<T> {
    /// because we don't use index 0 for easier calculation of parent/child index
    fn max_index(&self) -> usize {
        self.vec.len() - 1
    }

    pub fn swim(&mut self) {
        let mut current_index = self.max_index();

        loop {
            let parent_index = current_index / 2;

            if parent_index < 1 || self.vec[current_index] >= self.vec[parent_index] {
                break;
            }

            self.vec.swap(parent_index, current_index);
            current_index = parent_index;
        }
    }

    pub fn sink(&mut self) {
        let mut current_index = 1;
        let max_index = self.max_index();

        loop {
            let left_child_index = current_index * 2;

            if left_child_index > max_index {
                break;
            }

            let right_child_index = left_child_index + 1;

            let child_index: usize = if right_child_index > max_index
                || self.vec[left_child_index] < self.vec[right_child_index]
            {
                left_child_index
            } else {
                right_child_index
            };

            if self.vec[current_index] <= self.vec[child_index] {
                break;
            }

            self.vec.swap(current_index, child_index);
            current_index = child_index;
        }
    }
}

impl<T: PartialOrd + Default> MinPQ<T> for HeapMinPQ<T> {
    /// we do not use index 0
    fn new() -> Self {
        HeapMinPQ {
            vec: vec![T::default()],
        }
    }

    fn insert(&mut self, key: T) {
        self.vec.push(key);
        self.swim();
    }

    fn min(&self) -> &T {
        &self.vec[1]
    }

    fn delete_min(&mut self) -> T {
        let min = self.vec.swap_remove(1);

        self.sink();

        min
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn size(&self) -> usize {
        self.vec.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_min_pq() {
        let mut min_pq = HeapMinPQ::new();

        assert!(min_pq.is_empty());

        min_pq.insert(4);
        min_pq.insert(3);
        min_pq.insert(3);
        min_pq.insert(9);
        min_pq.insert(2);
        min_pq.insert(3);

        assert_eq!(min_pq.delete_min(), 2);
        assert_eq!(min_pq.size(), 5);

        min_pq.insert(1);
        assert_eq!(min_pq.delete_min(), 1);
        assert_eq!(min_pq.delete_min(), 3);
        assert_eq!(min_pq.delete_min(), 3);
        assert_eq!(min_pq.delete_min(), 3);
    }
}
