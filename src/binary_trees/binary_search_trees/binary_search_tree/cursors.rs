use super::{BstNode};
use crate::binary_trees::{
    Neighborhood,
    Side,
    binary_tree,
    cursor_errors::CursorError,
    binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursor,
        PeekingCursorMut,
    },
};

/// A cursor over a BinarySearchTree.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
pub struct Cursor<'t, K, V>(binary_tree::Cursor<'t, BstNode<K, V>>);

/// Make own implementation of Clone, so K and V don't have to be Clone.
impl<'t, K, V> Clone for Cursor<'t, K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'t, K, V> Copy for Cursor<'t, K, V> {}

impl<'t, K, V> From<CursorMut<'t, K, V>> for Cursor<'t, K, V> {
    fn from(value: CursorMut<'t, K, V>) -> Self {
        Self(value.0.into())
    }
}

impl<'t, K, V> Cursor<'t, K, V> {
    pub(super) fn new(cursor: binary_tree::Cursor<'t, BstNode<K, V>>) -> Self {
        Self(cursor)
    }

    pub(super) fn into_inner(self) -> binary_tree::Cursor<'t, BstNode<K, V>> {
        self.0
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
        self.0.get().map(BstNode::data)
    }

    fn peek_up(&self) -> Option<Self::Item> {
        self.0.peek_up().map(BstNode::data)
    }

    fn peek_left(&self) -> Option<Self::Item> {
        self.0.peek_left().map(BstNode::data)
    }

    fn peek_right(&self) -> Option<Self::Item> {
        self.0.peek_right().map(BstNode::data)
    }

    fn peek_neighborhood(&self) -> Neighborhood<Self::Item> {
        self.0.peek_neighborhood().map(BstNode::data)
    }
}

/// A cursor over a BinarySearchTree with editing operations.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
pub struct CursorMut<'t, K, V>(binary_tree::CursorMut<'t, BstNode<K, V>>);

impl<'t, K, V> CursorMut<'t, K, V> {
    pub(super) fn new(cursor: binary_tree::CursorMut<'t, BstNode<K, V>>) -> Self {
        Self(cursor)
    }

    pub(super) fn into_inner(self) -> binary_tree::CursorMut<'t, BstNode<K, V>> {
        self.0
    }

    /// Spawn N cursors and move them around the tree according to the supplied function.
    /// Reports mutable references to the data the cursors end up pointing at.
    /// Requires the cursors to end up pointing at distinct, existing nodes; else None is returned.
    pub fn spawn_and_peek_mut<F, const N: usize>(&mut self, cursors_fn: F) -> Option<[(&K, &mut V); N]>
    where
        F: FnOnce(&mut [Cursor<'_, K, V>; N]),
    {
        // "Downgrade" cursors_fn to one that works on binary_tree::Cursor.
        let cursors_fn = |cursors: &mut [binary_tree::Cursor<'_, BstNode<K, V>>; N]| {
            let mut bst_cursors = std::array::from_fn(|i| Cursor(cursors[i]));
            cursors_fn(&mut bst_cursors);
            *cursors = bst_cursors.map(|cursor| cursor.0);
        };
        self.0.spawn_and_peek_mut(cursors_fn)
            .map(|arr| arr.map(BstNode::data_with_mut_value))
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
        self.0.get().map(BstNode::data)
    }

    fn get_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.get_mut().map(BstNode::data_with_mut_value)
    }

    fn as_cursor(&self) -> Self::AsCursor<'_> {
        Cursor::new(self.0.as_cursor())
    }

    fn peek_up(&self) -> Option<Self::Item<'_>> {
        self.0.peek_up().map(BstNode::data)
    }

    fn peek_left(&self) -> Option<Self::Item<'_>> {
        self.0.peek_left().map(BstNode::data)
    }

    fn peek_right(&self) -> Option<Self::Item<'_>> {
        self.0.peek_right().map(BstNode::data)
    }

    fn peek_neighborhood(&self) -> Neighborhood<Self::Item<'_>> {
        self.0.peek_neighborhood().map(BstNode::data)
    }

    fn peek_up_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_up_mut().map(BstNode::data_with_mut_value)
    }

    fn peek_left_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_left_mut().map(BstNode::data_with_mut_value)
    }

    fn peek_right_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_right_mut().map(BstNode::data_with_mut_value)
    }

    fn peek_neighborhood_mut(&mut self) -> Neighborhood<Self::ItemMut<'_>> {
        self.0.peek_neighborhood_mut().map(BstNode::data_with_mut_value)
    }
}
