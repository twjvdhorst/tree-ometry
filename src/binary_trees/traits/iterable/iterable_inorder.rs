use crate::binary_trees::{
    traits::{BinaryTree, BinaryTreeMut},
    tree_iterators::{InorderIter, InorderIterMut},
};

pub trait IterableInorder: BinaryTree {
    /// Returns an inorder iterator over references to the nodes of the tree.
    fn inorder_iter(&self) -> InorderIter<'_, Self, impl Fn(&Self::Node) -> bool> {
        self.inorder_iter_filtered(|_| true)
    }

    /// Returns an inorder iterator over references to the nodes of the tree, filtering subtrees with the supplied filter function.
    /// If a subtree is filtered out, none of the nodes contained in it are reported.
    fn inorder_iter_filtered<F>(&self, f: F) -> InorderIter<'_, Self, F>
    where
        F: Fn(&Self::Node) -> bool,
    {
        InorderIter::new(self, f)
    }
}

pub trait IterableInorderMut: IterableInorder + BinaryTreeMut {
    /// Returns an inorder iterator over mutable references to the nodes of the tree.
    fn inorder_iter_mut(&mut self) -> InorderIterMut<'_, Self, impl Fn(&Self::Node) -> bool> {
        self.inorder_iter_filtered_mut(|_| true)
    }

    /// Returns an inorder iterator over mutable references to the nodes of the tree, filtering subtrees with the supplied filter function.
    /// If a subtree is filtered out, none of the nodes contained in it are reported.
    fn inorder_iter_filtered_mut<F>(&mut self, f: F) -> InorderIterMut<'_, Self, F>
    where
        F: Fn(&Self::Node) -> bool,
    {
        InorderIterMut::new(self, f)
    }
}

impl<T> IterableInorder for T
where 
    T: BinaryTree + ?Sized,
{}

impl<T> IterableInorderMut for T
where 
    T: BinaryTreeMut + ?Sized,
{}
