use derive_more::Debug;

use super::BstNode;
use crate::binary_trees::{
    Neighborhood, Side, binary_tree, cursor_errors::CursorError
};

/// A cursor over a BinarySearchTree.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
#[derive(Clone, Copy, Debug)]
pub struct Cursor<'t, K, V>(binary_tree::Cursor<'t, BstNode<K, V>>);

impl<'t, K, V> Cursor<'t, K, V> {
    pub(super) fn new(cursor: binary_tree::Cursor<'t, BstNode<K, V>>) -> Self {
        Self(cursor)
    }
    
    pub fn try_move_up(&mut self) -> Option<Side> {
        self.0.try_move_up()
    }
    
    pub fn try_move_left(&mut self) -> bool {
        self.0.try_move_left()
    }
    
    pub fn try_move_right(&mut self) -> bool {
        self.0.try_move_right()
    }

    pub fn try_move_side(&mut self, side: Side) -> bool {
        self.0.try_move_side(side)
    }

    pub fn move_up(&mut self) -> Option<Side> {
        self.0.move_up()
    }

    pub fn move_left(&mut self) {
        self.0.move_left();
    }

    pub fn move_right(&mut self) {
        self.0.move_right();
    }

    pub fn move_side(&mut self, side: Side) {
        self.0.move_side(side);
    }

    /// Moves the cursor to the inorder predecessor of the current node.
    /// If no predecessor exists, false is returned and the cursor is not moved.
    pub fn try_move_prev(&mut self) -> bool {
        self.0.try_move_prev()
    }

    /// Moves the cursor to the inorder successor of the current node.
    /// If no successor exists, false is returned and the cursor is not moved.
    pub fn try_move_next(&mut self) -> bool {
        self.0.try_move_next()
    }

    /// Moves the cursor to the inorder predecessor of the current node.
    /// If no predecessor exists, the cursor is moved to a "null" node.
    pub fn move_prev(&mut self) {
        self.0.move_prev();
    }

    /// Moves the cursor to the inorder successor of the current node.
    /// If no successor exists, the cursor is moved to a "null" node.
    pub fn move_next(&mut self) {
        self.0.move_next();
    }

    pub fn get(&self) -> Option<(&'t K, &'t V)> {
        self.0.get().map(BstNode::data)
    }

    pub fn side_of_parent(&self) -> Option<Side> {
        self.0.side_of_parent()
    }

    pub fn peek_up(&self) -> Option<(&'t K, &'t V)> {
        self.0.peek_up().map(BstNode::data)
    }

    pub fn peek_left(&self) -> Option<(&'t K, &'t V)> {
        self.0.peek_left().map(BstNode::data)
    }

    pub fn peek_right(&self) -> Option<(&'t K, &'t V)> {
        self.0.peek_right().map(BstNode::data)
    }

    pub fn peek_side(&self, side: Side) -> Option<(&'t K, &'t V)> {
        self.0.peek_side(side).map(BstNode::data)
    }

    pub fn peek_neighborhood(&self) -> Neighborhood<(&'t K, &'t V)> {
        self.0.peek_neighborhood().map(BstNode::data)
    }
}

/// A cursor over a BinarySearchTree with editing operations.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
/// Cursors maintain the invariant that as long as the tree has a node, the cursor points to a node.
#[derive(Debug)]
pub struct CursorMut<'t, K, V>(binary_tree::CursorMut<'t, BstNode<K, V>>);

impl<'t, K, V> CursorMut<'t, K, V> {
    pub(super) fn new(cursor: binary_tree::CursorMut<'t, BstNode<K, V>>) -> Self {
        Self(cursor)
    }
    
    pub fn try_move_up(&mut self) -> Option<Side> {
        self.0.try_move_up()
    }
    
    pub fn try_move_left(&mut self) -> bool {
        self.0.try_move_left()
    }
    
    pub fn try_move_right(&mut self) -> bool {
        self.0.try_move_right()
    }

    pub fn try_move_side(&mut self, side: Side) -> bool {
        self.0.try_move_side(side)
    }

    pub fn move_up(&mut self) -> Option<Side> {
        self.0.move_up()
    }

    pub fn move_left(&mut self) {
        self.0.move_left();
    }

    pub fn move_right(&mut self) {
        self.0.move_right();
    }

    pub fn move_side(&mut self, side: Side) {
        self.0.move_side(side);
    }

    /// Moves the cursor to the inorder predecessor of the current node.
    /// If no predecessor exists, false is returned and the cursor is not moved.
    pub fn try_move_prev(&mut self) -> bool {
        self.0.try_move_prev()
    }

    /// Moves the cursor to the inorder successor of the current node.
    /// If no successor exists, false is returned and the cursor is not moved.
    pub fn try_move_next(&mut self) -> bool {
        self.0.try_move_next()
    }

    /// Moves the cursor to the inorder predecessor of the current node.
    /// If no predecessor exists, the cursor is moved to a "null" node.
    pub fn move_prev(&mut self) {
        self.0.move_prev();
    }

    /// Moves the cursor to the inorder successor of the current node.
    /// If no successor exists, the cursor is moved to a "null" node.
    pub fn move_next(&mut self) {
        self.0.move_next();
    }
    
    pub fn get(&mut self) -> Option<(&K, &mut V)> {
        self.0.get().map(BstNode::data_with_mut_value)
    }

    pub fn side_of_parent(&self) -> Option<Side> {
        self.0.side_of_parent()
    }

    pub fn peek_up(&mut self) -> Option<(&K, &mut V)> {
        self.0.peek_up().map(BstNode::data_with_mut_value)
    }

    pub fn peek_left(&mut self) -> Option<(&K, &mut  V)> {
        self.0.peek_left().map(BstNode::data_with_mut_value)
    }

    pub fn peek_right(&mut self) -> Option<(&K, &mut V)> {
        self.0.peek_right().map(BstNode::data_with_mut_value)
    }

    pub fn peek_side(&mut self, side: Side) -> Option<(&K, &mut V)> {
        self.0.peek_side(side).map(BstNode::data_with_mut_value)
    }

    pub fn peek_neighborhood(&mut self) -> Neighborhood<(&K, &mut V)> {
        self.0.peek_neighborhood().map(BstNode::data_with_mut_value)
    }

    /// Spawn N cursors and move them around the tree according to the supplied function.
    /// Reports mutable references to the nodes the cursors end up pointing at.
    /// Requires the cursors to end up pointing at distinct, existing nodes; else None is returned.
    pub fn spawn_and_peek<F, const N: usize>(&mut self, cursors_fn: F) -> Option<[(&K, &mut V); N]>
    where
        F: FnOnce(&mut [Cursor<'_, K, V>; N]),
    {
        // "Downgrade" cursors_fn to one that works on binary_tree::Cursor.
        let cursors_fn = |cursors: &mut [binary_tree::Cursor<'_, BstNode<K, V>>; N]| {
            let mut bst_cursors = std::array::from_fn(|i| Cursor(cursors[i]));
            cursors_fn(&mut bst_cursors);
            *cursors = bst_cursors.map(|cursor| cursor.0);
        };
        self.0.spawn_and_peek(cursors_fn)
            .map(|results| results.map(BstNode::data_with_mut_value))
    }

    /// Creates a new node and attaches it as a child to the node pointed at by the cursor.
    pub(super) fn attach_child(&mut self, key: K, value: V, side: Side) -> Result<(), CursorError> {
        self.0.attach_child(BstNode::new(key, value), side)
    }

    /// Detaches the child of the node pointed at by the cursor from the tree.
    /// The cursor stays in place.
    /// Does nothing if the child node is not a leaf.
    /// Returns the detached node.
    pub fn detach_child(&mut self, side: Side) -> Option<(K, V)> {
        self.0.detach_child(side).map(BstNode::into_data)
    }

    /// Detaches the node pointed at by the cursor from the tree, and moves the cursor up.
    /// Does nothing if the cursor does not point to a leaf.
    /// Returns the detached node.
    pub fn detach_node(&mut self) -> Option<(K, V)> {
        self.0.detach_node().map(BstNode::into_data)
    }

    /// Removes the node pointed at by the cursor from the tree, assuming the node has exactly one child.
    /// Replaces the node by its child, after which the cursor points to this child.
    /// Does nothing if the node has zero or two children.
    /// Returns the removed node.
    pub fn transplant_child(&mut self) -> Option<(K, V)> {
        self.0.transplant_child().map(BstNode::into_data)
    }

    /// Performs a left rotation around the node pointed at by the cursor.
    /// The cursor keeps pointing to the same node, which moves during rotation.
    pub fn rotate_left(&mut self) -> Result<(), CursorError> {
        self.0.rotate_left()
    }
    
    /// Performs a right rotation around the node pointed at by the cursor.
    /// The cursor keeps pointing to the same node, which moves during rotation.
    pub fn rotate_right(&mut self) -> Result<(), CursorError> {
        self.0.rotate_right()
    }

    pub fn rotate(&mut self, side: Side) -> Result<(), CursorError> {
        self.0.rotate(side)
    }
}
