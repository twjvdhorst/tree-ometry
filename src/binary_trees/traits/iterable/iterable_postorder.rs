use crate::binary_trees::{
    traits::{BinaryTree, BinaryTreeMut},
    tree_iterators::{PostorderIter, PostorderIterMut},
};

pub trait IterablePostorder: BinaryTree {
    /// Returns an inorder iterator over references to the nodes of the tree.
    fn postorder_iter(&self) -> PostorderIter<'_, Self, impl Fn(&Self::Node) -> bool> {
        self.postorder_iter_filtered(|_| true)
    }

    /// Returns an inorder iterator over references to the nodes of the tree, filtering subtrees with the supplied filter function.
    /// If a subtree is filtered out, none of the nodes contained in it are reported.
    fn postorder_iter_filtered<F>(&self, f: F) -> PostorderIter<'_, Self, F>
    where
        F: Fn(&Self::Node) -> bool,
    {
        PostorderIter::new(self, f)
    }
}

pub trait IterablePostorderMut: IterablePostorder + BinaryTreeMut {
    /// Returns an inorder iterator over mutable references to the nodes of the tree.
    fn postorder_iter_mut(&mut self) -> PostorderIterMut<'_, Self, impl Fn(&Self::Node) -> bool> {
        self.postorder_iter_filtered_mut(|_| true)
    }

    /// Returns an inorder iterator over mutable references to the nodes of the tree, filtering subtrees with the supplied filter function.
    /// If a subtree is filtered out, none of the nodes contained in it are reported.
    fn postorder_iter_filtered_mut<F>(&mut self, f: F) -> PostorderIterMut<'_, Self, F>
    where
        F: Fn(&Self::Node) -> bool,
    {
        PostorderIterMut::new(self, f)
    }
}

impl<T> IterablePostorder for T
where 
    T: BinaryTree + ?Sized,
{}

impl<T> IterablePostorderMut for T
where 
    T: BinaryTreeMut + ?Sized,
{}
