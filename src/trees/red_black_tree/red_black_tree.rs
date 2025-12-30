/// A red-black tree implementation. The tree stores only unique keys.

use super::red_black_node::*;

use std::{
    borrow::Borrow,
    fmt
};

#[derive(Debug, PartialEq, Eq)]
pub struct RedBlackTree<T>(Option<RBPointer<T>>);

impl<T> Default for RedBlackTree<T> {
    #[inline]
    fn default() -> Self {
        Self(None)
    }
}

/// Construction methods
impl<T> RedBlackTree<T> {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

/// Queries
impl<T> RedBlackTree<T> {
    /// Finds the predecessor of the given value, if it exists.
    /// Time complexity: O(log n).
    pub fn predecessor<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.get_root().and_then(|root| root.predecessor(value))
    }

    /// Finds the successor of the given value, if it exists.
    /// Time complexity: O(log n).
    pub fn successor<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.get_root().and_then(|root| root.successor(value))
    }

    /// Finds the stored value that equals the given value, if it exists.
    /// Time complexity: O(log n).
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.get_root().and_then(|root| root.get(value))
    }
}

/// Tree access
impl<T> RedBlackTree<T> {
    fn get_root(&self) -> Option<&RBNode<T>> {
        self.0.as_ref().map(|root| root.as_ref())
    }

    fn get_root_mut(&mut self) -> Option<&mut RBNode<T>> {
        self.0.as_mut().map(|root| root.as_mut())
    }
}

impl<T: Ord> RedBlackTree<T> {
    /// Adds a value to the tree. If there was an equal value already in the tree, nothing happens.
    /// Returns a Boolean indicating if the value was inserted or not.
    /// Time complexity: O(log n).
    pub fn insert(&mut self, value: T) -> bool {
        if let Some(tree) = self.get_root_mut() {
            tree.insert(value)
        } else {
            self.0 = Some(Box::new(RBNode::new(value)));
            true
        }
    }
}

impl<T: fmt::Display> fmt::Display for RedBlackTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.get_root() {
            Some(root) => root.fmt(f),
            None => write!(f, "└──L\n"),
        }
    }
}
