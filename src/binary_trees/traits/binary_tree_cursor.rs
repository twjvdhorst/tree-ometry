use crate::binary_trees::Side;

pub trait BinaryTreeCursor {
    /// Advances the cursor to the parent node, returning the side of the parent node that the cursor previously pointed to.
    /// If the parent node does not exit, None is returned and the cursor is not moved.
    fn try_move_up(&mut self) -> Option<Side>;
    
    /// Advances the cursor to the left child node.
    /// If the left child does not exist, false is returned and the cursor is not moved.
    fn try_move_left(&mut self) -> bool;
    
    /// Advances the cursor to the right child node.
    /// If the right child does not exist, false is returned and the cursor is not moved.
    fn try_move_right(&mut self) -> bool;

    fn try_move_side(&mut self, side: Side) -> bool {
        match side {
            Side::Left => self.try_move_left(),
            Side::Right => self.try_move_right(),
        }
    }

    /// Advances the cursor to the parent node, returning the side of the parent node that the cursor previously pointed to.
    /// If the parent node does not exit, the cursor is moved to a "null" node.
    fn move_up(&mut self) -> Option<Side>;
    
    /// Advances the cursor to the left child node.
    /// If the left child does not exist, the cursor is moved to a "null" node.
    fn move_left(&mut self);
    
    /// Advances the cursor to the right child node.
    /// If the right child does not exist, the cursor is moved to a "null" node.
    fn move_right(&mut self);

    fn move_side(&mut self, side: Side) {
        match side {
            Side::Left => self.move_left(),
            Side::Right => self.move_right(),
        }
    }
}

pub trait PeekingCursor<'t>: BinaryTreeCursor {
    type Item;

    fn get(&self) -> Option<&'t Self::Item>;
    fn spawn_cursor(&self) -> Self;

    fn side_of_parent(&self) -> Option<Side>;

    fn peek_up(&self) -> Option<&'t Self::Item>;
    fn peek_left(&self) -> Option<&'t Self::Item>;
    fn peek_right(&self) -> Option<&'t Self::Item>;
    fn peek_side(&self, side: Side) -> Option<&'t Self::Item> {
        match side {
            Side::Left => self.peek_left(),
            Side::Right => self.peek_right(),
        }
    }
}

pub trait PeekingCursorMut: BinaryTreeCursor {
    type Item;
    type SpawnedCursor<'c>: PeekingCursor<'c, Item = Self::Item>
    where Self: 'c;

    fn get(&self) -> Option<&Self::Item>;
    fn get_mut(&mut self) -> Option<&mut Self::Item>;
    fn spawn_cursor(&self) -> Self::SpawnedCursor<'_>;

    fn side_of_parent(&self) -> Option<Side>;

    fn peek_up(&self) -> Option<&Self::Item>;
    fn peek_left(&self) -> Option<&Self::Item>;
    fn peek_right(&self) -> Option<&Self::Item>;
    fn peek_side(&self, side: Side) -> Option<&Self::Item> {
        match side {
            Side::Left => self.peek_left(),
            Side::Right => self.peek_right(),
        }
    }

    fn peek_up_mut(&mut self) -> Option<&mut Self::Item>;
    fn peek_left_mut(&mut self) -> Option<&mut Self::Item>;
    fn peek_right_mut(&mut self) -> Option<&mut Self::Item>;
    fn peek_side_mut(&mut self, side: Side) -> Option<&mut Self::Item> {
        match side {
            Side::Left => self.peek_left_mut(),
            Side::Right => self.peek_right_mut(),
        }
    }
}
