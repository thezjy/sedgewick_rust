// use super::SymbolTable;

// #[derive(Debug)]
// pub struct BST<K, V> {
//     root: Tree<K, V>,
// }

// type Tree<K, V> = Option<Box<Node<K, V>>>;

// #[derive(Debug)]
// pub struct Node<K, V> {
//     left: Tree<K, V>,
//     right: Tree<K, V>,
//     value: V,
//     key: K,
//     nodes_count: usize,
// }

// impl<K, V> Node<K, V> {
//     fn new(key: K, value: V, nodes_count: usize) -> Self {
//         Node {
//             key,
//             value,
//             nodes_count,
//             left: None,
//             right: None,
//         }
//     }
// }

// impl<K: Ord, V> SymbolTable<K, V> for BST<K, V> {
//     fn put(&mut self, key: K, value: V) {
//         self.root = Self::tree_put(self.root.take(), key, value);
//     }

//     fn get(&self, key: &K) -> Option<&V> {
//         BST::tree_get(key, &self.root)
//     }

//     fn get_mut(&mut self, key: &K) -> Option<&mut V> {
//         BST::tree_get_mut(key, &mut self.root)
//     }

//     fn delete(&mut self, key: K) {
//         todo!()
//     }

//     fn contains(&self, key: &K) -> bool {
//         self.get(&key).is_some()
//     }

//     fn is_empty(&self) -> bool {
//         self.root.is_none()
//     }

//     fn size(&self) -> usize {
//         BST::tree_size(&self.root)
//     }
// }

// impl<K: Ord, V> BST<K, V> {
//     pub fn new() -> Self {
//         BST { root: None }
//     }

//     pub fn max(&self) -> Option<&K> {
//         let mut tree = &self.root;

//         match tree {
//             Some(node) => loop {
//                 if node.right.is_none() {
//                     return Some(&node.key);
//                 } else {
//                     tree = &node.right;
//                 }
//             },
//             None => None,
//         }
//     }

//     fn tree_size(tree: &Tree<K, V>) -> usize {
//         match tree {
//             Some(node) => node.nodes_count,
//             None => 0,
//         }
//     }

//     fn tree_get<'a>(key: &K, tree: &'a Tree<K, V>) -> Option<&'a V> {
//         match tree {
//             Some(node) => {
//                 if *key < node.key {
//                     Self::tree_get(key, &node.left)
//                 } else if *key > node.key {
//                     Self::tree_get(key, &node.right)
//                 } else {
//                     Some(&node.value)
//                 }
//             }
//             None => None,
//         }
//     }

//     fn tree_get_mut<'a>(key: &K, tree: &'a mut Tree<K, V>) -> Option<&'a mut V> {
//         match tree {
//             Some(node) => {
//                 if *key < node.key {
//                     Self::tree_get_mut(key, &mut node.left)
//                 } else if *key > node.key {
//                     Self::tree_get_mut(key, &mut node.right)
//                 } else {
//                     Some(&mut node.value)
//                 }
//             }
//             None => None,
//         }
//     }

//     fn tree_put(mut tree: Tree<K, V>, key: K, value: V) -> Tree<K, V> {
//         match tree.take() {
//             Some(mut node) => {
//                 if key < node.key {
//                     node.left = Self::tree_put(node.left, key, value);
//                 } else if key > node.key {
//                     node.right = Self::tree_put(node.right, key, value);
//                 } else {
//                     node.value = value;
//                 }

//                 node.nodes_count = Self::tree_size(&node.left) + Self::tree_size(&node.right) + 1;

//                 tree
//             }
//             None => Some(Box::new(Node::new(key, value, 1))),
//         }
//     }
// }
