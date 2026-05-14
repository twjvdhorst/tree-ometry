use crate::binary_trees::traits::binary_tree_cursor::{BinaryTreeCursor, BinaryTreeCursorMut};

pub trait BinaryTree {
    type Node;
    type Cursor<'c>: BinaryTreeCursor<Node = Self::Node>
    where Self: 'c;
    
    fn cursor(&self) -> Self::Cursor<'_>;
}

pub trait BinaryTreeMut: BinaryTree {
    type CursorMut<'c>: BinaryTreeCursorMut<Node = Self::Node>
    where Self: 'c;

    fn cursor_mut(&mut self) -> Self::CursorMut<'_>;
}
