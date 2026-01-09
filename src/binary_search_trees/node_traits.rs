use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub trait BinaryTree {
    type Node;

    fn new(root: Self::Node) -> Self;
    fn new_leaf() -> Self;
    fn root(&self) -> Option<&Self::Node>;
    fn is_leaf(&self) -> bool;
}

pub trait BinaryTreeMut: BinaryTree {
    fn root_mut(&mut self) -> Option<&mut Self::Node>;
}

pub trait BinaryTreeNode {
    type Tree: BinaryTree;

    fn get_left(&self) -> &Self::Tree;
    fn get_right(&self) -> &Self::Tree;
    fn get_child(&self, side: Side) -> &Self::Tree {
        match side {
            Side::Left => self.get_left(),
            Side::Right => self.get_right(),
        }
    }
    
    fn has_left(&self) -> bool { !self.get_left().is_leaf() }
    fn has_right(&self) -> bool { !self.get_right().is_leaf() }
    fn has_child(&self, side: Side) -> bool {
        match side {
            Side::Left => self.has_left(),
            Side::Right => self.has_right(),
        }
    }
}

pub trait BinaryTreeNodeMut: BinaryTreeNode<Tree: BinaryTreeMut> {
    fn get_left_mut(&mut self) -> &mut Self::Tree;
    fn get_right_mut(&mut self) -> &mut Self::Tree;
    fn get_child_mut(&mut self, side: Side) -> &mut Self::Tree {
        match side {
            Side::Left => self.get_left_mut(),
            Side::Right => self.get_right_mut(),
        }
    }

    fn attach_left(&mut self, tree: impl Into<Self::Tree>) -> bool;
    fn attach_right(&mut self, tree: impl Into<Self::Tree>) -> bool;
    fn attach_child(&mut self, side: Side, tree: impl Into<Self::Tree>) -> bool {
        match side {
            Side::Left => self.attach_left(tree),
            Side::Right => self.attach_right(tree),
        }
    }
    
    fn detach_left(&mut self) -> Self::Tree;
    fn detach_right(&mut self) -> Self::Tree;
    fn detach_child(&mut self, side: Side) -> Self::Tree {
        match side {
            Side::Left => self.detach_left(),
            Side::Right => self.detach_right(),
        }
    }

    fn replace_left(&mut self, tree: impl Into<Self::Tree>) -> Self::Tree;
    fn replace_right(&mut self, tree: impl Into<Self::Tree>) -> Self::Tree;
    fn replace_child(&mut self, side: Side, tree: impl Into<Self::Tree>) -> Self::Tree {
        match side {
            Side::Left => self.replace_left(tree),
            Side::Right => self.replace_right(tree),
        }
    }
}

pub trait BinarySearchTreeNode: BinaryTreeNodeMut {
    type Key: Ord;
    type Value;

    fn new(key: Self::Key, value: Self::Value) -> Self;
    fn key(&self) -> &Self::Key;
    fn value(&self) -> &Self::Value;
    fn replace_value(&mut self, value: Self::Value) -> Self::Value;
}

impl<T> BinaryTree for T
where
    T: Deref<Target: BinaryTree + Sized> + From<T::Target>,
{
    type Node = <<T as Deref>::Target as BinaryTree>::Node;

    fn new(root: Self::Node) -> Self {
        <Self as Deref>::Target::new(root).into()
    }

    fn new_leaf() -> Self {
        <Self as Deref>::Target::new_leaf().into()
    }

    fn root(&self) -> Option<&Self::Node> {
        self.deref().root()
    }

    fn is_leaf(&self) -> bool {
        self.deref().is_leaf()
    }
}

impl<T> BinaryTreeMut for T
where 
    T: DerefMut<Target: BinaryTreeMut + Sized> + From<T::Target>,
{
    fn root_mut(&mut self) -> Option<&mut Self::Node> {
        self.deref_mut().root_mut()
    }
}

impl<N> BinaryTreeNode for N
where
    N: Deref<Target: BinaryTreeNode>,
{
    type Tree = <<N as Deref>::Target as BinaryTreeNode>::Tree;

    fn get_left(&self) -> &Self::Tree {
        self.deref().get_left()
    }

    fn get_right(&self) -> &Self::Tree {
        self.deref().get_right()
    }
}
