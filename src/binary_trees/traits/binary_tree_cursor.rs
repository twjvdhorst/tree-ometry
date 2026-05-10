use crate::binary_trees::Side;

pub trait BinaryTreeCursor: Sized {
    type Node;

    fn node(&self) -> Option<&Self::Node>;

    fn peek_up(&self) -> Option<&Self::Node>;
    fn peek_left(&self) -> Option<&Self::Node>;
    fn peek_right(&self) -> Option<&Self::Node>;
    fn peek_side(&self, side: Side) -> Option<&Self::Node> {
        match side {
            Side::Left => self.peek_left(),
            Side::Right => self.peek_right(),
        }
    }

    fn peek_both(&self) -> (Option<&Self::Node>, Option<&Self::Node>) {
        (self.peek_left(), self.peek_right())
    }
    
    fn move_up(&mut self) -> Option<&Self::Node>;
    fn move_left(&mut self) -> Option<&Self::Node>;
    fn move_right(&mut self) -> Option<&Self::Node>;
    fn move_side(&mut self, side: Side) -> Option<&Self::Node> {
        match side {
            Side::Left => self.move_left(),
            Side::Right => self.move_right(),
        }
    }
}

pub trait BinaryTreeCursorMut: BinaryTreeCursor {
    fn node_mut(&mut self) -> Option<&mut Self::Node>;

    fn peek_up_mut(&mut self) -> Option<&mut Self::Node>;
    fn peek_left_mut(&mut self) -> Option<&mut Self::Node>;
    fn peek_right_mut(&mut self) -> Option<&mut Self::Node>;
    fn peek_side_mut(&mut self, side: Side) -> Option<&mut Self::Node> {
        match side {
            Side::Left => self.peek_left_mut(),
            Side::Right => self.peek_right_mut(),
        }
    }

    fn peek_both_mut(&mut self) -> (Option<&mut Self::Node>, Option<&mut Self::Node>);

    fn move_up_mut(&mut self) -> Option<&mut Self::Node>;
    fn move_left_mut(&mut self) -> Option<&mut Self::Node>;
    fn move_right_mut(&mut self) -> Option<&mut Self::Node>;
    fn move_side_mut(&mut self, side: Side) -> Option<&mut Self::Node> {
        match side {
            Side::Left => self.move_left_mut(),
            Side::Right => self.move_right_mut(),
        }
    }
}
