/// A red-black tree implementation. The tree stores only unique keys.

use super::red_black_node::*;

use std::{
    borrow::Borrow,
    fmt
};

#[derive(Debug, PartialEq, Eq)]
pub struct RedBlackTree<K, V>(Option<RBPointer<K, V>>);

impl<K, V> Default for RedBlackTree<K, V> {
    #[inline]
    fn default() -> Self {
        Self(None)
    }
}

/// Construction methods
impl<K, V> RedBlackTree<K, V> {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

/// Queries
impl<K, V> RedBlackTree<K, V> {
    /// Searches for the predecessor of the given value among the keys stored in the tree.
    /// Time complexity: O(log n).
    pub fn predecessor<T>(&self, value: &T) -> Option<&K>
    where
        K: Borrow<T>,
        T: Ord + ?Sized,
    {
        self.get_root().and_then(|root| root.predecessor(value))
    }

    /// Searches for the successor of the given value among the keys stored in the tree.
    /// Time complexity: O(log n).
    pub fn successor<T>(&self, value: &T) -> Option<&K>
    where
        K: Borrow<T>,
        T: Ord + ?Sized,
    {
        self.get_root().and_then(|root| root.successor(value))
    }

    /// Searches for the stored key that equals the given value.
    /// Time complexity: O(log n).
    pub fn get<T>(&self, value: &T) -> Option<&K>
    where
        K: Borrow<T>,
        T: Ord + ?Sized,
    {
        self.get_root().and_then(|root| root.get(value))
    }
}

/// Tree access
impl<K, V> RedBlackTree<K, V> {
    fn get_root(&self) -> Option<&RBNode<K, V>> {
        self.0.as_ref().map(|root| root.as_ref())
    }

    fn get_root_mut(&mut self) -> Option<&mut RBNode<K, V>> {
        self.0.as_mut().map(|root| root.as_mut())
    }
}

impl<K: Ord, V> RedBlackTree<K, V> {
    /// Inserts the key-data pair into the tree.
    /// If the key was not present in the tree yet, None is returned.
    /// Otherwise, the data stored at the given key is updated, and the old data is returned.
    /// Time complexity: O(log n).
    pub fn insert(&mut self, key: K, data: V) -> Option<V> {
        if let Some(tree) = self.get_root_mut() {
            tree.insert(key, data)
        } else {
            self.0 = Some(Box::new(RBNode::new(key, data)));
            None
        }
    }
}

impl<K: fmt::Display, V> fmt::Display for RedBlackTree<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.get_root() {
            Some(root) => root.fmt(f),
            None => write!(f, "└──L\n"),
        }
    }
}
