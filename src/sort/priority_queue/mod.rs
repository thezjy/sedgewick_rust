mod heap;
mod unordered;

pub use heap::*;
pub use unordered::*;

use std::cmp::Ordering;

pub trait MaxPQ<T: PartialOrd> {
    fn new() -> Self;

    fn insert(key: T);

    fn max(&self) -> &T;

    fn delete_max() -> T;

    fn is_empty() -> bool;

    fn size() -> usize;
}

pub trait MinPQ<T: PartialOrd> {
    fn new() -> Self;

    fn insert(&mut self, key: T);

    fn min(&self) -> &T;

    fn delete_min(&mut self) -> T;

    fn is_empty(&self) -> bool;

    fn size(&self) -> usize;
}
