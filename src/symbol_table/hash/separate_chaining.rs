use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::symbol_table::{ordered_vec::OrderedVecST, SymbolTable};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub struct SeparateChainingHashST<K: Ord, V> {
    st: Vec<OrderedVecST<K, V>>,
}

struct Node<K, V> {
    key: K,
    val: V,
}

impl<K: Ord + Hash, V> SeparateChainingHashST<K, V> {
    const SIZE: usize = 30011;

    pub fn new() -> Self {
        let mut st = Vec::with_capacity(Self::SIZE);
        (0..Self::SIZE).for_each(|_| st.push(OrderedVecST::new()));

        SeparateChainingHashST { st }
    }

    fn hash(key: &K) -> usize {
        (calculate_hash(key) % Self::SIZE as u64) as usize
    }
}

impl<K: Ord + Hash, V> SymbolTable<K, V> for SeparateChainingHashST<K, V> {
    fn put(&mut self, key: K, value: V) {
        self.st[Self::hash(&key)].put(key, value);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.st[Self::hash(&key)].get(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.st[Self::hash(&key)].get_mut(key)
    }

    fn delete(&mut self, key: K) {
        self.st[Self::hash(&key)].delete(key)
    }

    fn contains(&self, key: &K) -> bool {
        self.st[Self::hash(&key)].contains(key)
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn keys(&self) -> Box<dyn Iterator<Item = &K> + '_> {
        Box::new(self.st.iter().flat_map(|st| st.keys()))
    }
}
