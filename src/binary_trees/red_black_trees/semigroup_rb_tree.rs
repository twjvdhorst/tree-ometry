use std::borrow::Borrow;

use lending_iterator::LendingIterator;
use paste::paste;

use crate::binary_trees::{binary_tree_traits::{BinaryTree, BinaryTreeNode}, red_black_trees::{red_black_node::RedBlackNode, semigroup::Semigroup}, tree_iterators::{inorder::{InorderIter, InorderIterMut}, postorder::{PostorderIter, PostorderIterMut}, preorder::{PreorderIter, PreorderIterMut}}};

pub struct SemigroupRbNode<K, V, S, T> {
    node: RedBlackNode<K, V, T>,
    semigroup_value: S,
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

    fn get_semigroup_value(&self) -> Option<&S> {
        Some(&self.0.as_ref()?.semigroup_value)
    }

    fn set_semigroup_value(&mut self, new_data: S) {
        if let Some(root) = self.0.as_mut() {
            root.semigroup_value = new_data
        }
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
            semigroup_value: S::default(), 
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
        let result = RedBlackNode::insert(self, key, value);
        self.update_semigroup_values();
        result
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
        let result = RedBlackNode::remove_entry(self, key);
        self.update_semigroup_values();
        result
    }

    /// Updates semigroup values in a bottom-up fashion.
    /// Considers only tree nodes that have been accessed mutably,
    /// as others have their subtree, and thus semigroup value, intact.
    fn update_semigroup_values(&mut self) {
        let mut changed_trees_iter = self.postorder_iter_mut_filtered(|tree|
            tree.0.as_ref().map(|root| root.accessed_mut) == Some(true)
        );
        while let Some(tree) = changed_trees_iter.next() {
            let Some(root) = tree.root() else { continue; };
            let (left, right) = root.subtrees();
            let new_semigroup_value = match (left.get_semigroup_value(), right.get_semigroup_value()) {
                (Some(x), Some(y)) => Semigroup::op(x, y),
                (Some(x), None) => Semigroup::op(x, &S::default()),
                (None, Some(y)) => Semigroup::op(&S::default(), y),
                (None, None) => S::default(),
            };
            tree.set_semigroup_value(new_semigroup_value);
        }
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

macro_rules! make_iter {
    ($vis: vis, $iter_name: ident, $iter_type: ident) => {
        paste!{
            $vis fn $iter_name(&mut self) -> $iter_type<'_, Self, impl Fn(&Self) -> bool> {
                self.[<$iter_name _filtered>](|_| true)
            }

            $vis fn [<$iter_name _filtered>]<F>(&'_ mut self, f: F) -> $iter_type<'_, Self, F>
            where
                F: Fn(&Self) -> bool,
            {
                $iter_type::new(self, f)
            }
        }
    };
}

impl<K, V, S> SemigroupRbTree<K, V, S>
where
    S: Default,
{
    make_iter!(pub, inorder_iter, InorderIter);
    make_iter!(pub(crate), inorder_iter_mut, InorderIterMut);
    make_iter!(pub, preorder_iter, PreorderIter);
    make_iter!(pub(crate), preorder_iter_mut, PreorderIterMut);
    make_iter!(pub, postorder_iter, PostorderIter);
    make_iter!(pub(crate), postorder_iter_mut, PostorderIterMut);
}
