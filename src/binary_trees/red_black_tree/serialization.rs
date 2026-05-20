use serde::{Serialize, Deserialize, de::Error};
use thiserror::Error;

use crate::binary_trees::{
    binary_tree::{
        self, 
        BinaryTree,
    }, red_black_tree::{
        Color,
        RedBlackNode,
        RedBlackTree,
    },
    binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursor,
    },
};

impl<K, V> Serialize for RedBlackTree<K, V>
where 
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        SerializationTree::new(self).serialize(serializer)
    }
}

impl<'de, K, V> Deserialize<'de> for RedBlackTree<K, V>
where 
    K: Ord + Clone + Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        // Serialize into a BinaryTree<RedBlackNode<K, V>>, and try to convert that to a RedBlackTree<K, V>.
        let tree = SerializationTree::deserialize(deserializer)?;
        let tree = binary_tree::serialization::SerializationTree(tree.0.map(binary_tree::serialization::SerializationNode::from));
        let tree = BinaryTree::from(tree);

        if !is_binary_search_tree(&tree) {
            Err(RedBlackTreeError::BinarySearchTreeError).map_err(D::Error::custom)?;
        }
        if !is_red_black_tree(&tree) {
            Err(RedBlackTreeError::RedBlackTreeError).map_err(D::Error::custom)?;
        }
        Ok(Self(tree))
    }
}

#[derive(Error, Debug)]
pub enum RedBlackTreeError {
    #[error("tree is not a binary search tree")]
    BinarySearchTreeError,
    #[error("tree is not a red-black tree")]
    RedBlackTreeError,
}

fn is_binary_search_tree<K, V>(tree: &BinaryTree<RedBlackNode<K, V>>) -> bool
where 
    K: Ord,
{
    fn is_binary_search_tree_recursive<'t, K, V>(cursor: binary_tree::Cursor<'t, RedBlackNode<K, V>>) -> (bool, Option<(&'t K, &'t K)>)
    where
        K: Ord,
    {
        let Some(node) = cursor.get() else { return (true, None); };
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

fn is_red_black_tree<K, V>(tree: &BinaryTree<RedBlackNode<K, V>>) -> bool
where 
    K: Ord,
{
    /// Determines whether the given tree is a valid red-black tree, and returns the number of black nodes on any root-to-leaf path in the tree.
    fn is_red_black_tree_recursive<K, V>(cursor: binary_tree::Cursor<'_, RedBlackNode<K, V>>) -> (bool, Option<usize>)
    where
        K: Ord,
    {
        // Leaves are black.
        let Some(node) = cursor.get() else { return (true, Some(1)); };

        // No red-red edge.
        let left = cursor.peek_left();
        let right = cursor.peek_right();
        if node.is_red() &&
            (left.map_or(false, RedBlackNode::is_red) || right.map_or(false, RedBlackNode::is_red))
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
    if let Some(node) = cursor.get() && !node.is_black() {
        return false;
    }

    is_red_black_tree_recursive(cursor).0
}

#[derive(Serialize, Deserialize)]
pub struct SerializationNode<K, V> {
    pub(in super::super) key: K,
    pub(in super::super) value: V,
    pub(in super::super) color: Color,
    pub(in super::super) left: Option<Box<SerializationNode<K, V>>>,
    pub(in super::super) right: Option<Box<SerializationNode<K, V>>>,
}

impl<'t, K, V> From<binary_tree::serialization::SerializationNode<&'t RedBlackNode<K, V>>> for SerializationNode<&'t K, &'t V> {
    fn from(value: binary_tree::serialization::SerializationNode<&'t RedBlackNode<K, V>>) -> Self {
        let binary_tree::serialization::SerializationNode {
            data: node,
            left,
            right
        } = value;
        let left = left.map(|node| Self::from(*node));
        let right = right.map(|node| Self::from(*node));
        Self {
            key: node.key(),
            value: node.value(),
            color: node.color(),
            left: left.map(Into::into),
            right: right.map(Into::into),
        }
    }
}

impl<K, V> From<SerializationNode<K, V>> for binary_tree::serialization::SerializationNode<RedBlackNode<K, V>> {
    fn from(value: SerializationNode<K, V>) -> Self {
        let SerializationNode { key, value, color, left, right } = value;
        let left = if let Some(node) = left {
            Some(Box::new(Self::from(*node)))
        } else { None };

        let right = if let Some(node) = right {
            Some(Box::new(Self::from(*node)))
        } else { None };

        binary_tree::serialization::SerializationNode {
            data: RedBlackNode::new_with_color(key, value, color),
            left,
            right,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializationTree<K, V>(pub Option<SerializationNode<K, V>>);

impl<'t, K, V> SerializationTree<&'t K, &'t V> {
    pub fn new(tree: &'t RedBlackTree<K, V>) -> Self {
        Self::from(binary_tree::serialization::SerializationTree::new(tree.inner()))
    }
}

impl<'t, K, V> From<binary_tree::serialization::SerializationTree<&'t RedBlackNode<K, V>>> for SerializationTree<&'t K, &'t V> {
    fn from(value: binary_tree::serialization::SerializationTree<&'t RedBlackNode<K, V>>) -> Self {
        let binary_tree::serialization::SerializationTree(root) = value;
        Self(root.map(Into::into))
    }
}
