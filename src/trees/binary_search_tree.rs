use std::{borrow::Borrow, cmp::Ordering, ops::DerefMut};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub trait BinarySearchTree {
    type Key: Ord;
    type Node;
    type Edge: DerefMut<Target = Self::Node> + From<Self::Node>;

    fn new(key: Self::Key) -> Self;
    fn key(&self) -> &Self::Key;
    
    fn pick_branch<T>(&self, value: &T) -> Option<Side>
    where
        Self::Key: Borrow<T>,
        T: Ord + ?Sized,
    {
        match T::cmp(value, self.key().borrow()) {
            Ordering::Less => Some(Side::Left),
            Ordering::Greater => Some(Side::Right),
            Ordering::Equal => None,
        }
    }

    fn get_left(&self) -> Option<&Self::Node>;
    fn get_right(&self) -> Option<&Self::Node>;
    fn get_child(&self, side: Side) -> Option<&Self::Node> {
        match side {
            Side::Left => self.get_left(),
            Side::Right => self.get_right(),
        }
    }

    fn get_left_mut(&mut self) -> Option<&mut Self::Node>;
    fn get_right_mut(&mut self) -> Option<&mut Self::Node>;
    fn get_child_mut(&mut self, side: Side) -> Option<&mut Self::Node> {
        match side {
            Side::Left => self.get_left_mut(),
            Side::Right => self.get_right_mut(),
        }
    }
    
    fn has_left(&self) -> bool;
    fn has_right(&self) -> bool;
    fn has_child(&self, side: Side) -> bool {
        match side {
            Side::Left => self.has_left(),
            Side::Right => self.has_right(),
        }
    }

    fn attach_left(&mut self, tree: impl Into<Self::Edge>) -> bool;
    fn attach_right(&mut self, tree: impl Into<Self::Edge>) -> bool;
    fn attach_child(&mut self, side: Side, tree: impl Into<Self::Edge>) -> bool {
        match side {
            Side::Left => self.attach_left(tree),
            Side::Right => self.attach_right(tree),
        }
    }
    
    fn detach_left(&mut self) -> Option<Self::Edge>;
    fn detach_right(&mut self) -> Option<Self::Edge>;
    fn detach_child(&mut self, side: Side) -> Option<Self::Edge> {
        match side {
            Side::Left => self.detach_left(),
            Side::Right => self.detach_right(),
        }
    }

    fn replace_left(&mut self, tree: impl Into<Self::Edge>) -> Option<Self::Edge>;
    fn replace_right(&mut self, tree: impl Into<Self::Edge>) -> Option<Self::Edge>;
    fn replace_child(&mut self, side: Side, tree: impl Into<Self::Edge>) -> Option<Self::Edge> {
        match side {
            Side::Left => self.replace_left(tree),
            Side::Right => self.replace_right(tree),
        }
    }
}
