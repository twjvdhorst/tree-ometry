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

impl<K, V, T> BinaryTreeNode for BSTNode<K, V, T> {
    type Tree = T;

    fn left_subtree(&self) -> &T {
        &self.left_subtree
    }

    fn right_subtree(&self) -> &T {
        &self.right_subtree
    }

    fn left_subtree_mut(&mut self) -> &mut T {
        &mut self.left_subtree
    }

    fn right_subtree_mut(&mut self) -> &mut T {
        &mut self.right_subtree
    }
}

impl<K, V, T> BinarySearchTreeNode for BSTNode<K, V, T>
where 
    K: Ord,
{
    type Key = K;
    type Value = V;

    fn key(&self) -> &K {
        &self.key
    }

    fn value(&self) -> &V {
        &self.value
    }

    fn value_mut(&mut self) -> &mut Self::Value {
        &mut self.value
    }
}

impl<K, V, T> BSTNode<K, V, T>
where 
    T: BinaryTree,
{
    pub fn has_left(&self) -> bool {
        !self.left_subtree().is_leaf()
    }

    pub fn has_right(&self) -> bool {
        !self.right_subtree().is_leaf()
    }

    pub fn has_child(&self, side: Side) -> bool {
        match side {
            Side::Left => self.has_left(),
            Side::Right => self.has_right(),
        }
    }

    pub fn attach_left(&mut self, tree: impl Into<T>) -> bool {
        if !self.has_left() {
            self.left_subtree = tree.into();
            true
        } else { false }
    }
    
    pub fn attach_right(&mut self, tree: impl Into<T>) -> bool {
        if !self.has_right() {
            self.right_subtree = tree.into();
            true
        } else { false }
    }

    pub fn attach_subtree(&mut self, side: Side, tree: impl Into<T>) -> bool {
        match side {
            Side::Left => self.attach_left(tree),
            Side::Right => self.attach_right(tree),
        }
    }
    
    pub fn detach_left(&mut self) -> T {
        std::mem::replace(&mut self.left_subtree, T::new_leaf().into())
    }
    
    pub fn detach_right(&mut self) -> T {
        std::mem::replace(&mut self.right_subtree, T::new_leaf().into())
    }

    pub fn detach_subtree(&mut self, side: Side) -> T {
        match side {
            Side::Left => self.detach_left(),
            Side::Right => self.detach_right(),
        }
    }
    
    pub fn replace_left(&mut self, tree: impl Into<T>) -> T {
        std::mem::replace(&mut self.left_subtree, tree.into())
    }
    
    pub fn replace_right(&mut self, tree: impl Into<T>) -> T {
        std::mem::replace(&mut self.right_subtree, tree.into())
    }

    pub fn replace_subtree(&mut self, side: Side, tree: impl Into<T>) -> T {
        match side {
            Side::Left => self.replace_left(tree),
            Side::Right => self.replace_right(tree),
        }
    }
}
