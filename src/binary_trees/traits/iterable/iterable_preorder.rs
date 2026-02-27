use crate::binary_trees::{
    traits::{
        BinaryTree,
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
    fn preorder_iter(&self) -> PreorderIter<'_, Self, impl Fn(&Self) -> bool> {
        self.preorder_iter_filtered(|_| true)
    }

    fn preorder_iter_filtered<F>(&self, f: F) -> PreorderIter<'_, Self, F>
    where
        F: Fn(&Self) -> bool,
    {
        PreorderIter::new(self, f)
    }
}

pub(crate) trait IterablePreorderMut: IterablePreorder + Sized
where
    Self::Node: BinaryTreeNodeMut<Tree = Self>,
{
    fn preorder_iter_mut(&mut self) -> PreorderIterMut<'_, Self, impl Fn(&Self) -> bool> {
        self.preorder_iter_filtered_mut(|_| true)
    }

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
    T: BinaryTree<Node: BinaryTreeNodeMut<Tree = T>>,
{}
