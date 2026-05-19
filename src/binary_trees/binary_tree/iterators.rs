use paste::paste;
use slotmap::Key;

use super::BinaryTree;
use crate::binary_trees::{binary_tree::NodeId, tree_iterators::{self, *}};

impl<T> BinaryTree<T> {
    tree_iterators::impl_iters!(pub, inorder, T);

    pub fn into_inorder_iter(self) -> IntoInorderIter<T> {
        IntoInorderIter::new(self)
    }

    tree_iterators::impl_iters!(pub, preorder, T);

    pub fn into_preorder_iter(self) -> IntoPreorderIter<T> {
        IntoPreorderIter::new(self)
    }

    tree_iterators::impl_iters!(pub, postorder, T);

    pub fn into_postorder_iter(self) -> IntoPostorderIter<T> {
        IntoPostorderIter::new(self)
    }
}

pub struct IntoInorderIter<T> {
    tree: BinaryTree<T>,
    stack: Vec<NodeId>,
}

impl<T> IntoInorderIter<T> {
    fn new(tree: BinaryTree<T>) -> Self {
        // Move the "cursor" to the first node in the inorder order.
        let mut id = tree.root_id();
        let mut stack = Vec::new();
        while !id.is_null() {
            stack.push(id);
            id = tree.left_id(id).unwrap_or(NodeId::null());
        }

        Self {
            tree,
            stack,
        }
    }
}

impl<T> Iterator for IntoInorderIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Get id of the to-be-reported element, and expand stack.
        let next_id = self.stack.pop()?;
        if let Some(id) = self.tree.right_id(next_id) {
            self.stack.push(id);
            while let Some(id) = self.tree.left_id(*self.stack.last().unwrap()) {
                self.stack.push(id);
            }
        }
        self.tree.remove_node(next_id)
    }
}

pub struct IntoPreorderIter<T> {
    tree: BinaryTree<T>,
    stack: Vec<NodeId>,
}

impl<T> IntoPreorderIter<T> {
    fn new(tree: BinaryTree<T>) -> Self {
        let id = tree.root_id();
        let mut stack = Vec::new();
        if !id.is_null() {
            stack.push(id);
        }

        Self {
            tree,
            stack,
        }
    }
}

impl<T> Iterator for IntoPreorderIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Get id of the to-be-reported element, and expand stack.
        let next_id = self.stack.pop()?;
        if let Some(right_id) = self.tree.right_id(next_id) {
            self.stack.push(right_id);
        }
        if let Some(left_id) = self.tree.left_id(next_id) {
            self.stack.push(left_id);
        }
        self.tree.remove_node(next_id)
    }
}

pub struct IntoPostorderIter<T> {
    tree: BinaryTree<T>,
    stack: Vec<NodeId>,
}

impl<T> IntoPostorderIter<T> {
    fn new(tree: BinaryTree<T>) -> Self {
        // Move the "cursor" to the first node in the postorder order.
        let mut id = tree.root_id();
        let mut stack = Vec::new();
        while !id.is_null() {
            stack.push(id);
            id = tree.left_id(id).unwrap_or(NodeId::null());
        }

        Self {
            tree,
            stack,
        }
    }
}

impl<T> Iterator for IntoPostorderIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Expand the stack first, then report the element.
        let id = self.stack.last()?;
        if let Some(right_id) = self.tree.right_id(*id) {
            self.stack.push(right_id);
            while let Some(left_id) = self.tree.left_id(*self.stack.last().unwrap()) {
                self.stack.push(left_id);
            }
        }
        let next_id = self.stack.pop().unwrap();
        self.tree.remove_node(next_id)
    }
}
