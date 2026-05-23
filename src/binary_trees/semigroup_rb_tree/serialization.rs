use serde::{Serialize, Deserialize, de::Error};
use thiserror::Error;

use crate::binary_trees::{
    Neighborhood,
    binary_tree::{
        self, 
        BinaryTree,
    },
    semigroup_rb_tree::{
        Color,
        SemigroupRbNode, 
        SemigroupRbTree, 
        TreeSemigroup
    },
    binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursor,
    },
};

impl<K, V, SG> Serialize for SemigroupRbTree<K, V, SG>
where 
    K: Serialize,
    V: Serialize,
    SG: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        SerializationTree::new(self).serialize(serializer)
    }
}

impl<'de, K, V, S> Deserialize<'de> for SemigroupRbTree<K, V, S>
where 
    K: Ord + Clone + Deserialize<'de>,
    V: Deserialize<'de>,
    S: TreeSemigroup<K> + PartialEq + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        // Serialize into a BinaryTree<SemigroupRbNode<K, V, S>>, and try to convert that to a SemigroupRbTree<K, V, S>.
        let tree = SerializationTree::deserialize(deserializer)?;
        let tree = binary_tree::serialization::SerializationTree(tree.0.map(binary_tree::serialization::SerializationNode::from));
        let tree = BinaryTree::from(tree);

        if !is_binary_search_tree(&tree) {
            Err(SemigroupRbTreeError::BinarySearchTreeError).map_err(D::Error::custom)?;
        }
        if !is_red_black_tree(&tree) {
            Err(SemigroupRbTreeError::RedBlackTreeError).map_err(D::Error::custom)?;
        }
        if !check_semigroup_values(&tree) {
            Err(SemigroupRbTreeError::SemigroupError).map_err(D::Error::custom)?;
        }
        Ok(Self(tree))
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
        let Some(node) = cursor.get() else { return (true, Some(1)); };

        // No red-red edge.
        let left = cursor.peek_left();
        let right = cursor.peek_right();
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
    if let Some(node) = cursor.get() && !node.is_black() {
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
        let Some(node) = cursor.get() else { return true; };
        let Neighborhood { left, right, .. } = cursor.peek_neighborhood();
        let semigroup_value = S::op(node.key(), left.map(SemigroupRbNode::semigroup_value), right.map(SemigroupRbNode::semigroup_value));
        if *node.semigroup_value() != semigroup_value {
            return false;
        }
            
        let mut left_cursor = cursor;
        let mut right_cursor = cursor.clone();
        left_cursor.move_left();
        right_cursor.move_right();
        check_semigroup_values_recursive(left_cursor) && check_semigroup_values_recursive(right_cursor)
    }

    check_semigroup_values_recursive(tree.cursor())
}

#[derive(Serialize, Deserialize)]
pub struct SerializationNode<K, V, S> {
    pub(in super::super) key: K,
    pub(in super::super) value: V,
    pub(in super::super) semigroup_value: S,
    pub(in super::super) color: Color,
    pub(in super::super) left: Option<Box<SerializationNode<K, V, S>>>,
    pub(in super::super) right: Option<Box<SerializationNode<K, V, S>>>,
}

impl<'t, K, V, S> From<binary_tree::serialization::SerializationNode<&'t SemigroupRbNode<K, V, S>>> for SerializationNode<&'t K, &'t V, &'t S> {
    fn from(value: binary_tree::serialization::SerializationNode<&'t SemigroupRbNode<K, V, S>>) -> Self {
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
            semigroup_value: node.semigroup_value(),
            color: node.color(),
            left: left.map(Into::into),
            right: right.map(Into::into),
        }
    }
}

impl<K, V, S> From<SerializationNode<K, V, S>> for binary_tree::serialization::SerializationNode<SemigroupRbNode<K, V, S>> {
    fn from(value: SerializationNode<K, V, S>) -> Self {
        let SerializationNode { key, value, semigroup_value, color, left, right } = value;
        let left = if let Some(node) = left {
            Some(Box::new(Self::from(*node)))
        } else { None };

        let right = if let Some(node) = right {
            Some(Box::new(Self::from(*node)))
        } else { None };

        binary_tree::serialization::SerializationNode {
            data: SemigroupRbNode::new_with_color_and_semigroup_value(key, value, semigroup_value, color),
            left,
            right,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializationTree<K, V, S>(pub Option<SerializationNode<K, V, S>>);

impl<'t, K, V, S> SerializationTree<&'t K, &'t V, &'t S> {
    pub fn new(tree: &'t SemigroupRbTree<K, V, S>) -> Self {
        Self::from(binary_tree::serialization::SerializationTree::new(tree.inner()))
    }
}

impl<'t, K, V, S> From<binary_tree::serialization::SerializationTree<&'t SemigroupRbNode<K, V, S>>> for SerializationTree<&'t K, &'t V, &'t S> {
    fn from(value: binary_tree::serialization::SerializationTree<&'t SemigroupRbNode<K, V, S>>) -> Self {
        let binary_tree::serialization::SerializationTree(root) = value;
        Self(root.map(Into::into))
    }
}
