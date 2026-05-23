use std::fmt::{Debug, Display};

use std::marker::PhantomData;
use std::cmp::Ordering;

use crate::binary_trees::cartesian_tree::{
    InorderIter,
    InorderIterMut,
    IntoInorderIter,
};
use crate::binary_trees::{
    Side, 
    binary_tree::BinaryTree, 
    binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursorMut,
    },
};
use super::{cursors::{Cursor, CursorMut}};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

pub struct Min;
pub struct Max;

pub(super) trait Comparer {
    fn name() -> &'static str;

    fn compare<T>(left: &T, right: &T) -> Ordering
    where T: Ord;
}

impl Comparer for super::Min {
    fn name() -> &'static str {
        "min"
    }

    fn compare<T>(left: &T, right: &T) -> Ordering
    where T: Ord
    {
        T::cmp(left, right)
    }
}

impl Comparer for super::Max {
    fn name() -> &'static str {
        "max"
    }

    fn compare<T>(left: &T, right: &T) -> Ordering
    where T: Ord
    {
        match T::cmp(left, right) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CartesianNode<K, V> {
    key: K,
    value: V,
}

impl<K, V> CartesianNode<K, V> {
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
}

impl<K, V> Into<(K, V)> for CartesianNode<K, V> {
    fn into(self) -> (K, V) {
        (self.key, self.value)
    }
}

#[derive(Clone)]
pub struct CartesianTree<K, V, C>(pub(super) BinaryTree<CartesianNode<K, V>>, pub(super) PhantomData<C>);
pub type MinCartesianTree<K, V> = CartesianTree<K, V, Min>;
pub type MaxCartesianTree<K, V> = CartesianTree<K, V, Max>;

impl<K, V, C> Default for CartesianTree<K, V, C> {
    fn default() -> Self {
        Self(BinaryTree::default(), PhantomData)
    }
}

impl<K, V, C> CartesianTree<K, V, C> {
    fn new() -> Self {
        Self::default()
    }

    fn with_capacity(capacity: usize) -> Self {
        Self(BinaryTree::with_capacity(capacity), PhantomData)
    }

    pub(super) fn inner(&self) -> &BinaryTree<CartesianNode<K, V>> {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn map_values<U, F>(self, f: F) -> CartesianTree<K, U, C>
    where 
        F: Fn(V) -> U,
    {
        let f = |node: CartesianNode<K, V>| CartesianNode { key: node.key, value: f(node.value) };
        CartesianTree(self.0.map(f), PhantomData)
    }

    pub fn cursor(&self) -> Cursor<'_, K, V> {
        Cursor::new(self.0.cursor())
    }

    pub fn cursor_mut(&mut self) -> CursorMut<'_, K, V> {
        CursorMut::new(self.0.cursor_mut())
    }
}

impl<K, V, C> FromIterator<(K, V)> for CartesianTree<K, V, C>
where
    K: Ord,
    C: Comparer,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let mut tree = if let Some(capacity) = iter.size_hint().1 {
            Self::with_capacity(capacity)
        } else {
            Self::new()
        };
        let mut cursor = tree.cursor_mut();
        for (key, value) in iter {
            // Find the node that becomes the parent of the new node.
            while let Some(node) = cursor.get() && C::compare(node.key(), &key) == Ordering::Greater {
                cursor.move_up();
            }

            let new_node = CartesianNode { key, value };
            if cursor.get().is_none() {
                cursor.re_root_tree(new_node, Side::Left);
            } else {
                cursor.attach_or_insert_child(new_node, Side::Right).unwrap();
                cursor.move_right();
                // If the new node was inserted into an edge, its child node must move from the right side to the left side.
                let _ = cursor.swap_children();
            }
        }
        
        tree
    }
}

impl<'t, K, V, C> IntoIterator for &'t CartesianTree<K, V, C> {
    type Item = &'t CartesianNode<K, V>;
    type IntoIter = InorderIter<'t, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder_iter()
    }
}

impl<'t, K, V, C> IntoIterator for &'t mut CartesianTree<K, V, C> {
    type Item = &'t mut CartesianNode<K, V>;
    type IntoIter = InorderIterMut<'t, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder_iter_mut()
    }
}

impl<K, V, C> IntoIterator for CartesianTree<K, V, C> {
    type Item = (K, V);
    type IntoIter = IntoInorderIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_inorder_iter()
    }
}

impl<K, V> Debug for CartesianNode<K, V>
where 
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}: {:?})", self.key, self.value)
    }
}

impl<K, V, C> Debug for CartesianTree<K, V, C>
where 
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<K, V> Display for CartesianNode<K, V>
where 
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {})", self.key, self.value)
    }
}

impl<K, V, C> Display for CartesianTree<K, V, C>
where 
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_trees::{
        binary_tree::{
            self,
            BinaryTree,
        },
        binary_tree_cursor::{
            BinaryTreeCursor, 
            PeekingCursor,
        },
    };

    use rand::prelude::*;
    use serde::Serialize;

    fn is_heap<K, V, C>(tree: &BinaryTree<CartesianNode<K, V>>) -> bool
    where 
        K: Ord,
        C: Comparer,
    {
        fn is_heap_recursive<K, V, C>(cursor: binary_tree::Cursor<'_, CartesianNode<K, V>>) -> bool
        where
            K: Ord,
            C: Comparer,
        {
            let Some(node) = cursor.get() else { return true; };
            if let Some(left) = cursor.peek_left() {
                if C::compare(node.key(), left.key()) == Ordering::Greater {
                    return false;
                }
                let mut left_cursor = cursor.spawn_cursor();
                left_cursor.move_left();
                if !is_heap_recursive::<_, _, C>(left_cursor) {
                    return false;
                };
            }
            if let Some(right) = cursor.peek_right() {
                if C::compare(node.key(), right.key()) == Ordering::Greater {
                    return false;
                }
                let mut right_cursor = cursor.spawn_cursor();
                right_cursor.move_right();
                if !is_heap_recursive::<_, _, C>(right_cursor) {
                    return false;
                };
            }
            true
        }
            
        is_heap_recursive::<_, _, C>(tree.cursor())
    }

    fn assert_cartesian_tree<K, V>(sequence: Vec<(K, V)>)
    where 
        K: Clone + Debug + Ord + Serialize,
        V: Clone + Debug + Eq + Serialize,
    {
        let tree = sequence.clone()
            .into_iter()
            .collect::<CartesianTree<_, _, Max>>();
        assert!(is_heap::<_, _, Max>(tree.inner()));
        
        // Assert the sequence is preserved.
        let mut tree_sequence = Vec::new();
        let mut iter = tree.inorder_iter();
        while let Some(node) = iter.next() {
            tree_sequence.push((node.key.clone(), node.value.clone()));
        }
        for i in 0..sequence.len() {
            assert_eq!(sequence.get(i), tree_sequence.get(i));
        }
    }

    #[test]
    fn test_cartesian_tree() {
        let mut rng = rand::rng();
        for _ in 0..50 {
            let mut sequence = (1..=30).map(|x| (x, ())).collect::<Vec<_>>();
            sequence.shuffle(&mut rng);
            assert_cartesian_tree(sequence);
        }
    }
}
