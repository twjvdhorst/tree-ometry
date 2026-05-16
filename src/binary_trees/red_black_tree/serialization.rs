use serde::{Deserialize, Serialize};

use crate::binary_trees::semigroup_rb_tree::{self, Color, SemigroupRbTree};
use crate::binary_trees::red_black_tree::RedBlackTree;

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
        let tree = SemigroupRbTree::deserialize(deserializer)?;
        Ok(Self::from(tree))
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializationNode<K, V> {
    pub(crate) key: K,
    pub(crate) value: V,
    pub(crate) color: Color,
    pub(crate) left: Option<Box<SerializationNode<K, V>>>,
    pub(crate) right: Option<Box<SerializationNode<K, V>>>,
}

impl<'t, K, V> From<semigroup_rb_tree::serialization::SerializationNode<&'t K, &'t V, &'t ()>> for SerializationNode<&'t K, &'t V> {
    fn from(value: semigroup_rb_tree::serialization::SerializationNode<&'t K, &'t V, &'t ()>) -> Self {
        let semigroup_rb_tree::serialization::SerializationNode {
            key,
            value,
            color,
            left,
            right,
            ..
        } = value;
        let left = left.map(|node| Self::from(*node));
        let right = right.map(|node| Self::from(*node));
        Self {
            key,
            value,
            color,
            left: left.map(Into::into),
            right: right.map(Into::into),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializationTree<K, V>(pub Option<SerializationNode<K, V>>);

impl<'t, K, V> SerializationTree<&'t K, &'t V> {
    pub fn new(tree: &'t RedBlackTree<K, V>) -> Self {
        Self::from(semigroup_rb_tree::serialization::SerializationTree::new(tree.inner()))
    }
}

impl<'t, K, V> From<semigroup_rb_tree::serialization::SerializationTree<&'t K, &'t V, &'t ()>> for SerializationTree<&'t K, &'t V> {
    fn from(value: semigroup_rb_tree::serialization::SerializationTree<&'t K, &'t V, &'t ()>) -> Self {
        let semigroup_rb_tree::serialization::SerializationTree(root) = value;
        Self(root.map(Into::into))
    }
}
