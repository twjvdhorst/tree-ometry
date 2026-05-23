use super::{
    BinarySearchTree,
    BstNode,
    InorderIter,
    InorderIterMut,
    IntoInorderIter,
};
use crate::binary_trees::{
    Side,
    binary_tree::BinaryTree,
    binary_tree_cursor::BinaryTreeCursor,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BinarySearchTreeError {
    #[error("tree is not a binary search tree")]
    BinarySearchTreeError,
}

impl<K, V> TryFrom<BinaryTree<(K, V)>> for BinarySearchTree<K, V>
where 
    K: Ord,
{
    type Error = BinarySearchTreeError;

    fn try_from(value: BinaryTree<(K, V)>) -> Result<Self, Self::Error> {
        value.map(|(k, v)| BstNode::new(k, v))
            .try_into()
    }
}

impl<K, V> TryFrom<BinaryTree<BstNode<K, V>>> for BinarySearchTree<K, V>
where 
    K: Ord,
{
    type Error = BinarySearchTreeError;

    fn try_from(value: BinaryTree<BstNode<K, V>>) -> Result<Self, Self::Error> {
        let mut iter = value.inorder_iter().peekable();
        while let Some(curr) = iter.next()
            && let Some(next) = iter.peek()
        {
            if curr.key() > next.key() {
                return Err(BinarySearchTreeError::BinarySearchTreeError);
            }
        }
        Ok(Self(value))
    }
}

impl<K, V> From<BinarySearchTree<K, V>> for BinaryTree<(K, V)> {
    fn from(value: BinarySearchTree<K, V>) -> Self {
        value.0.map(BstNode::into_data)
    }
}

impl<K, V> From<BinarySearchTree<K, V>> for BinaryTree<BstNode<K, V>> {
    fn from(value: BinarySearchTree<K, V>) -> Self {
        value.0
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
    type Item = (&'t K, &'t V);
    type IntoIter = InorderIter<'t, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder_iter()
    }
}

impl<'t, K, V> IntoIterator for &'t mut BinarySearchTree<K, V> {
    type Item = (&'t K, &'t mut V);
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
