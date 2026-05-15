use serde::Serialize;

use crate::binary_trees::semigroup_rb_tree;
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

impl<'t, K, V> From<semigroup_rb_tree::serialization::Node<'t, K, V, ()>> for Node<'t, K, V> {
    fn from(value: semigroup_rb_tree::serialization::Node<'t, K, V, ()>) -> Self {
        let semigroup_rb_tree::serialization::Node {
            key,
            value,
            left,
            right,
            ..
        } = value;
        let left = left.map(|node| Self::from(*node));
        let right = right.map(|node| Self::from(*node));
        Self {
            key,
            value,
            left: left.map(Into::into),
            right: right.map(Into::into),
        }
    }
}

#[derive(Serialize)]
pub struct Tree<'t, K, V>(pub Option<Node<'t, K, V>>);

impl<'t, K, V> Tree<'t, K, V> {
    pub fn new(tree: &'t RedBlackTree<K, V>) -> Self {
        Self::from(semigroup_rb_tree::serialization::Tree::new(tree.inner()))
    }
}

impl<'t, K, V> From<semigroup_rb_tree::serialization::Tree<'t, K, V, ()>> for Tree<'t, K, V> {
    fn from(value: semigroup_rb_tree::serialization::Tree<'t, K, V, ()>) -> Self {
        let semigroup_rb_tree::serialization::Tree(root) = value;
        Self(root.map(Into::into))
    }
}
