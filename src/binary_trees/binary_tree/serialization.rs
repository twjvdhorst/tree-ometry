use serde::Serialize;

use super::{BinaryTreeNode, BinaryTree};

impl<T> Serialize for BinaryTree<T>
where 
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        Tree::new(self).serialize(serializer)
    }
}

#[derive(Serialize)]
pub struct Node<'t, T> {
    pub data: &'t T,
    pub left: Option<Box<Node<'t, T>>>,
    pub right: Option<Box<Node<'t, T>>>,
}

impl<'t, T> Node<'t, T> {
    fn new(tree: &'t BinaryTree<T>, node: &'t BinaryTreeNode<T>) -> Self {
        let left = if let Some(left) = tree.left_child(node) {
            Some(Box::new(Self::new(tree, left)))
        } else {
            None
        };

        let right = if let Some(right) = tree.right_child(node) {
            Some(Box::new(Self::new(tree, right)))
        } else {
            None
        };

        Self {
            data: node.data(),
            left,
            right,
        }
    }
}

#[derive(Serialize)]
pub struct Tree<'t, T>(pub Option<Node<'t, T>>);

impl<'t, T> Tree<'t, T> {
    pub fn new(tree: &'t BinaryTree<T>) -> Self {
        if let Some(root) = tree.root() {
            Self(Some(Node::new(tree, root)))
        } else {
            return Self(None);
        }
    }
}
