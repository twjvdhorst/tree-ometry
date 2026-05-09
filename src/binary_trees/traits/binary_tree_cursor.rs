use crate::binary_trees::Side;

pub trait BinaryTreeCursor: Sized {
    type Node;

    fn node(&self) -> &Self::Node;

    fn peek_up(&self) -> Option<&Self::Node>;
    fn peek_left(&self) -> Option<&Self::Node>;
    fn peek_right(&self) -> Option<&Self::Node>;
    fn peek_side(&self, side: Side) -> Option<&Self::Node> {
        match side {
            Side::Left => self.peek_left(),
            Side::Right => self.peek_right(),
        }
    }

    /// Advances the cursor to the parent node.
    /// If the cursor is already at the root of the tree, None is returned and the cursor is not moved.
    fn move_up(&mut self) -> Option<&Self::Node>;
    
    /// Advances the cursor to the left child node.
    /// If the cursor is already at a leaf of the tree, None is returned and the cursor is not moved.
    fn move_left(&mut self) -> Option<&Self::Node>;

    /// Advances the cursor to the right child node.
    /// If the cursor is already at a leaf of the tree, None is returned and the cursor is not moved.
    fn move_right(&mut self) -> Option<&Self::Node>;

    fn move_side(&mut self, side: Side) -> Option<&Self::Node> {
        match side {
            Side::Left => self.move_left(),
            Side::Right => self.move_right(),
        }
    }
}

pub(crate) trait BinaryTreeCursorMut: BinaryTreeCursor {
    fn node_mut(&mut self) -> &mut Self::Node;

    fn peek_up_mut(&mut self) -> Option<&mut Self::Node>;
    fn peek_left_mut(&mut self) -> Option<&mut Self::Node>;
    fn peek_right_mut(&mut self) -> Option<&mut Self::Node>;
    fn peek_side_mut(&mut self, side: Side) -> Option<&mut Self::Node> {
        match side {
            Side::Left => self.peek_left_mut(),
            Side::Right => self.peek_right_mut(),
        }
    }

    /// Advances the cursor to the parent node.
    /// If the cursor is already at the root of the tree, None is returned and the cursor is not moved.
    fn move_up_mut(&mut self) -> Option<&mut Self::Node>;

    /// Advances the cursor to the left child node.
    /// If the cursor is already at a leaf of the tree, None is returned and the cursor is not moved.
    fn move_left_mut(&mut self) -> Option<&mut Self::Node>;
    
    /// Advances the cursor to the right child node.
    /// If the cursor is already at a leaf of the tree, None is returned and the cursor is not moved.
    fn move_right_mut(&mut self) -> Option<&mut Self::Node>;

    fn move_side_mut(&mut self, side: Side) -> Option<&mut Self::Node> {
        match side {
            Side::Left => self.move_left_mut(),
            Side::Right => self.move_right_mut(),
        }
    }
}
