use crate::binary_trees::{
    traits::{
        BinaryTree,
        BinaryTreeNode,
        BinaryTreeNodeMut,
    },
    tree_iterators::{
        inorder::{InorderIter, InorderIterMut},
    },
};

pub trait IterableInorder: BinaryTree
where 
    Self::Node: BinaryTreeNode<Tree = Self>,
{
    fn inorder_iter(&self) -> InorderIter<'_, Self, impl Fn(&Self) -> bool> {
        self.inorder_iter_filtered(|_| true)
    }

    fn inorder_iter_filtered<F>(&self, f: F) -> InorderIter<'_, Self, F>
    where
        F: Fn(&Self) -> bool,
    {
        InorderIter::new(self, f)
    }
}

pub(crate) trait IterableInorderMut: IterableInorder + Sized
where
    Self::Node: BinaryTreeNodeMut<Tree = Self>,
{
    fn inorder_iter_mut(&mut self) -> InorderIterMut<'_, Self, impl Fn(&Self) -> bool> {
        self.inorder_iter_filtered_mut(|_| true)
    }

    fn inorder_iter_filtered_mut<F>(&mut self, f: F) -> InorderIterMut<'_, Self, F>
    where
        F: Fn(&Self) -> bool,
    {
        InorderIterMut::new(self, f)
    }
}

impl<T> IterableInorder for T
where 
    T: BinaryTree<Node: BinaryTreeNode<Tree = T>> + ?Sized,
{}

impl<T> IterableInorderMut for T
where 
    T: BinaryTree<Node: BinaryTreeNodeMut<Tree = T>>,
{}
