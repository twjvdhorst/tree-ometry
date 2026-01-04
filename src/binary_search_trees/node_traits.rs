use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub trait BinaryTreeNode {
    type Wrapper;
    type NodePointer: Deref<Target = Self::Wrapper> + From<Self::Wrapper>;

    fn get_left(&self) -> Option<&Self::Wrapper>;
    fn get_right(&self) -> Option<&Self::Wrapper>;
    fn get_child(&self, side: Side) -> Option<&Self::Wrapper> {
        match side {
            Side::Left => self.get_left(),
            Side::Right => self.get_right(),
        }
    }
    
    fn has_left(&self) -> bool { self.get_left().is_some() }
    fn has_right(&self) -> bool { self.get_right().is_some() }
    fn has_child(&self, side: Side) -> bool {
        match side {
            Side::Left => self.has_left(),
            Side::Right => self.has_right(),
        }
    }
}

pub trait BinaryTreeNodeMut: BinaryTreeNode<NodePointer: DerefMut<Target = Self::Wrapper>> {
    fn get_left_mut(&mut self) -> Option<&mut Self::Wrapper>;
    fn get_right_mut(&mut self) -> Option<&mut Self::Wrapper>;
    fn get_child_mut(&mut self, side: Side) -> Option<&mut Self::Wrapper> {
        match side {
            Side::Left => self.get_left_mut(),
            Side::Right => self.get_right_mut(),
        }
    }

    fn attach_left(&mut self, tree: impl Into<Self::NodePointer>) -> bool;
    fn attach_right(&mut self, tree: impl Into<Self::NodePointer>) -> bool;
    fn attach_child(&mut self, side: Side, tree: impl Into<Self::NodePointer>) -> bool {
        match side {
            Side::Left => self.attach_left(tree),
            Side::Right => self.attach_right(tree),
        }
    }
    
    fn detach_left(&mut self) -> Option<Self::NodePointer>;
    fn detach_right(&mut self) -> Option<Self::NodePointer>;
    fn detach_child(&mut self, side: Side) -> Option<Self::NodePointer> {
        match side {
            Side::Left => self.detach_left(),
            Side::Right => self.detach_right(),
        }
    }

    fn replace_left(&mut self, tree: impl Into<Self::NodePointer>) -> Option<Self::NodePointer>;
    fn replace_right(&mut self, tree: impl Into<Self::NodePointer>) -> Option<Self::NodePointer>;
    fn replace_child(&mut self, side: Side, tree: impl Into<Self::NodePointer>) -> Option<Self::NodePointer> {
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
