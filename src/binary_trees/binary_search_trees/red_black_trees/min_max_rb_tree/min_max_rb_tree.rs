use std::borrow::Borrow;

use slotmap::Key;

use super::{
    Cursor,
    CursorMut,
};
use crate::binary_trees::{
    binary_search_trees::{
        min_max_rb_tree::iterators::{
            InorderIter,
            InorderIterMut,
            IntoInorderIter,
        },
        red_black_trees::{
            base, 
            ord_by_key::OrdByKey,
        }
    },
    binary_tree::NodeId,
    binary_tree_cursor::PeekingCursorMut,
};

/// Struct containing the data in each node of the tree.
/// Values are considered equal if their keys are equal, regardless of what other data they store.
/// id_min and id_max store the ids of the leftmost, respectively rightmost node in the node's subtree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct MinMaxRbData<K, V> {
    key: K,
    value: V,
    id_min: NodeId,
    id_max: NodeId,
}

impl<K, V> OrdByKey for MinMaxRbData<K, V>
where 
    K: Ord,
{
    type Key = K;

    fn key(&self) -> &Self::Key {
        &self.key
    }
}

impl<K, V> MinMaxRbData<K, V> {
    fn value(&self) -> &V {
        &self.value
    }

    fn into_value(self) -> V {
        self.value
    }

    pub(super) fn id_min(&self) -> NodeId {
        self.id_min
    }

    pub(super) fn id_max(&self) -> NodeId {
        self.id_max
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
pub struct MinMaxRbTree<K, V>(pub(super) base::RedBlackTree<MinMaxRbData<K, V>>);

impl<K, V> Default for MinMaxRbTree<K, V> {
    fn default() -> Self {
        Self(base::RedBlackTree::default())
    }
}

impl<K, V> MinMaxRbTree<K, V> {
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

impl<K, V> MinMaxRbTree<K, V>
where 
    K: Ord,
{
    /// Recomputes the semigroup value of the current node.
    fn on_subtree_change(cursor: &mut base::CursorMut<'_, MinMaxRbData<K, V>>) {
        let node_id = cursor.node_id();
        let id_min = cursor.peek_left().map_or(node_id, MinMaxRbData::id_min);
        let id_max = cursor.peek_right().map_or(node_id, MinMaxRbData::id_max);
        if let Some(node) = cursor.get_mut() {
            node.id_min = id_min;
            node.id_max = id_max;
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let data = MinMaxRbData {
            key,
            value,
            id_min: NodeId::null(),
            id_max: NodeId::null(),
        };
        self.0.insert(data, Self::on_subtree_change)
            .map(MinMaxRbData::into_value)
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.remove(key, Self::on_subtree_change)
            .map(MinMaxRbData::into_data)
    }
}

impl<K, V> MinMaxRbTree<K, V>
where 
    K: Ord,
{
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.contains_key(key)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.get(key)
            .map(MinMaxRbData::value)
    }

    pub fn pred_key<Q>(&self, key: &Q) -> Option<&K>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.pred_data(key).map(|(k, ..)| k)
    }

    pub fn pred_data<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.pred(key)
            .map(MinMaxRbData::data)
    }

    pub fn pred_data_with_mut_value<Q>(&mut self, key: &Q) -> Option<(&K, &mut V)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.pred_mut(key)
            .map(MinMaxRbData::data_with_mut_value)
    }

    pub fn succ_key<Q>(&self, key: &Q) -> Option<&K>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.succ_data(key).map(|(k, ..)| k)
    }

    pub fn succ_data<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.succ(key)
            .map(MinMaxRbData::data)
    }

    pub fn succ_data_with_mut_value<Q>(&mut self, key: &Q) -> Option<(&K, &mut V)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.succ_mut(key)
            .map(MinMaxRbData::data_with_mut_value)
    }
}

impl<K, V> Extend<(K, V)> for MinMaxRbTree<K, V>
where 
    K: Ord,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<K, V> FromIterator<(K, V)> for MinMaxRbTree<K, V>
where 
    K: Ord,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut tree = Self::default();
        tree.extend(iter);
        tree
    }
}

impl<'t, K, V> IntoIterator for &'t MinMaxRbTree<K, V> {
    type Item = (&'t K, &'t V);
    type IntoIter = InorderIter<'t, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder_iter()
    }
}

impl<'t, K, V> IntoIterator for &'t mut MinMaxRbTree<K, V> {
    type Item = (&'t K, &'t mut V);
    type IntoIter = InorderIterMut<'t, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder_iter_mut()
    }
}

impl<K, V> IntoIterator for MinMaxRbTree<K, V> {
    type Item = (K, V);
    type IntoIter = IntoInorderIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_inorder_iter()
    }
}
