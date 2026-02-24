use std::borrow::Borrow;

use semigroup::Semigroup;

use crate::binary_trees::{red_black_trees::red_black_node::RedBlackNode, binary_tree_traits::BinaryTree};

pub struct SemigroupRbNode<K, V, S, T> {
    node: RedBlackNode<K, V, T>,
    semigroup_data: S,
    accessed_mut: bool,
}

pub struct SemigroupRbTree<K, V, S>(Option<SemigroupRbNode<K, V, S, Self>>);

impl<K, V, S> Default for SemigroupRbTree<K, V, S>
where 
    S: Default,
{
    fn default() -> Self {
        Self::new_leaf()
    }
}

impl<K, V, S> SemigroupRbTree<K, V, S>
where 
    S: Default,
{
    pub fn new() -> Self {
        Self::new_leaf()
    }
}

impl<K, V, S> Extend<(K, V)> for SemigroupRbTree<K, V, S>
where 
    K: Ord,
    S: Default + Semigroup,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            self.insert(key, value);
        }
    }
}

impl<K, V, S> FromIterator<(K, V)> for SemigroupRbTree<K, V, S>
where 
    K: Ord,
    S: Default + Semigroup,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut tree = Self::default();
        tree.extend(iter);
        tree
    }
}

impl<K, V, S> BinaryTree for SemigroupRbTree<K, V, S>
where 
    S: Default,
{
    type Node = RedBlackNode<K, V, Self>;

    fn new_leaf() -> Self {
        Self(None)
    }

    fn new_node(node: Self::Node) -> Self {
        Self(Some(SemigroupRbNode {
            node, 
            semigroup_data: S::default(), 
            accessed_mut: false
        }))
    }

    fn is_leaf(&self) -> bool {
        self.0.is_none()
    }

    fn root(&self) -> Option<&Self::Node> {
        self.0.as_ref().map(|root| &root.node)
    }

    fn root_mut(&mut self) -> Option<&mut Self::Node> {
        let root = self.0.as_mut()?;
        root.accessed_mut = true;
        Some(&mut root.node)
    }

    fn into_root(self) -> Option<Self::Node> {
        self.0.map(|root| root.node)
    }
}

impl<K, V, S> SemigroupRbTree<K, V, S>
where
    K: Ord,
    S: Default + Semigroup,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        RedBlackNode::insert(self, key, value)
        // TODO: Update semigroup values
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.remove_entry(key).map(|(_, v)| v)
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        RedBlackNode::remove_entry(self, key)
        // TODO
    }
}
