use crate::binary_trees::{
    traits::{
        BinaryTree,
        BinaryTreeMut,
        BinaryTreeNode,
        BinaryTreeNodeMut,
    },
    tree_iterators::{
        preorder::{PreorderIter, PreorderIterMut},
    },
};

pub trait IterablePreorder: BinaryTree
where 
    Self::Node: BinaryTreeNode<Tree = Self>,
{
    /// Returns an iterator yielding references to all subtrees in the preorder order of their roots.
    fn preorder_iter(&self) -> PreorderIter<'_, Self, impl Fn(&Self) -> bool> {
        self.preorder_iter_filtered(|_| true)
    }

    /// Returns an iterator yielding references to subtrees in the preorder order of their roots,
    /// filtering subtrees with the supplied filter function.
    /// If a subtree is filtered out, none of the subtrees contained in it are reported either.
    fn preorder_iter_filtered<F>(&self, f: F) -> PreorderIter<'_, Self, F>
    where
        F: Fn(&Self) -> bool,
    {
        PreorderIter::new(self, f)
    }
}

pub(crate) trait IterablePreorderMut: IterablePreorder + BinaryTreeMut + Sized
where
    Self::Node: BinaryTreeNodeMut<Tree = Self>,
{
    /// Returns an iterator yielding mutable references to all subtrees in the preorder order of their roots.
    fn preorder_iter_mut(&mut self) -> PreorderIterMut<'_, Self, impl Fn(&Self) -> bool> {
        self.preorder_iter_filtered_mut(|_| true)
    }

    /// Returns an iterator yielding mutable references to all subtrees in the preorder order of their roots,
    /// filtering subtrees with the supplied filter function.
    /// If a subtree is filtered out, none of the subtrees contained in it are reported either.
    fn preorder_iter_filtered_mut<F>(&mut self, f: F) -> PreorderIterMut<'_, Self, F>
    where
        F: Fn(&Self) -> bool,
    {
        PreorderIterMut::new(self, f)
    }
}

impl<T> IterablePreorder for T
where 
    T: BinaryTree<Node: BinaryTreeNode<Tree = T>> + ?Sized,
{}

impl<T> IterablePreorderMut for T
where 
    T: BinaryTreeMut<Node: BinaryTreeNodeMut<Tree = T>>,
{}
