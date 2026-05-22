use std::{borrow::Borrow, cmp::Ordering};

use crate::binary_trees::{Side, binary_search_tree::{Cursor, CursorMut}, binary_tree::BinaryTree, binary_tree_cursor::{BinaryTreeCursor, Neighborhood, PeekingCursorMut}};

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
    /// Moves the given cursor to either the direct predecessor or the direct successor of the given key.
    fn find_with_cursor<Q>(cursor: &mut CursorMut<'_, K, V>, key: &Q)
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        while let Neighborhood { node: Some(node), parent: Some(parent), .. } = cursor.peek_neighborhood() {
            match (K::cmp(&node.key, &parent.key), Q::cmp(node.key.borrow(), key)) {
                (Ordering::Greater, Ordering::Less) => {
                    // Predecessor of key is in the right subtree of the cursor.
                    while cursor.try_move_right() {}
                    return;
                },
                (Ordering::Less, Ordering::Greater) => {
                    // Successor of key is in the left subtree of the cursor.
                    while cursor.try_move_left() {}
                    return;
                },
                (Ordering::Equal, _) => return,
                _ => {
                    cursor.move_up();
                },
            }
        }

        // Cursor is at the root of the tree.
        while let Some(node) = cursor.get() {
            match Q::cmp(node.key.borrow(), key) {
                Ordering::Greater => if !cursor.try_move_left() { return; },
                Ordering::Less => if !cursor.try_move_right() { return; },
                Ordering::Equal => return,
            }
        }
    }
}

impl<K, V> FromIterator<(K, V)> for BinarySearchTree<K, V>
where 
    K: Ord,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut tree = Self::new();
        let mut cursor = tree.cursor_mut();
        for (key, value) in iter.into_iter() {
            Self::find_with_cursor(&mut cursor, &key);
            if let Some(node) = cursor.get() {
                match K::cmp(&node.key, &key) {
                    Ordering::Greater => cursor.attach_or_insert_child(key, value, Side::Right).unwrap(), // Insert a new node as the cursor's right child.
                    _ => cursor.attach_or_insert_child(key, value, Side::Left).unwrap(), // Insert a new node as the cursor's left child.
                }
            } else {
                cursor.root_tree(key, value).unwrap();
            }
        }
        tree
    }
}
