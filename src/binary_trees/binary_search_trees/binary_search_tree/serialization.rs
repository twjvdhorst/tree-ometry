use serde::{Serialize, Deserialize, de::Error};

use super::{BinarySearchTree, BstNode};
use crate::binary_trees::{
    binary_tree::{
        self,
        BinaryTree,
    },
};

impl<K, V> Serialize for BinarySearchTree<K, V>
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

impl<'de, K, V> Deserialize<'de> for BinarySearchTree<K, V>
where 
    K: Ord + Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let tree = BinaryTree::from(SerializationTree::deserialize(deserializer)?);
        tree.try_into().map_err(D::Error::custom)
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializationNode<K, V> {
    key: K,
    value: V,
    left: Option<Box<SerializationNode<K, V>>>,
    right: Option<Box<SerializationNode<K, V>>>,
}

impl<'t, K, V> From<binary_tree::serialization::SerializationNode<&'t BstNode<K, V>>> for SerializationNode<&'t K, &'t V> {
    fn from(value: binary_tree::serialization::SerializationNode<&'t BstNode<K, V>>) -> Self {
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

impl<K, V> From<SerializationNode<K, V>> for binary_tree::serialization::SerializationNode<BstNode<K, V>> {
    fn from(value: SerializationNode<K, V>) -> Self {
        let SerializationNode { key, value, left, right } = value;
        let left = if let Some(node) = left {
            Some(Box::new(Self::from(*node)))
        } else { None };

        let right = if let Some(node) = right {
            Some(Box::new(Self::from(*node)))
        } else { None };

        binary_tree::serialization::SerializationNode {
            data: BstNode::new(key, value),
            left,
            right,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializationTree<K, V>(pub Option<SerializationNode<K, V>>);

impl<'t, K, V> SerializationTree<&'t K, &'t V> {
    pub fn new(tree: &'t BinarySearchTree<K, V>) -> Self {
        Self::from(binary_tree::serialization::SerializationTree::new(&tree.0))
    }
}

impl<'t, K, V> From<binary_tree::serialization::SerializationTree<&'t BstNode<K, V>>> for SerializationTree<&'t K, &'t V> {
    fn from(value: binary_tree::serialization::SerializationTree<&'t BstNode<K, V>>) -> Self {
        let binary_tree::serialization::SerializationTree(root) = value;
        Self(root.map(Into::into))
    }
}

impl<K, V> From<SerializationTree<K, V>> for BinaryTree<BstNode<K, V>> {
    fn from(value: SerializationTree<K, V>) -> Self {
        Self::from(binary_tree::serialization::SerializationTree(value.0.map(binary_tree::serialization::SerializationNode::from)))
    }
}
