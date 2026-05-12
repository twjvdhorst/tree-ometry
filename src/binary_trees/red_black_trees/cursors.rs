use derive_more::Debug;

use super::red_black_tree::RedBlackNode;
use crate::binary_trees::{
    Side, binary_tree::{self, BinaryTreeNode}, cursor_errors::CursorError, red_black_trees::Color, traits::binary_tree_cursor::{BinaryTreeCursor, BinaryTreeCursorMut}
};

/// A cursor over a RedBlackTree.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
#[derive(Debug)]
pub struct Cursor<'tree, K, V>(binary_tree::Cursor<'tree, RedBlackNode<K, V>>);

impl<'tree, K, V> Cursor<'tree, K, V> {
    pub(super) fn new(cursor: binary_tree::Cursor<'tree, RedBlackNode<K, V>>) -> Self {
        Self(cursor)
    }
}

impl<'tree, K, V> Clone for Cursor<'tree, K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'tree, K, V> Copy for Cursor<'tree, K, V> {}

impl<'tree, K, V> BinaryTreeCursor for Cursor<'tree, K, V> {
    type Node = RedBlackNode<K, V>;
    type Cursor<'c> = Self
    where Self: 'c;

    fn node(&self) -> Option<&Self::Node> {
        self.0.node().map(BinaryTreeNode::data)
    }

    fn spawn_cursor(&self) -> Self::Cursor<'_> {
        self.clone()
    }

    fn side_of_parent(&self) -> Option<crate::binary_trees::Side> {
        self.0.side_of_parent()
    }

    fn peek_up(&self) -> Option<&Self::Node> {
        self.0.peek_up().map(BinaryTreeNode::data)
    }

    fn peek_left(&self) -> Option<&Self::Node> {
        self.0.peek_left().map(BinaryTreeNode::data)
    }

    fn peek_right(&self) -> Option<&Self::Node> {
        self.0.peek_right().map(BinaryTreeNode::data)
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

/// A cursor over a BinaryTree with editing operations.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
/// Cursors maintain the invariant that as long as the tree has a node, the cursor points to a node.
#[derive(Debug)]
pub struct CursorMut<'tree, K, V>(binary_tree::CursorMut<'tree, RedBlackNode<K, V>>);

impl<'tree, K, V> CursorMut<'tree, K, V> {
    pub(super) fn new(cursor: binary_tree::CursorMut<'tree, RedBlackNode<K, V>>) -> Self {
        Self(cursor)
    }

    pub(super) fn color(&self) -> Option<Color> {
        self.node().map(RedBlackNode::color)
    }

    pub(super) fn set_color(&mut self, color: Color) {
        if let Some(node) = self.node_mut() {
            node.set_color(color);
        }
    }

    /// Creates a new node and attaches it as a child to the node pointed at by the cursor.
    pub(super) fn attach_child(&mut self, node: RedBlackNode<K, V>, side: Side) -> Result<(), CursorError> {
        self.0.attach_child(node, side)
    }

    pub(super) fn rotate(&mut self, side: Side) -> Result<(), CursorError> {
        match side {
            Side::Left => self.0.rotate_left(),
            Side::Right => self.0.rotate_right(),
        }
    }
}

impl<'tree, K, V> BinaryTreeCursor for CursorMut<'tree, K, V> {
    type Node = RedBlackNode<K, V>;
    type Cursor<'c> = Cursor<'c, K, V>
    where Self: 'c;

    fn node(&self) -> Option<&Self::Node> {
        self.0.node().map(BinaryTreeNode::data)
    }

    fn spawn_cursor(&self) -> Self::Cursor<'_> {
        Cursor::new(self.0.spawn_cursor())
    }

    fn side_of_parent(&self) -> Option<crate::binary_trees::Side> {
        self.0.side_of_parent()
    }

    fn peek_up(&self) -> Option<&Self::Node> {
        self.0.peek_up().map(BinaryTreeNode::data)
    }

    fn peek_left(&self) -> Option<&Self::Node> {
        self.0.peek_left().map(BinaryTreeNode::data)
    }

    fn peek_right(&self) -> Option<&Self::Node> {
        self.0.peek_right().map(BinaryTreeNode::data)
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

impl<'tree, K, V> BinaryTreeCursorMut for CursorMut<'tree, K, V> {
    fn node_mut(&mut self) -> Option<&mut Self::Node> {
        self.0.node_mut().map(BinaryTreeNode::data_mut)
    }

    fn peek_up_mut(&mut self) -> Option<&mut Self::Node> {
        self.0.peek_up_mut().map(BinaryTreeNode::data_mut)
    }

    fn peek_left_mut(&mut self) -> Option<&mut Self::Node> {
        self.0.peek_left_mut().map(BinaryTreeNode::data_mut)
    }

    fn peek_right_mut(&mut self) -> Option<&mut Self::Node> {
        self.0.peek_right_mut().map(BinaryTreeNode::data_mut)
    }

    fn peek_both_mut(&mut self) -> (Option<&mut Self::Node>, Option<&mut Self::Node>) {
        let (left, right) = self.0.peek_both_mut();
        (left.map(BinaryTreeNode::data_mut), right.map(BinaryTreeNode::data_mut))
    }
}
