use serde::Serialize;

use crate::binary_trees::binary_tree;
use crate::binary_trees::semigroup_rb_tree::{SemigroupRbNode, SemigroupRbTree};

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
        Tree::new(self).serialize(serializer)
    }
}

#[derive(Serialize)]
pub struct Node<'t, K, V, S> {
    pub key: &'t K,
    pub value: &'t V,
    pub semigroup_value: &'t S,
    pub left: Option<Box<Node<'t, K, V, S>>>,
    pub right: Option<Box<Node<'t, K, V, S>>>,
}

impl<'t, K, V, S> From<binary_tree::serialization::Node<'t, SemigroupRbNode<K, V, S>>> for Node<'t, K, V, S> {
    fn from(value: binary_tree::serialization::Node<'t, SemigroupRbNode<K, V, S>>) -> Self {
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
            semigroup_value: node.semigroup_value(),
            left: left.map(Into::into),
            right: right.map(Into::into),
        }
    }
}

#[derive(Serialize)]
pub struct Tree<'t, K, V, S>(pub Option<Node<'t, K, V, S>>);

impl<'t, K, V, S> Tree<'t, K, V, S> {
    pub fn new(tree: &'t SemigroupRbTree<K, V, S>) -> Self {
        Self::from(binary_tree::serialization::Tree::new(tree.inner()))
    }
}

impl<'t, K, V, S> From<binary_tree::serialization::Tree<'t, SemigroupRbNode<K, V, S>>> for Tree<'t, K, V, S> {
    fn from(value: binary_tree::serialization::Tree<'t, SemigroupRbNode<K, V, S>>) -> Self {
        let binary_tree::serialization::Tree(root) = value;
        Self(root.map(Into::into))
    }
}
