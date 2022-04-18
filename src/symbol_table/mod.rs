pub mod binary_search_tree;
pub mod hash;
pub mod ordered_vec;

pub trait SymbolTable<K, V> {
    fn put(&mut self, key: K, value: V);

    fn get(&self, key: &K) -> Option<&V>;

    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    fn delete(&mut self, key: K);

    fn contains(&self, key: &K) -> bool;

    fn is_empty(&self) -> bool;

    fn size(&self) -> usize;

    fn keys(&self) -> Box<dyn Iterator<Item = &K> + '_>;
}

pub trait OrderedSymbolTable<K: Ord, V>: SymbolTable<K, V> {
    fn min(&self) -> K;

    fn max(&self) -> K;

    fn floor(&self, key: K) -> K;

    fn ceil(&self, key: K) -> K;

    fn rank(&self, key: K) -> Result<usize, usize>;

    fn select(&self, rank: usize) -> K;

    fn delete_min(&mut self);

    fn delete_max(&mut self);

    fn size(&self, low: K, high: K) -> usize;
}
