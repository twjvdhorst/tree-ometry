use std::ops::DerefMut;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub trait BinaryTreeNode {
    type Wrapper;
    type Edge: DerefMut<Target = Self::Wrapper> + From<Self::Wrapper>;

    fn get_left(&self) -> Option<&Self::Wrapper>;
    fn get_right(&self) -> Option<&Self::Wrapper>;
    fn get_child(&self, side: Side) -> Option<&Self::Wrapper> {
        match side {
            Side::Left => self.get_left(),
            Side::Right => self.get_right(),
        }
    }

    fn get_left_mut(&mut self) -> Option<&mut Self::Wrapper>;
    fn get_right_mut(&mut self) -> Option<&mut Self::Wrapper>;
    fn get_child_mut(&mut self, side: Side) -> Option<&mut Self::Wrapper> {
        match side {
            Side::Left => self.get_left_mut(),
            Side::Right => self.get_right_mut(),
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
