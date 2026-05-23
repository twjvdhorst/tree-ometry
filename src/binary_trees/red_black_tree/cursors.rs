use derive_more::Debug;

use super::Color;
use crate::binary_trees::{
    Neighborhood,
    Side,
    binary_tree,
    cursor_errors::CursorError,
    red_black_tree::RedBlackNode,
    binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursor,
        PeekingCursorMut,
    },
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

    pub(super) fn child_color(&self, side: Side) -> Option<Color> {
        Some(match side {
            Side::Left => self.0.peek_left()?.color(),
            Side::Right => self.0.peek_right()?.color(),
        })
    }
}

impl<'t, K, V> BinaryTreeCursor for Cursor<'t, K, V> {   
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

impl<'t, K, V> PeekingCursor for Cursor<'t, K, V> {
    type Item = (&'t K, &'t V);

    fn get(&self) -> Option<Self::Item> {
        self.0.get().map(RedBlackNode::data)
    }

    fn peek_up(&self) -> Option<Self::Item> {
        self.0.peek_up().map(RedBlackNode::data)
    }

    fn peek_left(&self) -> Option<Self::Item> {
        self.0.peek_left().map(RedBlackNode::data)
    }

    fn peek_right(&self) -> Option<Self::Item> {
        self.0.peek_right().map(RedBlackNode::data)
    }

    fn peek_neighborhood(&self) -> Neighborhood<Self::Item> {
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

    pub(super) fn node_color(&self) -> Option<Color> {
        self.0.get().map(RedBlackNode::color)
    }

    pub(super) fn parent_color(&self) -> Option<Color> {
        self.0.peek_up().map(RedBlackNode::color)
    }

    pub(super) fn left_color(&self) -> Option<Color> {
        self.0.peek_left().map(RedBlackNode::color)
    }

    pub(super) fn right_color(&self) -> Option<Color> {
        self.0.peek_right().map(RedBlackNode::color)
    }

    pub(super) fn child_color(&self, side: Side) -> Option<Color> {
        match side {
            Side::Left => self.left_color(),
            Side::Right => self.right_color(),
        }
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

    /// Spawn N cursors and move them around the tree according to the supplied function.
    /// Reports mutable references to the nodes the cursors end up pointing at.
    /// Requires the cursors to end up pointing at distinct, existing nodes; else None is returned.
    pub fn spawn_and_peek_mut<F, const N: usize>(&mut self, cursors_fn: F) -> Option<[&mut RedBlackNode<K, V>; N]>
    where
        F: FnOnce(&mut [Cursor<'_, K, V>; N]),
    {
        // "Downgrade" cursors_fn to one that works on binary_tree::Cursor.
        let cursors_fn = |cursors: &mut [binary_tree::Cursor<'_, RedBlackNode<K, V>>; N]| {
            let mut rb_cursors = std::array::from_fn(|i| Cursor(cursors[i]));
            cursors_fn(&mut rb_cursors);
            *cursors = rb_cursors.map(|cursor| cursor.0);
        };
        self.0.spawn_and_peek_mut(cursors_fn)
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

impl<'t, K, V> BinaryTreeCursor for CursorMut<'t, K, V> {   
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

impl<'t, K, V> PeekingCursorMut for CursorMut<'t, K, V> {
    type Item<'c> = (&'c K, &'c V) where Self: 'c;
    type ItemMut<'c> = (&'c K, &'c mut V) where Self: 'c;
    type AsCursor<'c> = Cursor<'c, K, V> where Self: 'c;

    fn get(&self) -> Option<Self::Item<'_>> {
        self.0.get().map(RedBlackNode::data)
    }

    fn get_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.get_mut().map(RedBlackNode::data_with_mut_value)
    }

    fn as_cursor(&self) -> Cursor<'_, K, V> {
        Cursor::new(self.0.as_cursor())
    }

    fn peek_up(&self) -> Option<Self::Item<'_>> {
        self.0.peek_up().map(RedBlackNode::data)
    }

    fn peek_left(&self) -> Option<Self::Item<'_>> {
        self.0.peek_left().map(RedBlackNode::data)
    }

    fn peek_right(&self) -> Option<Self::Item<'_>> {
        self.0.peek_right().map(RedBlackNode::data)
    }

    fn peek_neighborhood(&self) -> Neighborhood<Self::Item<'_>> {
        self.0.peek_neighborhood().map(RedBlackNode::data)
    }

    fn peek_up_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_up_mut().map(RedBlackNode::data_with_mut_value)
    }

    fn peek_left_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_left_mut().map(RedBlackNode::data_with_mut_value)
    }

    fn peek_right_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_right_mut().map(RedBlackNode::data_with_mut_value)
    }

    fn peek_neighborhood_mut(&mut self) -> Neighborhood<Self::ItemMut<'_>> {
        self.0.peek_neighborhood_mut().map(RedBlackNode::data_with_mut_value)
    }
}
