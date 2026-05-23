use derive_more::Debug;

use super::Color;
use crate::binary_trees::{
    Neighborhood,
    Side,
    binary_tree,
    cursor_errors::CursorError,
    red_black_tree::RedBlackNode,
};

/// A cursor over a RedBlackTree.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
#[derive(Debug)]
pub struct Cursor<'t, K, V>(binary_tree::Cursor<'t, RedBlackNode<K, V>>);

/// Make own implementation of Clone, so K and V don't have to be Clone.
impl<'t, K, V> Clone for Cursor<'t, K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'t, K, V> Copy for Cursor<'t, K, V> {}

impl<'t, K, V> Cursor<'t, K, V> {
    pub(super) fn new(cursor: binary_tree::Cursor<'t, RedBlackNode<K, V>>) -> Self {
        Self(cursor)
    }

    pub(super) fn child(&self, side: Side) -> Option<&RedBlackNode<K, V>> {
        match side {
            Side::Left => self.0.peek_left(),
            Side::Right => self.0.peek_right(),
        }
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
    
    pub fn get(&self) -> Option<(&'t K, &'t V)> {
        self.0.get().map(RedBlackNode::data)
    }

    pub fn side_of_parent(&self) -> Option<crate::binary_trees::Side> {
        self.0.side_of_parent()
    }

    pub fn peek_up(&self) -> Option<(&'t K, &'t V)> {
        self.0.peek_up().map(RedBlackNode::data)
    }

    pub fn peek_left(&self) -> Option<(&'t K, &'t V)> {
        self.0.peek_left().map(RedBlackNode::data)
    }

    pub fn peek_right(&self) -> Option<(&'t K, &'t V)> {
        self.0.peek_right().map(RedBlackNode::data)
    }

    pub fn peek_side(&self, side: Side) -> Option<(&'t K, &'t V)> {
        self.0.peek_side(side).map(RedBlackNode::data)
    }

    pub fn peek_neighborhood(&self) -> Neighborhood<(&'t K, &'t V)> {
        self.0.peek_neighborhood().map(RedBlackNode::data)
    }
}

/// A cursor over a RedBlackTree with editing operations.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
/// Cursors maintain the invariant that as long as the tree has a node, the cursor points to a node.
#[derive(Debug)]
pub struct CursorMut<'t, K, V>(binary_tree::CursorMut<'t, RedBlackNode<K, V>>);

impl<'t, K, V> CursorMut<'t, K, V> {
    pub(super) fn new(cursor: binary_tree::CursorMut<'t, RedBlackNode<K, V>>) -> Self {
        Self(cursor)
    }

    pub(super) fn set_color(&mut self, color: Color) {
        if let Some(node) = self.0.get() {
            node.set_color(color);
        }
    }

    pub(super) fn node(&mut self) -> Option<&mut RedBlackNode<K, V>> {
        self.0.get()
    }

    pub(super) fn parent(&mut self) -> Option<&mut RedBlackNode<K, V>> {
        self.0.peek_up()
    }

    pub(super) fn left(&mut self) -> Option<&mut RedBlackNode<K, V>> {
        self.0.peek_left()
    }

    pub(super) fn right(&mut self) -> Option<&mut RedBlackNode<K, V>> {
        self.0.peek_right()
    }

    pub(super) fn child(&mut self, side: Side) -> Option<&mut RedBlackNode<K, V>> {
        match side {
            Side::Left => self.left(),
            Side::Right => self.right(),
        }
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
    
    pub fn get(&mut self) -> Option<(&K, &mut V)> {
        self.0.get().map(RedBlackNode::data_with_mut_value)
    }

    pub fn as_cursor(&self) -> Cursor<'_, K, V> {
        Cursor::new(self.0.as_cursor())
    }

    pub fn side_of_parent(&self) -> Option<crate::binary_trees::Side> {
        self.0.side_of_parent()
    }

    pub fn peek_up(&mut self) -> Option<(&K, &mut V)> {
        self.0.peek_up().map(RedBlackNode::data_with_mut_value)
    }

    pub fn peek_left(&mut self) -> Option<(&K, &mut V)> {
        self.0.peek_left().map(RedBlackNode::data_with_mut_value)
    }

    pub fn peek_right(&mut self) -> Option<(&K, &mut V)> {
        self.0.peek_right().map(RedBlackNode::data_with_mut_value)
    }

    pub fn peek_side(&mut self, side: Side) -> Option<(&K, &mut V)> {
        self.0.peek_side(side).map(RedBlackNode::data_with_mut_value)
    }

    pub fn peek_neighborhood(&mut self) -> Neighborhood<(&K, &mut V)> {
        self.0.peek_neighborhood().map(RedBlackNode::data_with_mut_value)
    }
    
    /// Spawn N cursors and move them around the tree according to the supplied function.
    /// Reports mutable references to the nodes the cursors end up pointing at.
    /// Requires the cursors to end up pointing at distinct, existing nodes; else None is returned.
    pub fn spawn_and_peek<F, const N: usize>(&mut self, cursors_fn: F) -> Option<[&mut RedBlackNode<K, V>; N]>
    where
        F: FnOnce(&mut [Cursor<'_, K, V>; N]),
    {
        // "Downgrade" cursors_fn to one that works on binary_tree::Cursor.
        let cursors_fn = |cursors: &mut [binary_tree::Cursor<'_, RedBlackNode<K, V>>; N]| {
            let mut rb_cursors = std::array::from_fn(|i| Cursor(cursors[i]));
            cursors_fn(&mut rb_cursors);
            *cursors = rb_cursors.map(|cursor| cursor.0);
        };
        self.0.spawn_and_peek(cursors_fn)
    }

    /// Removes the node pointed at by the cursor from the tree, assuming the node has exactly one child.
    /// Replaces the node by its child, after which the cursor points to this child.
    /// Does nothing if the node has zero or two children.
    /// Returns the removed node.
    pub(super) fn transplant_child(&mut self) -> Option<(K, V)> {
        // No need to fix semigroup values for the cursor node, as the subtree of the child is unchanged.
        self.0.transplant_child().map(Into::into)
    }

    /// Creates a new node and attaches it as a child to the node pointed at by the cursor.
    pub(super) fn attach_child(&mut self, node: RedBlackNode<K, V>, side: Side) -> Result<(), CursorError> {
        self.0.attach_child(node, side)
    }

    /// Detaches the node pointed at by the cursor from the tree, and moves the cursor up.
    /// Does nothing if the cursor does not point to a leaf.
    /// Returns the detached node.
    pub(super) fn detach_node(&mut self) -> Option<(K, V)> {
        self.0.detach_node().map(Into::into)
    }

    /// Performs a tree rotation.
    /// The cursor keeps pointing to the node it originally pointed to.
    pub(super) fn rotate(&mut self, side: Side) -> Result<(), CursorError> {
        self.0.rotate(side)
    }
}
