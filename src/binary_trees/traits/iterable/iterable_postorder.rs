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
    fn postorder_iter(&self) -> PostorderIter<'_, Self, impl Fn(&Self) -> bool> {
        self.postorder_iter_filtered(|_| true)
    }

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
    fn postorder_iter_mut(&mut self) -> PostorderIterMut<'_, Self, impl Fn(&Self) -> bool> {
        self.postorder_iter_filtered_mut(|_| true)
    }

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
