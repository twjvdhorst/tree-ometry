use serde::Serialize;

use crate::binary_trees::binary_tree;
use crate::binary_trees::cartesian_tree::{CartesianTree, CartesianTreeNode};

impl<K, V, C> Serialize for CartesianTree<K, V, C>
where 
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        Tree::new(self).serialize(serializer)
    }
}

#[derive(Serialize)]
pub struct Node<'t, K, V> {
    pub key: &'t K,
    pub value: &'t V,
    pub left: Option<Box<Node<'t, K, V>>>,
    pub right: Option<Box<Node<'t, K, V>>>,
}

impl<'t, K, V> From<binary_tree::serialization::Node<'t, CartesianTreeNode<K, V>>> for Node<'t, K, V> {
    fn from(value: binary_tree::serialization::Node<'t, CartesianTreeNode<K, V>>) -> Self {
        let binary_tree::serialization::Node {
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

#[derive(Serialize)]
pub struct Tree<'t, K, V>(pub Option<Node<'t, K, V>>);

impl<'t, K, V> Tree<'t, K, V> {
    pub fn new<C>(tree: &'t CartesianTree<K, V, C>) -> Self {
        Self::from(binary_tree::serialization::Tree::new(tree.inner()))
    }
}

impl<'t, K, V> From<binary_tree::serialization::Tree<'t, CartesianTreeNode<K, V>>> for Tree<'t, K, V> {
    fn from(value: binary_tree::serialization::Tree<'t, CartesianTreeNode<K, V>>) -> Self {
        let binary_tree::serialization::Tree(root) = value;
        Self(root.map(Into::into))
    }
}
