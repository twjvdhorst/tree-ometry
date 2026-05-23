use std::{borrow::Borrow, cmp::Ordering, fmt::{Debug, Display}};

use super::{
    InorderIter,
    InorderIterMut,
    IntoInorderIter,
};
use crate::binary_trees::{
    Side, 
    binary_search_tree::{
        Cursor, 
        CursorMut,
    }, 
    binary_tree::BinaryTree,
    binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursor,
        PeekingCursorMut,
    },
};

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

    pub fn data(&self) -> (&K, &V) {
        (&self.key, &self.value)
    }

    pub fn data_with_mut_value(&mut self) -> (&K, &mut V) {
        (&self.key, &mut self.value)
    }

    pub fn into_data(self) -> (K, V) {
        (self.key, self.value)
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
        while let Some((curr_key, _)) = cursor.get() {
            match Q::cmp(curr_key.borrow(), key) {
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
        self.pred_data(key).map(|(pred_key, _)| pred_key)
    }

    pub fn succ_key<Q>(&self, key: &Q) -> Option<&K>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        self.succ_data(key).map(|(succ_key, _)| succ_key)
    }

    pub fn pred_data<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        let mut cursor = self.cursor();
        let mut pred = None;
        while let Some(data @ (curr_key, _)) = cursor.get() {
            match Q::cmp(curr_key.borrow(), key) {
                Ordering::Greater => cursor.move_left(),
                Ordering::Less => {
                    pred = Some(data);
                    cursor.move_right();
                },
                Ordering::Equal => return Some(data),
            }
        }
        pred
    }

    pub fn succ_data<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        let mut cursor = self.cursor();
        let mut succ = None;
        while let Some(data @ (curr_key, _)) = cursor.get() {
            match Q::cmp(curr_key.borrow(), key) {
                Ordering::Greater => {
                    succ = Some(data);
                    cursor.move_left();
                },
                Ordering::Less => cursor.move_right(),
                Ordering::Equal => return Some(data),
            }
        }
        succ
    }

    pub fn pred_data_with_mut_value<Q>(&mut self, key: &Q) -> Option<(&K, &mut V)>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        let mut cursor = self.cursor_mut();
        let mut depth_since_pred = None;
        while let Some((curr_key, _)) = cursor.get() {
            match Q::cmp(curr_key.borrow(), key) {
                Ordering::Greater => {
                    if cursor.try_move_left() {
                        if let Some(depth) = depth_since_pred {
                            depth_since_pred = Some(depth + 1);
                        }
                    } else {
                        // Move the cursor back to the last seen predecessor.
                        let depth = depth_since_pred?;
                        for _ in 0..depth {
                            cursor.move_up();
                        }
                        break;
                    }
                },
                Ordering::Less => {
                    if cursor.try_move_right() {
                        depth_since_pred = Some(1);
                    } else {
                        break;
                    }
                },
                Ordering::Equal => break,
            }
        }

        // Cursor is in the predecessor.
        // Extend the lifetime of the yielded references to be independent of the cursor.
        // This is safe, because we don't alter the tree or any value after returning.
        let (pred_key, pred_value) = cursor.get_mut()?;
        let key_pointer = pred_key as *const K;
        let value_pointer = pred_value as *mut V;
        unsafe { Some((&*key_pointer, &mut *value_pointer)) }
    }

    pub fn succ_node_mut<Q>(&mut self, key: &Q) -> Option<(&K, &mut V)>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        let mut cursor = self.cursor_mut();
        let mut depth_since_succ = None;
        while let Some((curr_key, _)) = cursor.get() {
            match Q::cmp(curr_key.borrow(), key) {
                Ordering::Greater => {
                    if cursor.try_move_left() {
                        depth_since_succ = Some(1);
                    } else {
                        break;
                    }
                },
                Ordering::Less => {
                    if cursor.try_move_right() {
                        if let Some(depth) = depth_since_succ {
                            depth_since_succ = Some(depth + 1);
                        }
                    } else {
                        // Move the cursor back to the last seen successor.
                        let depth = depth_since_succ?;
                        for _ in 0..depth {
                            cursor.move_up();
                        }
                        break;
                    }
                },
                Ordering::Equal => break,
            }
        }

        // Cursor is in the successor.
        // Extend the lifetime of the yielded reference to be independent of the cursor.
        // This is safe, because we don't alter the tree or any value after returning.
        let (succ_key, succ_value) = cursor.get_mut()?;
        let key_pointer = succ_key as *const K;
        let value_pointer = succ_value as *mut V;
        unsafe { Some((&*key_pointer, &mut *value_pointer)) }
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
