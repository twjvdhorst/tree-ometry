use serde::{Serialize, Deserialize, de::Error};

use crate::binary_trees::binary_tree::{self, BinaryTree};
use crate::binary_trees::semigroup_rb_tree::{Color, SemigroupRbNode, SemigroupRbTree, TreeSemigroup};

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
        let tree = BinaryTree::from(SerializationTree::deserialize(deserializer)?);
        Self::try_from(tree).map_err(D::Error::custom)
    }
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

impl<K, V, S> From<SerializationTree<K, V, S>> for BinaryTree<SemigroupRbNode<K, V, S>> {
    fn from(value: SerializationTree<K, V, S>) -> Self {
        Self::from(binary_tree::serialization::SerializationTree(value.0.map(binary_tree::serialization::SerializationNode::from)))
    }
}
