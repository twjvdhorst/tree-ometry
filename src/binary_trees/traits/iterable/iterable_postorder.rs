use crate::binary_trees::{
    traits::{
        BinaryTree,
        BinaryTreeNode,
        BinaryTreeNodeMut,
    },
    tree_iterators::{
        postorder::{PostorderIter, PostorderIterMut},
    },
};

pub trait IterablePostorder: BinaryTree
where 
    Self::Node: BinaryTreeNode<Tree = Self>,
{
    /// Returns an iterator yielding references to all subtrees in the postorder order of their roots.
    fn postorder_iter(&self) -> PostorderIter<'_, Self, impl Fn(&Self) -> bool> {
        self.postorder_iter_filtered(|_| true)
    }

    /// Returns an iterator yielding references to subtrees in the postorder order of their roots,
    /// filtering subtrees with the supplied filter function.
    /// If a subtree is filtered out, none of the subtrees contained in it are reported either.
    fn postorder_iter_filtered<F>(&self, f: F) -> PostorderIter<'_, Self, F>
    where
        F: Fn(&Self) -> bool,
    {
        PostorderIter::new(self, f)
    }
}

pub(crate) trait IterablePostorderMut: IterablePostorder + Sized
where
    Self::Node: BinaryTreeNodeMut<Tree = Self>,
{
    /// Returns an iterator yielding mutable references to all subtrees in the postorder order of their roots.
    fn postorder_iter_mut(&mut self) -> PostorderIterMut<'_, Self, impl Fn(&Self) -> bool> {
        self.postorder_iter_filtered_mut(|_| true)
    }

    /// Returns an iterator yielding mutable references to all subtrees in the postorder order of their roots,
    /// filtering subtrees with the supplied filter function.
    /// If a subtree is filtered out, none of the subtrees contained in it are reported either.
    fn postorder_iter_filtered_mut<F>(&mut self, f: F) -> PostorderIterMut<'_, Self, F>
    where
        F: Fn(&Self) -> bool,
    {
        PostorderIterMut::new(self, f)
    }
}

impl<T> IterablePostorder for T
where 
    T: BinaryTree<Node: BinaryTreeNode<Tree = T>> + ?Sized,
{}

impl<T> IterablePostorderMut for T
where 
    T: BinaryTree<Node: BinaryTreeNodeMut<Tree = T>>,
{}
