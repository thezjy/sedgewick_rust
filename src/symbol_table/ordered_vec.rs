use crate::binary_search::binary_search;

use super::{OrderedSymbolTable, SymbolTable};

#[derive(Clone)]
pub struct OrderedVecST<K: Ord, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K: Ord, V> OrderedVecST<K, V> {
    pub fn keys(&self) -> std::slice::Iter<K> {
        self.keys.iter()
    }

    pub fn new() -> Self {
        OrderedVecST {
            keys: vec![],
            values: vec![],
        }
    }
}

impl<K, V> SymbolTable<K, V> for OrderedVecST<K, V>
where
    K: Ord,
{
    fn put(&mut self, key: K, value: V) {
        match binary_search(&self.keys, &key) {
            Ok(i) => {
                self.values[i] = value;
            }
            Err(i) => {
                self.keys.insert(i, key);
                self.values.insert(i, value);
            }
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        match binary_search(&self.keys, &key) {
            Ok(i) => Some(&self.values[i]),
            Err(_) => None,
        }
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match binary_search(&self.keys, &key) {
            Ok(i) => Some(&mut self.values[i]),
            Err(_) => None,
        }
    }

    fn delete(&mut self, key: K) {
        if let Ok(i) = binary_search(&self.keys, &key) {
            self.keys.remove(i);
            self.values.remove(i);
        }
    }

    fn contains(&self, key: &K) -> bool {
        binary_search(&self.keys, &key).is_ok()
    }

    fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    fn size(&self) -> usize {
        self.keys.len()
    }

    fn keys(&self) -> Box<dyn Iterator<Item = &K> + '_> {
        Box::new(self.keys.iter())
    }
}
