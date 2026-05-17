use crate::binary_trees::{
    binary_tree::{
        self, 
        BinaryTree, 
        BinaryTreeNode
    }, 
    red_black_tree::RedBlackNode,
    semigroup_rb_tree::{
        SemigroupRbNode, 
        SemigroupRbTree, 
        TreeSemigroup
    }, 
    traits::{
        binary_tree::BinaryTree as BinaryTreeTrait, 
        binary_tree_cursor::{
            BinaryTreeCursor, 
            PeekingCursor
        }
    }
};

use ref_cast::RefCast;
use thiserror::Error;

impl<K, V> AsRef<RedBlackNode<K, V>> for SemigroupRbNode<K, V, ()> {
    fn as_ref(&self) -> &RedBlackNode<K, V> {
        RedBlackNode::ref_cast(self)
    }
}

impl<K, V> AsMut<RedBlackNode<K, V>> for SemigroupRbNode<K, V, ()> {
    fn as_mut(&mut self) -> &mut RedBlackNode<K, V> {
        RedBlackNode::ref_cast_mut(self)
    }
}

#[derive(Error, Debug)]
pub enum SemigroupRbTreeError {
    #[error("tree is not a binary search tree")]
    BinarySearchTreeError,
    #[error("tree is not a red-black tree")]
    RedBlackTreeError,
    #[error("semigroup values are incorrect")]
    SemigroupError,
}

fn is_binary_search_tree<K, V, S>(tree: &BinaryTree<SemigroupRbNode<K, V, S>>) -> bool
where 
    K: Ord,
{
    fn is_binary_search_tree_recursive<'t, K, V, S>(cursor: binary_tree::Cursor<'t, SemigroupRbNode<K, V, S>>) -> (bool, Option<(&'t K, &'t K)>)
    where
        K: Ord,
    {
        let Some(node) = cursor.node().map(BinaryTreeNode::data) else { return (true, None); };
        let mut left_cursor = cursor;
        let mut right_cursor = cursor.clone();
        left_cursor.move_left();
        right_cursor.move_right();
        let (is_left_bst, left_range) = is_binary_search_tree_recursive(left_cursor);
        let (is_right_bst, right_range) = is_binary_search_tree_recursive(right_cursor);

        if !is_left_bst || !is_right_bst {
            return (false, None);
        }

        if let Some((_, max_left)) = left_range && max_left > node.key() {
            return (false, None);
        }

        if let Some((min_right, _)) = right_range && min_right < node.key() {
            return (false, None);
        }

        (true, Some((
            left_range.map_or(node.key(), |(min, _)| min),
            right_range.map_or(node.key(), |(_, max)| max)
        )))
    }
    
    is_binary_search_tree_recursive(tree.cursor()).0
}

fn is_red_black_tree<K, V, S>(tree: &BinaryTree<SemigroupRbNode<K, V, S>>) -> bool
where 
    K: Ord,
{
    /// Determines whether the given tree is a valid red-black tree, and returns the number of black nodes on any root-to-leaf path in the tree.
    fn is_red_black_tree_recursive<K, V, S>(cursor: binary_tree::Cursor<'_, SemigroupRbNode<K, V, S>>) -> (bool, Option<usize>)
    where
        K: Ord,
    {
        // Leaves are black.
        let Some(node) = cursor.node().map(BinaryTreeNode::data) else { return (true, Some(1)); };

        // No red-red edge.
        let left = cursor.peek_left().map(BinaryTreeNode::data);
        let right = cursor.peek_right().map(BinaryTreeNode::data);
        if node.is_red() &&
            (left.map_or(false, SemigroupRbNode::is_red) || right.map_or(false, SemigroupRbNode::is_red))
        {
            return (false, None);
        }

        // Check validity of subtrees.
        let mut left_cursor = cursor;
        let mut right_cursor = cursor.clone();
        left_cursor.move_left();
        right_cursor.move_right();
        let (is_left_rb, num_black_left) = is_red_black_tree_recursive(left_cursor);
        let (is_right_rb, num_black_right) = is_red_black_tree_recursive(right_cursor);
        if !is_left_rb || !is_right_rb || !(num_black_left == num_black_right) {
            return (false, None);
        }

        // Return number of black nodes on any root-to-leaf path.
        if node.is_red() {
            (true, num_black_left)
        } else {
            (true, Some(1 + num_black_left.unwrap_or(0)))
        }
    }
    
    let cursor = tree.cursor();

    // Root must be black.
    if let Some(node) = cursor.node().map(BinaryTreeNode::data) && !node.is_black() {
        return false;
    }

    is_red_black_tree_recursive(cursor).0
}

fn check_semigroup_values<K, V, S>(tree: &BinaryTree<SemigroupRbNode<K, V, S>>) -> bool
where 
    S: TreeSemigroup<K> + PartialEq,
{
    fn check_semigroup_values_recursive<K, V, S>(cursor: binary_tree::Cursor<'_, SemigroupRbNode<K, V, S>>) -> bool
    where 
        S: TreeSemigroup<K> + PartialEq,
    {
        let Some(node) = cursor.node().map(BinaryTreeNode::data) else { return true; };
        let binary_tree::Neighborhood { left, right, .. } = cursor.peek_neighborhood();
        let left = left.map(BinaryTreeNode::data);
        let right = right.map(BinaryTreeNode::data);
        let semigroup_value = S::op(node.key(), left.map(SemigroupRbNode::semigroup_value), right.map(SemigroupRbNode::semigroup_value));
        if *node.semigroup_value() != semigroup_value {
            return false;
        }
            
        let mut left_cursor = cursor;
        let mut right_cursor = cursor.spawn_cursor();
        left_cursor.move_left();
        right_cursor.move_right();
        check_semigroup_values_recursive(left_cursor) && check_semigroup_values_recursive(right_cursor)
    }

    check_semigroup_values_recursive(tree.cursor())
}

impl<K, V, S> TryFrom<BinaryTree<SemigroupRbNode<K, V, S>>> for SemigroupRbTree<K, V, S>
where
    K: Ord,
    S: TreeSemigroup<K> + PartialEq,
{
    type Error = SemigroupRbTreeError;

    fn try_from(value: BinaryTree<SemigroupRbNode<K, V, S>>) -> Result<Self, Self::Error> {
        if !is_binary_search_tree(&value) {
            Err(SemigroupRbTreeError::BinarySearchTreeError)
        } else if !is_red_black_tree(&value) {
            Err(SemigroupRbTreeError::RedBlackTreeError)
        } else if !check_semigroup_values(&value) {
            Err(SemigroupRbTreeError::SemigroupError)
        } else {
            Ok(Self(value))
        }
    }
}
