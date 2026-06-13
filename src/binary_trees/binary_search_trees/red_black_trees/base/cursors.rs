use std::marker::PhantomData;

use derive_more::Debug;

use super::{RbNode, RedBlackTree};
use crate::binary_trees::{
    Neighborhood, Side, binary_search_trees::red_black_trees::Color, binary_tree, binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursor,
        PeekingCursorMut,
    }, cursor_errors::CursorError
};

/// A cursor over a RedBlackTreeBase.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
#[derive(Debug)]
pub(super) struct Cursor<'t, T>(binary_tree::Cursor<'t, RbNode<T>>);

/// Make own implementation of Clone, so T doesn't have to be Clone.
impl<'t, T> Clone for Cursor<'t, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'t, T> Copy for Cursor<'t, T> {}

impl<'t, T> From<CursorMut<'t, T>> for Cursor<'t, T> {
    fn from(value: CursorMut<'t, T>) -> Self {
        Self(value.0.into())
    }
}

impl<'t, T> Cursor<'t, T> {
    pub(super) fn new(cursor: binary_tree::Cursor<'t, RbNode<T>>) -> Self {
        Self(cursor)
    }
}

impl<'t, T> BinaryTreeCursor for Cursor<'t, T> {   
    fn side_of_parent(&self) -> Option<Side> {
        self.0.side_of_parent()
    }

    fn try_move_up(&mut self) -> Option<Side> {
        self.0.try_move_up()
    }
    
    fn try_move_left(&mut self) -> bool {
        self.0.try_move_left()
    }
    
    fn try_move_right(&mut self) -> bool {
        self.0.try_move_right()
    }

    fn move_up(&mut self) -> Option<Side> {
        self.0.move_up()
    }

    fn move_left(&mut self) {
        self.0.move_left();
    }

    fn move_right(&mut self) {
        self.0.move_right();
    }
}

impl<'t, T> PeekingCursor for Cursor<'t, T> {
    type Item = &'t RbNode<T>;

    fn get(&self) -> Option<Self::Item> {
        self.0.get()
    }

    fn peek_up(&self) -> Option<Self::Item> {
        self.0.peek_up()
    }

    fn peek_left(&self) -> Option<Self::Item> {
        self.0.peek_left()
    }

    fn peek_right(&self) -> Option<Self::Item> {
        self.0.peek_right()
    }

    fn peek_neighborhood(&self) -> Neighborhood<Self::Item> {
        self.0.peek_neighborhood()
    }
}

/// A cursor over a SemigroupRbTree with editing operations.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
/// Cursors maintain the invariant that as long as the tree has a node, the cursor points to a node.
#[derive(Debug)]
pub(super) struct CursorMut<'t, T>(binary_tree::CursorMut<'t, RbNode<T>>);

impl<'t, T> CursorMut<'t, T> {
    pub(super) fn new(cursor: binary_tree::CursorMut<'t, RbNode<T>>) -> Self {
        Self(cursor)
    }

    pub(super) fn color(&self) -> Option<Color> {
        self.0.get().map(RbNode::color)
    }

    pub(super) fn parent_color(&self) -> Option<Color> {
        self.0.peek_up().map(RbNode::color)
    }

    pub(super) fn left_color(&self) -> Option<Color> {
        self.0.peek_left().map(RbNode::color)
    }

    pub(super) fn right_color(&self) -> Option<Color> {
        self.0.peek_right().map(RbNode::color)
    }

    pub(super) fn uncle_color(&self) -> Option<Color> {
        let mut cursor = self.as_cursor();
        let side = cursor.move_up()?;
        cursor.peek_side(side.opposite()).map(RbNode::color)
    }

    pub(super) fn set_color(&mut self, color: Color) {
        if let Some(node) = self.0.get_mut() {
            node.set_color(color);
        }
    }

    pub(super) fn set_child_color(&mut self, side: Side, color: Color) {
        match side {
            Side::Left => if let Some(left) = self.0.peek_left_mut() {
                left.set_color(color);
            },
            Side::Right => if let Some(right) = self.0.peek_right_mut() {
                right.set_color(color);
            }
        }
    }
    
    pub(super) fn rotate(&mut self, side: Side) -> Result<(), CursorError> {
        self.0.rotate(side)
    }

    pub(super) fn attach_child(&mut self, node: RbNode<T>, side: Side) -> Result<(), CursorError> {
        self.0.attach_child(node, side)
    }
}

impl<'t, T> BinaryTreeCursor for CursorMut<'t, T> {   
    fn side_of_parent(&self) -> Option<Side> {
        self.0.side_of_parent()
    }

    fn try_move_up(&mut self) -> Option<Side> {
        self.0.try_move_up()
    }
    
    fn try_move_left(&mut self) -> bool {
        self.0.try_move_left()
    }
    
    fn try_move_right(&mut self) -> bool {
        self.0.try_move_right()
    }

    fn move_up(&mut self) -> Option<Side> {
        self.0.move_up()
    }

    fn move_left(&mut self) {
        self.0.move_left();
    }

    fn move_right(&mut self) {
        self.0.move_right();
    }
}

impl<'t, T> PeekingCursorMut for CursorMut<'t, T> {
    type Item<'c> = &'c RbNode<T> where Self: 'c;
    type ItemMut<'c> = &'c mut RbNode<T> where Self: 'c;
    type AsCursor<'c> = Cursor<'c, T> where Self: 'c;

    fn get(&self) -> Option<Self::Item<'_>> {
        self.0.get()
    }

    fn get_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.get_mut()
    }

    fn as_cursor(&self) -> Self::AsCursor<'_> {
        Cursor::new(self.0.as_cursor())
    }

    fn peek_up(&self) -> Option<Self::Item<'_>> {
        self.0.peek_up()
    }

    fn peek_left(&self) -> Option<Self::Item<'_>> {
        self.0.peek_left()
    }

    fn peek_right(&self) -> Option<Self::Item<'_>> {
        self.0.peek_right()
    }

    fn peek_neighborhood(&self) -> Neighborhood<Self::Item<'_>> {
        self.0.peek_neighborhood()
    }

    fn peek_up_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_up_mut()
    }

    fn peek_left_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_left_mut()
    }

    fn peek_right_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_right_mut()
    }

    fn peek_neighborhood_mut(&mut self) -> Neighborhood<Self::ItemMut<'_>> {
        self.0.peek_neighborhood_mut()
    }
}
