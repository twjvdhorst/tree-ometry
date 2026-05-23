use crate::binary_trees::Side;

pub struct Neighborhood<'c, T> {
    pub node: Option<&'c T>,
    pub parent: Option<&'c T>,
    pub left: Option<&'c T>,
    pub right: Option<&'c T>,
}

pub struct NeighborhoodMut<'c, T> {
    pub node: Option<&'c mut T>,
    pub parent: Option<&'c mut T>,
    pub left: Option<&'c mut T>,
    pub right: Option<&'c mut T>,
}

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

    /// Moves the cursor to the inorder predecessor of the current node.
    /// If no predecessor exists, false is returned and the cursor is not moved.
    fn try_move_prev(&mut self) -> bool {
        // Inorder predecessor is either the rightmost node in the left subtree, or the first ancestor that we are a right descendant of.
        if self.try_move_left() {
            while self.try_move_right() {}
            return true;
        }
        
        while let Some(side) = self.try_move_up() {
            if side == Side::Right {
                return true;
            }
        }

        false
    }

    /// Moves the cursor to the inorder successor of the current node.
    /// If no successor exists, false is returned and the cursor is not moved.
    fn try_move_next(&mut self) -> bool {
        // Inorder successor is either the leftmost node in the right subtree, or the first ancestor that we are a left descendant of.
        if self.try_move_right() {
            while self.try_move_left() {}
            return true;
        }
        
        while let Some(side) = self.try_move_up() {
            if side == Side::Left {
                return true;
            }
        }

        false
    }

    /// Moves the cursor to the inorder predecessor of the current node.
    /// If no predecessor exists, the cursor is moved to a "null" node.
    fn move_prev(&mut self) {
        // Inorder predecessor is either the rightmost node in the left subtree, or the first ancestor that we are a right descendant of.
        if self.try_move_left() {
            while self.try_move_right() {}
        } else {
            while let Some(side) = self.move_up() && side != Side::Right {}
        }
    }

    /// Moves the cursor to the inorder successor of the current node.
    /// If no successor exists, the cursor is moved to a "null" node.
    fn move_next(&mut self) {
        // Inorder successor is either the leftmost node in the right subtree, or the first ancestor that we are a left descendant of.
        if self.try_move_right() {
            while self.try_move_left() {}
        } else {
            while let Some(side) = self.move_up() && side != Side::Left {}
        }
    }
}

pub trait PeekingCursor: BinaryTreeCursor {
    type Item;

    fn get(&self) -> Option<Self::Item>;
    fn spawn_cursor(&self) -> Self;

    fn side_of_parent(&self) -> Option<Side>;

    fn peek_up(&self) -> Option<Self::Item>;
    fn peek_left(&self) -> Option<Self::Item>;
    fn peek_right(&self) -> Option<Self::Item>;
    fn peek_side(&self, side: Side) -> Option<Self::Item> {
        match side {
            Side::Left => self.peek_left(),
            Side::Right => self.peek_right(),
        }
    }

    fn peek_neighborhood(&self) -> Neighborhood<'_, Self::Item>;
}

pub trait PeekingCursorMut: BinaryTreeCursor {
    type Item<'c>
    where Self: 'c;
    type AsCursor<'c>: PeekingCursor<Item = Self::Item<'c>>
    where Self: 'c;

    fn get(&self) -> Option<Self::Item<'_>>;
    fn spawn_cursor(&self) -> Self::AsCursor<'_>;

    fn side_of_parent(&self) -> Option<Side>;

    fn peek_up(&self) -> Option<Self::Item<'_>>;
    fn peek_left(&self) -> Option<Self::Item<'_>>;
    fn peek_right(&self) -> Option<Self::Item<'_>>;
    fn peek_side(&self, side: Side) -> Option<Self::Item<'_>> {
        match side {
            Side::Left => self.peek_left(),
            Side::Right => self.peek_right(),
        }
    }

    fn peek_neighborhood(&self) -> Neighborhood<'_, Self::Item<'_>>;
}
