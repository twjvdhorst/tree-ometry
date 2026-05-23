use std::{borrow::Borrow, cmp::Ordering, fmt::{Debug, Display}};

use super::{
    InorderIter,
    InorderIterMut,
    IntoInorderIter,
};
use crate::binary_trees::{Side, binary_search_tree::{Cursor, CursorMut}, binary_tree::BinaryTree, binary_tree_cursor::{BinaryTreeCursor, PeekingCursor}};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BstNode<K, V> {
    key: K,
    value: V,
}

impl<K, V> BstNode<K, V> {
    pub(super) fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
        }
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }
}

impl<K, V> Into<(K, V)> for BstNode<K, V> {
    fn into(self) -> (K, V) {
        (self.key, self.value)
    }
}

#[derive(Clone)]
pub struct BinarySearchTree<K, V>(pub(super) BinaryTree<BstNode<K, V>>);

impl<K, V> Default for BinarySearchTree<K, V> {
    fn default() -> Self {
        Self(BinaryTree::default())
    }
}

impl<K, V> BinarySearchTree<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(BinaryTree::with_capacity(capacity))
    }

    pub fn rebalance(&mut self) {
        self.0.rebalance();
    }

    pub fn cursor(&self) -> Cursor<'_, K, V> {
        Cursor::new(self.0.cursor())
    }

    pub fn cursor_mut(&mut self) -> CursorMut<'_, K, V> {
        CursorMut::new(self.0.cursor_mut())
    }
}

impl<K, V> BinarySearchTree<K, V>
where 
    K: Ord,
{
    pub fn contains<Q>(&self, key: &Q) -> bool
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        let mut cursor = self.cursor();
        while let Some(node) = cursor.get() {
            match Q::cmp(node.key.borrow(), key) {
                Ordering::Greater => cursor.move_left(),
                Ordering::Less => cursor.move_right(),
                Ordering::Equal => return true,
            }
        }
        false
    }

    pub fn pred_key<Q>(&self, key: &Q) -> Option<&K>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        self.pred_node(key).map(BstNode::key)
    }

    pub fn succ_key<Q>(&self, key: &Q) -> Option<&K>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        self.succ_node(key).map(BstNode::key)
    }

    pub fn pred_node<Q>(&self, key: &Q) -> Option<&BstNode<K, V>>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        let mut cursor = self.cursor();
        let mut pred = None;
        while let Some(node) = cursor.get() {
            match Q::cmp(node.key.borrow(), key) {
                Ordering::Greater => cursor.move_left(),
                Ordering::Less => {
                    pred = Some(node);
                    cursor.move_right();
                },
                Ordering::Equal => return Some(node),
            }
        }
        pred
    }

    pub fn succ_node<Q>(&self, key: &Q) -> Option<&BstNode<K, V>>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        let mut cursor = self.cursor();
        let mut succ = None;
        while let Some(node) = cursor.get() {
            match Q::cmp(node.key.borrow(), key) {
                Ordering::Greater => {
                    succ = Some(node);
                    cursor.move_left();
                },
                Ordering::Less => cursor.move_right(),
                Ordering::Equal => return Some(node),
            }
        }
        succ
    }
}

impl<K, V> FromIterator<(K, V)> for BinarySearchTree<K, V>
where 
    K: Ord,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        // Start from a right-leaning path and rebalance.
        let mut sorted_sequence = iter.into_iter().collect::<Vec<_>>();
        sorted_sequence.sort_by(|(k1, _), (k2, _)| K::cmp(k1, k2));
        let mut tree = Self::with_capacity(sorted_sequence.len());
        let mut cursor = tree.cursor_mut();
        for (key, value) in sorted_sequence.into_iter() {
            cursor.attach_child(key, value, Side::Right).unwrap();
            cursor.move_right();
        }
        tree.rebalance();
        tree
    }
}

impl<'t, K, V> IntoIterator for &'t BinarySearchTree<K, V> {
    type Item = &'t BstNode<K, V>;
    type IntoIter = InorderIter<'t, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder_iter()
    }
}

impl<'t, K, V> IntoIterator for &'t mut BinarySearchTree<K, V> {
    type Item = &'t mut BstNode<K, V>;
    type IntoIter = InorderIterMut<'t, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder_iter_mut()
    }
}

impl<K, V> IntoIterator for BinarySearchTree<K, V> {
    type Item = (K, V);
    type IntoIter = IntoInorderIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_inorder_iter()
    }
}

impl<K, V> Debug for BstNode<K, V>
where 
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}: {:?})", self.key, self.value)
    }
}

impl<K, V> Debug for BinarySearchTree<K, V>
where 
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<K, V> Display for BstNode<K, V>
where 
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {})", self.key, self.value)
    }
}

impl<K, V> Display for BinarySearchTree<K, V>
where 
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
