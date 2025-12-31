use std::ops::DerefMut;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub trait BinaryTreeNode {
    type Data;
    type DataPointer: DerefMut<Target = Self::Data> + From<Self::Data>;

    fn get_left(&self) -> Option<&Self::Data>;
    fn get_right(&self) -> Option<&Self::Data>;
    fn get_child(&self, side: Side) -> Option<&Self::Data> {
        match side {
            Side::Left => self.get_left(),
            Side::Right => self.get_right(),
        }
    }

    fn get_left_mut(&mut self) -> Option<&mut Self::Data>;
    fn get_right_mut(&mut self) -> Option<&mut Self::Data>;
    fn get_child_mut(&mut self, side: Side) -> Option<&mut Self::Data> {
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

    fn attach_left(&mut self, tree: impl Into<Self::DataPointer>) -> bool;
    fn attach_right(&mut self, tree: impl Into<Self::DataPointer>) -> bool;
    fn attach_child(&mut self, side: Side, tree: impl Into<Self::DataPointer>) -> bool {
        match side {
            Side::Left => self.attach_left(tree),
            Side::Right => self.attach_right(tree),
        }
    }
    
    fn detach_left(&mut self) -> Option<Self::DataPointer>;
    fn detach_right(&mut self) -> Option<Self::DataPointer>;
    fn detach_child(&mut self, side: Side) -> Option<Self::DataPointer> {
        match side {
            Side::Left => self.detach_left(),
            Side::Right => self.detach_right(),
        }
    }

    fn replace_left(&mut self, tree: impl Into<Self::DataPointer>) -> Option<Self::DataPointer>;
    fn replace_right(&mut self, tree: impl Into<Self::DataPointer>) -> Option<Self::DataPointer>;
    fn replace_child(&mut self, side: Side, tree: impl Into<Self::DataPointer>) -> Option<Self::DataPointer> {
        match side {
            Side::Left => self.replace_left(tree),
            Side::Right => self.replace_right(tree),
        }
    }
}
