use std::{cmp::Ordering, marker::PhantomData};

use serde::{Serialize, Deserialize, de::Error};
use thiserror::Error;

use super::{CartesianTree, CartesianNode, Comparer};
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

impl<K, V, C> Serialize for CartesianTree<K, V, C>
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

impl<'de, K, V, C> Deserialize<'de> for CartesianTree<K, V, C>
where 
    K: Ord + Deserialize<'de>,
    V: Deserialize<'de>,
    C: Comparer,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let tree = BinaryTree::from(SerializationTree::deserialize(deserializer)?);
        if !is_heap::<_, _, C>(&tree) {
            Err(CartesianTreeError::HeapError(String::from(C::name()))).map_err(D::Error::custom)?;
        }

        Ok(Self(tree, PhantomData))
    }
}

#[derive(Error, Debug)]
pub enum CartesianTreeError {
    #[error("tree is not a {0} heap")]
    HeapError(String),
}

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
            let mut left_cursor = cursor.clone();
            left_cursor.move_left();
            if !is_heap_recursive::<_, _, C>(left_cursor) {
                return false;
            };
        }
        if let Some(right) = cursor.peek_right() {
            if C::compare(node.key(), right.key()) == Ordering::Greater {
                return false;
            }
            let mut right_cursor = cursor.clone();
            right_cursor.move_right();
            if !is_heap_recursive::<_, _, C>(right_cursor) {
                return false;
            };
        }
        true
    }
        
    is_heap_recursive::<_, _, C>(tree.cursor())
}

#[derive(Serialize, Deserialize)]
pub struct SerializationNode<K, V> {
    key: K,
    value: V,
    left: Option<Box<SerializationNode<K, V>>>,
    right: Option<Box<SerializationNode<K, V>>>,
}

impl<'t, K, V> From<binary_tree::serialization::SerializationNode<&'t CartesianNode<K, V>>> for SerializationNode<&'t K, &'t V> {
    fn from(value: binary_tree::serialization::SerializationNode<&'t CartesianNode<K, V>>) -> Self {
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
            left: left.map(Into::into),
            right: right.map(Into::into),
        }
    }
}

impl<K, V> From<SerializationNode<K, V>> for binary_tree::serialization::SerializationNode<CartesianNode<K, V>> {
    fn from(value: SerializationNode<K, V>) -> Self {
        let SerializationNode { key, value, left, right } = value;
        let left = if let Some(node) = left {
            Some(Box::new(Self::from(*node)))
        } else { None };

        let right = if let Some(node) = right {
            Some(Box::new(Self::from(*node)))
        } else { None };

        binary_tree::serialization::SerializationNode {
            data: CartesianNode::new(key, value),
            left,
            right,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializationTree<K, V>(pub Option<SerializationNode<K, V>>);

impl<'t, K, V> SerializationTree<&'t K, &'t V> {
    pub fn new<C>(tree: &'t CartesianTree<K, V, C>) -> Self {
        Self::from(binary_tree::serialization::SerializationTree::new(tree.inner()))
    }
}

impl<'t, K, V> From<binary_tree::serialization::SerializationTree<&'t CartesianNode<K, V>>> for SerializationTree<&'t K, &'t V> {
    fn from(value: binary_tree::serialization::SerializationTree<&'t CartesianNode<K, V>>) -> Self {
        let binary_tree::serialization::SerializationTree(root) = value;
        Self(root.map(Into::into))
    }
}

impl<K, V> From<SerializationTree<K, V>> for BinaryTree<CartesianNode<K, V>> {
    fn from(value: SerializationTree<K, V>) -> Self {
        Self::from(binary_tree::serialization::SerializationTree(value.0.map(binary_tree::serialization::SerializationNode::from)))
    }
}
