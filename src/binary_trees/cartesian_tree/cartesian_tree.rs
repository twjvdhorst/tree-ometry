use std::{borrow::{Borrow, BorrowMut}, fmt::{Debug, Display}};
use paste::paste;

use std::marker::PhantomData;
use std::cmp::Ordering;

use crate::binary_trees::{
    Side, 
    binary_tree::{
        self,
        BinaryTree,
    },
    tree_iterators::{self, *},
    traits::{
        self,
        BinaryTree as BinaryTreeTrait,
        BinaryTreeMut, 
        binary_tree_cursor::{
            BinaryTreeCursor, 
            PeekingCursor,
            PeekingCursorMut,
        },
    },
};
use super::{cursors::{Cursor, CursorMut}};

use thiserror::Error;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Error, Debug)]
pub enum CartesianTreeError {
    #[error("tree is not a {0} heap")]
    HeapError(String),
}

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
pub struct CartesianTreeNode<K, V> {
    key: K,
    value: V,
}

impl<K, V> CartesianTreeNode<K, V> {
    pub fn new(key: K, value: V) -> Self {
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

#[derive(Clone)]
pub struct CartesianTree<K, V, C>(BinaryTree<CartesianTreeNode<K, V>>, PhantomData<C>);
pub type MinCartesianTree<K, V> = CartesianTree<K, V, Min>;
pub type MaxCartesianTree<K, V> = CartesianTree<K, V, Max>;

impl<K, V, C> Default for CartesianTree<K, V, C> {
    fn default() -> Self {
        Self(BinaryTree::default(), PhantomData)
    }
}

impl<K, V, C> CartesianTree<K, V, C> {
    pub fn new() -> Self {
        Self::default()
    }

    tree_iterators::impl_iters!(pub, inorder, CartesianTreeNode<K, V>);
    tree_iterators::impl_iters!(pub, preorder, CartesianTreeNode<K, V>);
    tree_iterators::impl_iters!(pub, postorder, CartesianTreeNode<K, V>);
}

impl<K, V, C> FromIterator<(K, V)> for CartesianTree<K, V, C>
where
    K: Ord,
    C: Comparer,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut tree = Self::default();
        let mut cursor = tree.cursor_mut();
        for (key, value) in iter {
            // Find the node that becomes the parent of the new node.
            while let Some(node) = cursor.node() && C::compare(node.key(), &key) == Ordering::Greater {
                cursor.move_up();
            }

            let new_node = CartesianTreeNode { key, value };
            if cursor.node().is_none() {
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

fn is_heap<K, V, C>(tree: &BinaryTree<CartesianTreeNode<K, V>>) -> bool
where 
    K: Ord,
    C: Comparer,
{
    fn is_heap_recursive<K, V, C>(cursor: binary_tree::Cursor<'_, CartesianTreeNode<K, V>>) -> bool
    where
        K: Ord,
        C: Comparer,
    {
        let Some(node) = cursor.node() else { return true; };
        if let Some(left) = cursor.peek_left() {
            if C::compare(node.data().key(), left.data().key()) == Ordering::Greater {
                return false;
            }
            let mut left_cursor = cursor.spawn_cursor();
            left_cursor.move_left();
            if !is_heap_recursive::<_, _, C>(left_cursor) {
                return false;
            };
        }
        if let Some(right) = cursor.peek_right() {
            if C::compare(node.data().key(), right.data().key()) == Ordering::Greater {
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

impl<K, V, C> TryFrom<BinaryTree<CartesianTreeNode<K, V>>> for CartesianTree<K, V, C>
where
    K: Ord,
    C: Comparer,
{
    type Error = CartesianTreeError;

    fn try_from(value: BinaryTree<CartesianTreeNode<K, V>>) -> Result<Self, Self::Error> {
        if !is_heap::<_, _, C>(&value) {
            Err(CartesianTreeError::HeapError(String::from(C::name())))
        } else {
            Ok(Self(value, PhantomData))
        }
    }
}

impl<K, V, C> Borrow<BinaryTree<CartesianTreeNode<K, V>>> for CartesianTree<K, V, C> {
    fn borrow(&self) -> &BinaryTree<CartesianTreeNode<K, V>> {
        &self.0
    }
}

impl<K, V, C> BorrowMut<BinaryTree<CartesianTreeNode<K, V>>> for CartesianTree<K, V, C> {
    fn borrow_mut(&mut self) -> &mut BinaryTree<CartesianTreeNode<K, V>> {
        &mut self.0
    }
}

impl<K, V, C> From<CartesianTree<K, V, C>> for BinaryTree<CartesianTreeNode<K, V>> {
    fn from(value: CartesianTree<K, V, C>) -> Self {
        value.0
    }
}

impl<K, V, C> traits::BinaryTree for CartesianTree<K, V, C> {
    type Node = CartesianTreeNode<K, V>;
    type Cursor<'c> = Cursor<'c, K, V>
    where Self: 'c;

    fn cursor(&self) -> Self::Cursor<'_> {
        Cursor::new(self.0.cursor())
    }
}

impl<K, V, C> traits::BinaryTreeMut for CartesianTree<K, V, C> {
    type CursorMut<'c> = CursorMut<'c, K, V>
    where Self: 'c;

    fn cursor_mut(&mut self) -> Self::CursorMut<'_> {
        CursorMut::new(self.0.cursor_mut())
    }
}

impl<K, V> Debug for CartesianTreeNode<K, V>
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
        traits::binary_tree::fmt_debug_binary_tree(self, f)
    }
}

impl<K, V> Display for CartesianTreeNode<K, V>
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
        traits::binary_tree::fmt_display_binary_tree(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::prelude::*;
    use serde::Serialize;

    fn assert_cartesian_tree<K, V>(sequence: Vec<(K, V)>)
    where 
        K: Clone + Debug + Ord + Serialize,
        V: Clone + Debug + Eq + Serialize,
    {
        let tree = sequence.clone()
            .into_iter()
            .collect::<CartesianTree<_, _, Max>>();
        assert!(is_heap::<_, _, Max>(tree.borrow()));
        
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
