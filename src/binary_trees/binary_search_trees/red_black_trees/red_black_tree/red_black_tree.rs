use std::borrow::Borrow;

use super::{
    Cursor,
    CursorMut,
};
use crate::binary_trees::{
    binary_search_trees::{
        red_black_trees::{
            base, 
            ord_by_key::OrdByKey,
        },
        red_black_tree::iterators::{
            InorderIter,
            InorderIterMut,
            IntoInorderIter,
        },
    },
};

/// Struct containing the data in each node of the tree.
/// Values are considered equal if their keys are equal, regardless of what other data they store.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct RbData<K, V> {
    key: K,
    value: V,
}

impl<K, V> OrdByKey for RbData<K, V>
where 
    K: Ord,
{
    type Key = K;

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }

    fn cmp_to_key<Q>(&self, key: &Q) -> std::cmp::Ordering
    where
        Self::Key: Borrow<Q>,
        Q: Ord + ?Sized
    {
        self.key.borrow().cmp(key)
    }
}

impl<K, V> RbData<K, V> {
    fn into_value(self) -> V {
        self.value
    }

    pub(super) fn data(&self) -> (&K, &V) {
        (&self.key, &self.value)
    }

    pub(super) fn data_with_mut_value(&mut self) -> (&K, &mut V) {
        (&self.key, &mut self.value)
    }

    pub(super) fn into_data(self) -> (K, V) {
        (self.key, self.value)
    }
}

#[derive(Clone)]
pub struct RedBlackTree<K, V>(pub(super) base::RedBlackTree<RbData<K, V>>);

impl<K, V> Default for RedBlackTree<K, V> {
    fn default() -> Self {
        Self(base::RedBlackTree::default())
    }
}

impl<K, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        Self(base::RedBlackTree::new())
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
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let data = RbData {
            key,
            value,
        };
        self.0.insert(data, |_| {})
            .map(RbData::into_value)
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.remove(key, |_| {})
            .map(RbData::into_data)
    }
}

impl<K, V> Extend<(K, V)> for RedBlackTree<K, V>
where 
    K: Ord,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
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

impl<'t, K, V> IntoIterator for &'t RedBlackTree<K, V> {
    type Item = (&'t K, &'t V);
    type IntoIter = InorderIter<'t, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder_iter()
    }
}

impl<'t, K, V> IntoIterator for &'t mut RedBlackTree<K, V> {
    type Item = (&'t K, &'t mut V);
    type IntoIter = InorderIterMut<'t, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder_iter_mut()
    }
}

impl<K, V> IntoIterator for RedBlackTree<K, V> {
    type Item = (K, V);
    type IntoIter = IntoInorderIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_inorder_iter()
    }
}
