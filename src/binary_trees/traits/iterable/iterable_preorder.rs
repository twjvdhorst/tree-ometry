use crate::binary_trees::{
    traits::{BinaryTree, BinaryTreeMut},
    tree_iterators::{PreorderIter, PreorderIterMut},
};

pub trait IterablePreorder: BinaryTree {
    /// Returns an inorder iterator over references to the nodes of the tree.
    fn preorder_iter(&self) -> PreorderIter<'_, Self, impl Fn(&Self::Node) -> bool> {
        self.preorder_iter_filtered(|_| true)
    }

    /// Returns an inorder iterator over references to the nodes of the tree, filtering subtrees with the supplied filter function.
    /// If a subtree is filtered out, none of the nodes contained in it are reported.
    fn preorder_iter_filtered<F>(&self, f: F) -> PreorderIter<'_, Self, F>
    where
        F: Fn(&Self::Node) -> bool,
    {
        PreorderIter::new(self, f)
    }
}

pub trait IterablePreorderMut: IterablePreorder + BinaryTreeMut {
    /// Returns an inorder iterator over mutable references to the nodes of the tree.
    fn preorder_iter_mut(&mut self) -> PreorderIterMut<'_, Self, impl Fn(&Self::Node) -> bool> {
        self.preorder_iter_filtered_mut(|_| true)
    }

    /// Returns an inorder iterator over mutable references to the nodes of the tree, filtering subtrees with the supplied filter function.
    /// If a subtree is filtered out, none of the nodes contained in it are reported.
    fn preorder_iter_filtered_mut<F>(&mut self, f: F) -> PreorderIterMut<'_, Self, F>
    where
        F: Fn(&Self::Node) -> bool,
    {
        PreorderIterMut::new(self, f)
    }
}

impl<T> IterablePreorder for T
where 
    T: BinaryTree + ?Sized,
{}

impl<T> IterablePreorderMut for T
where 
    T: BinaryTreeMut + ?Sized,
{}
