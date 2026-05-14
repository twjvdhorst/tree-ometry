use ref_cast::RefCast;
use std::{borrow::Borrow, fmt::{Debug, Display}};

use crate::binary_trees::semigroup_rb_tree::{SemigroupRbNode, SemigroupRbTree};
use super::{cursors::{Cursor, CursorMut}};

#[derive(Clone, Copy, PartialEq, Eq, RefCast)]
#[repr(transparent)]
pub struct RedBlackNode<K, V>(SemigroupRbNode<K, V, ()>);

impl<K, V> From<SemigroupRbNode<K, V, ()>> for RedBlackNode<K, V> {
    fn from(value: SemigroupRbNode<K, V, ()>) -> Self {
        Self(value)
    }
}

impl<K, V> RedBlackNode<K, V> {
    pub fn key(&self) -> &K {
        self.0.key()
    }

    pub fn value(&self) -> &V {
        self.0.value()
    }

    pub fn value_mut(&mut self) -> &mut V {
        self.0.value_mut()
    }

    pub fn into_data(self) -> (K, V) {
        self.0.into_data()
    }
}

#[derive(Clone)]
pub struct RedBlackTree<K, V>(SemigroupRbTree<K, V, ()>);

impl<K, V> Default for RedBlackTree<K, V> {
    fn default() -> Self {
        Self(SemigroupRbTree::default())
    }
}

impl<K, V> Extend<(K, V)> for RedBlackTree<K, V>
where 
    K: Ord,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            self.insert(key, value);
        }
    }
}

impl<K, V> FromIterator<(K, V)> for RedBlackTree<K, V>
where 
    K: Ord,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut tree = Self::default();
        tree.extend(iter);
        tree
    }
}

impl<K, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn root(&self) -> Option<&RedBlackNode<K, V>> {
        self.0.root().map(Borrow::borrow)
    }

    pub fn cursor(&self) -> Cursor<'_, K, V> {
        Cursor::new(self.0.cursor())
    }

    pub fn cursor_mut(&mut self) -> CursorMut<'_, K, V> {
        CursorMut::new(self.0.cursor_mut())
    }
}

impl<K, V> RedBlackTree<K, V>
where 
    K: Ord,
{
    /// Inserts the key-value pair into the tree.
    /// If the key was not present in the tree yet, None is returned.
    /// Otherwise, the value stored at the given key is updated, and the old value is returned.
    /// Time complexity: O(log n).
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.insert(key, value)
    }

    /// Removes the node with the given key from the tree.
    /// Returns the key and associated value.
    /// Time complexity: O(log n).
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized + Debug,
    {
        self.0.remove_entry(key)
    }
}

impl<K, V> Debug for RedBlackNode<K, V>
where 
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.0.color() {
            crate::binary_trees::semigroup_rb_tree::Color::Red => "r",
            crate::binary_trees::semigroup_rb_tree::Color::Black => "b",
        };
        write!(f, "({:?}: {:?}) ({c})", self.0.key(), self.0.value())
    }
}

impl<K, V> Debug for RedBlackTree<K, V>
where 
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/*
impl<K, V> Display for RedBlackNode<K, V>
where 
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {})", self.0.key(), self.0.value())
    }
}

impl<K, V> Display for RedBlackTree<K, V>
where 
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
*/
