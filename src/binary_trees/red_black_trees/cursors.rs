use derive_more::Debug;

use super::red_black_tree::RedBlackNode;
use crate::binary_trees::{
    binary_tree::{self, binary_tree::BinaryTreeNode}, traits::binary_tree_cursor::{BinaryTreeCursor, BinaryTreeCursorMut}
};

/// A cursor over a RedBlackTree.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
#[derive(Debug)]
pub struct Cursor<'tree, K, V>(binary_tree::cursors::Cursor<'tree, RedBlackNode<K, V>>);

impl<'tree, K, V> Cursor<'tree, K, V> {
    pub(super) fn new(cursor: binary_tree::cursors::Cursor<'tree, RedBlackNode<K, V>>) -> Self {
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

    fn node(&self) -> Option<&Self::Node> {
        self.0.node().map(BinaryTreeNode::data)
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

    /// Advances the cursor to the parent node.
    /// If the cursor is already at the root of the tree, None is returned and the cursor is not moved.
    fn move_up(&mut self) -> Option<&Self::Node> {
        self.0.move_up().map(BinaryTreeNode::data)
    }
    
    /// Advances the cursor to the left child node.
    /// If the cursor is already at a leaf of the tree, None is returned and the cursor is not moved.
    fn move_left(&mut self) -> Option<&Self::Node> {
        self.0.move_left().map(BinaryTreeNode::data)
    }
    
    /// Advances the cursor to the right child node.
    /// If the cursor is already at a leaf of the tree, None is returned and the cursor is not moved.
    fn move_right(&mut self) -> Option<&Self::Node> {
        self.0.move_right().map(BinaryTreeNode::data)
    }
}

/// A cursor over a BinaryTree with editing operations.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
/// Cursors maintain the invariant that as long as the tree has a node, the cursor points to a node.
#[derive(Debug)]
pub struct CursorMut<'tree, K, V>(binary_tree::cursors::CursorMut<'tree, RedBlackNode<K, V>>);

impl<'tree, K, V> CursorMut<'tree, K, V> {
    pub(super) fn new(cursor: binary_tree::cursors::CursorMut<'tree, RedBlackNode<K, V>>) -> Self {
        Self(cursor)
    }
}

impl<'tree, K, V> BinaryTreeCursor for CursorMut<'tree, K, V> {
    type Node = RedBlackNode<K, V>;

    fn node(&self) -> Option<&Self::Node> {
        self.0.node().map(BinaryTreeNode::data)
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

    /// Advances the cursor to the parent node.
    /// If the cursor is already at the root of the tree, None is returned and the cursor is not moved.
    fn move_up(&mut self) -> Option<&Self::Node> {
        self.0.move_up().map(BinaryTreeNode::data)
    }
    
    /// Advances the cursor to the left child node.
    /// If the cursor is already at a leaf of the tree, None is returned and the cursor is not moved.
    fn move_left(&mut self) -> Option<&Self::Node> {
        self.0.move_left().map(BinaryTreeNode::data)
    }
    
    /// Advances the cursor to the right child node.
    /// If the cursor is already at a leaf of the tree, None is returned and the cursor is not moved.
    fn move_right(&mut self) -> Option<&Self::Node> {
        self.0.move_right().map(BinaryTreeNode::data)
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
    
    /// Advances the cursor to the parent node.
    /// If the cursor is already at the root of the tree, None is returned and the cursor is not moved.
    fn move_up_mut(&mut self) -> Option<&mut Self::Node> {
        self.0.move_up_mut().map(BinaryTreeNode::data_mut)
    }
    
    /// Advances the cursor to the left child node.
    /// If the cursor is already at a leaf of the tree, None is returned and the cursor is not moved.
    fn move_left_mut(&mut self) -> Option<&mut Self::Node> {
        self.0.move_left_mut().map(BinaryTreeNode::data_mut)
    }
    
    /// Advances the cursor to the right child node.
    /// If the cursor is already at a leaf of the tree, None is returned and the cursor is not moved.
    fn move_right_mut(&mut self) -> Option<&mut Self::Node> {
        self.0.move_right_mut().map(BinaryTreeNode::data_mut)
    }
}
