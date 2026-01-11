use crate::binary_search_trees::node_traits::{BinarySearchTreeNode, BinaryTree, BinaryTreeNode};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct BSTNode<K, V, T> {
    key: K,
    value: V,
    left_subtree: T,
    right_subtree: T,
}

impl<K, V, T> BSTNode<K, V, T>
where 
    T: BinaryTree,
{
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            left_subtree: T::new_leaf().into(),
            right_subtree: T::new_leaf().into(),
        }
    }
}

impl<K, V, T> BinaryTreeNode for BSTNode<K, V, T>
where 
    T: BinaryTree,
{
    type Tree = T;

    fn left_subtree(&self) -> &Self::Tree {
        &self.left_subtree
    }

    fn right_subtree(&self) -> &Self::Tree {
        &self.right_subtree
    }

    fn left_subtree_mut(&mut self) -> &mut Self::Tree {
        &mut self.left_subtree
    }

    fn right_subtree_mut(&mut self) -> &mut Self::Tree {
        &mut self.right_subtree
    }

    fn attach_left(&mut self, tree: impl Into<Self::Tree>) -> bool {
        if !self.has_left() {
            self.left_subtree = tree.into();
            true
        } else { false }
    }
    
    fn attach_right(&mut self, tree: impl Into<Self::Tree>) -> bool {
        if !self.has_right() {
            self.right_subtree = tree.into();
            true
        } else { false }
    }
    
    fn detach_left(&mut self) -> Self::Tree {
        std::mem::replace(&mut self.left_subtree, T::new_leaf().into())
    }
    
    fn detach_right(&mut self) -> Self::Tree {
        std::mem::replace(&mut self.right_subtree, T::new_leaf().into())
    }
    
    fn replace_left(&mut self, tree: impl Into<Self::Tree>) -> Self::Tree {
        std::mem::replace(&mut self.left_subtree, tree.into())
    }
    
    fn replace_right(&mut self, tree: impl Into<Self::Tree>) -> Self::Tree {
        std::mem::replace(&mut self.right_subtree, tree.into())
    }
}

impl<K, V, T> BinarySearchTreeNode for BSTNode<K, V, T>
where 
    K: Ord,
{
    type Key = K;
    type Value = V;

    fn key(&self) -> &Self::Key {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }

    fn value_mut(&mut self) -> &mut Self::Value {
        &mut self.value
    }
}
