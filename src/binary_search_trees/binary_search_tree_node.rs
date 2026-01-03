use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub trait BinaryTreeNode {
    type Wrapper;
    type Edge: DerefMut<Target = Self::Wrapper> + From<Self::Wrapper>;

    fn get_left_node(&self) -> Option<&Self::Wrapper> { self.get_left_edge().map(|left| left.deref()) }
    fn get_right_node(&self) -> Option<&Self::Wrapper> { self.get_right_edge().map(|right| right.deref()) }
    fn get_child_node(&self, side: Side) -> Option<&Self::Wrapper> {
        match side {
            Side::Left => self.get_left_node(),
            Side::Right => self.get_right_node(),
        }
    }

    fn get_left_node_mut(&mut self) -> Option<&mut Self::Wrapper> { self.get_left_edge_mut().map(|left| left.deref_mut()) }
    fn get_right_node_mut(&mut self) -> Option<&mut Self::Wrapper> { self.get_right_edge_mut().map(|right| right.deref_mut()) }
    fn get_child_node_mut(&mut self, side: Side) -> Option<&mut Self::Wrapper> {
        match side {
            Side::Left => self.get_left_node_mut(),
            Side::Right => self.get_right_node_mut(),
        }
    }
    
    fn get_left_edge(&self) -> Option<&Self::Edge>;
    fn get_right_edge(&self) -> Option<&Self::Edge>;
    fn get_edge(&self, side: Side) -> Option<&Self::Edge> {
        match side {
            Side::Left => self.get_left_edge(),
            Side::Right => self.get_right_edge(),
        }
    }

    fn get_left_edge_mut(&mut self) -> Option<&mut Self::Edge>;
    fn get_right_edge_mut(&mut self) -> Option<&mut Self::Edge>;
    fn get_edge_mut(&mut self, side: Side) -> Option<&mut Self::Edge> {
        match side {
            Side::Left => self.get_left_edge_mut(),
            Side::Right => self.get_right_edge_mut(),
        }
    }
    
    fn has_left(&self) -> bool { self.get_left_node().is_some() }
    fn has_right(&self) -> bool { self.get_right_node().is_some() }
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

pub trait BinarySearchTreeNode: BinaryTreeNode {
    type Key: Ord;
    type Value;

    fn new(key: Self::Key, value: Self::Value) -> Self;
    fn key(&self) -> &Self::Key;
    fn value(&self) -> &Self::Value;
    fn replace_value(&mut self, value: Self::Value) -> Self::Value;
}
