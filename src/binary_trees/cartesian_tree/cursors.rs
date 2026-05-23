use derive_more::Debug;

use crate::binary_trees::{
    Neighborhood,
    Side,
    binary_tree,
    cartesian_tree::CartesianNode,
    cursor_errors::CursorError,
    binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursor,
        PeekingCursorMut,
    },
};

/// A cursor over a SemigroupRbTree.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
#[derive(Debug)]
pub struct Cursor<'t, K, V>(binary_tree::Cursor<'t, CartesianNode<K, V>>);

/// Make own implementation of Clone, so K and V don't have to be Clone.
impl<'t, K, V> Clone for Cursor<'t, K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'t, K, V> Copy for Cursor<'t, K, V> {}

impl<'t, K, V> Cursor<'t, K, V> {
    pub(super) fn new(cursor: binary_tree::Cursor<'t, CartesianNode<K, V>>) -> Self {
        Self(cursor)
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
        self.0.get().map(CartesianNode::data)
    }

    fn peek_up(&self) -> Option<Self::Item> {
        self.0.peek_up().map(CartesianNode::data)
    }

    fn peek_left(&self) -> Option<Self::Item> {
        self.0.peek_left().map(CartesianNode::data)
    }

    fn peek_right(&self) -> Option<Self::Item> {
        self.0.peek_right().map(CartesianNode::data)
    }

    fn peek_neighborhood(&self) -> Neighborhood<Self::Item> {
        self.0.peek_neighborhood().map(CartesianNode::data)
    }
}

/// A cursor over a SemigroupRbTree with editing operations.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
/// Cursors maintain the invariant that as long as the tree has a node, the cursor points to a node.
#[derive(Debug)]
pub struct CursorMut<'t, K, V>(binary_tree::CursorMut<'t, CartesianNode<K, V>>);

impl<'t, K, V> CursorMut<'t, K, V> {
    pub(super) fn new(cursor: binary_tree::CursorMut<'t, CartesianNode<K, V>>) -> Self {
        Self(cursor)
    }
    
    /// Spawn N cursors and move them around the tree according to the supplied function.
    /// Reports mutable references to the nodes the cursors end up pointing at.
    /// Requires the cursors to end up pointing at distinct, existing nodes; else None is returned.
    pub fn spawn_and_peek_mut<F, const N: usize>(&mut self, cursors_fn: F) -> Option<[&mut CartesianNode<K, V>; N]>
    where
        F: FnOnce(&mut [Cursor<'_, K, V>; N]),
    {
        // "Downgrade" cursors_fn to one that works on binary_tree::Cursor.
        let cursors_fn = |cursors: &mut [binary_tree::Cursor<'_, CartesianNode<K, V>>; N]| {
            let mut cart_cursors = std::array::from_fn(|i| Cursor(cursors[i]));
            cursors_fn(&mut cart_cursors);
            *cursors = cart_cursors.map(|cursor| cursor.0);
        };
        self.0.spawn_and_peek_mut(cursors_fn)
    }

    pub(super) fn re_root_tree(&mut self, root: CartesianNode<K, V>, side: Side) {
        self.0.re_root_tree(root, side);
    }

    pub(super) fn attach_or_insert_child(&mut self, node: CartesianNode<K, V>, side: Side) -> Result<(), CursorError> {
        self.0.attach_or_insert_child(node, side)
    }

    pub(super) fn swap_children(&mut self) -> Result<(), CursorError> {
        self.0.swap_children()
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
        self.0.get().map(CartesianNode::data)
    }

    fn get_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.get_mut().map(CartesianNode::data_with_mut_value)
    }

    fn as_cursor(&self) -> Cursor<'_, K, V> {
        Cursor::new(self.0.as_cursor())
    }

    fn peek_up(&self) -> Option<Self::Item<'_>> {
        self.0.peek_up().map(CartesianNode::data)
    }

    fn peek_left(&self) -> Option<Self::Item<'_>> {
        self.0.peek_left().map(CartesianNode::data)
    }

    fn peek_right(&self) -> Option<Self::Item<'_>> {
        self.0.peek_right().map(CartesianNode::data)
    }

    fn peek_neighborhood(&self) -> Neighborhood<Self::Item<'_>> {
        self.0.peek_neighborhood().map(CartesianNode::data)
    }

    fn peek_up_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_up_mut().map(CartesianNode::data_with_mut_value)
    }

    fn peek_left_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_left_mut().map(CartesianNode::data_with_mut_value)
    }

    fn peek_right_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_right_mut().map(CartesianNode::data_with_mut_value)
    }

    fn peek_neighborhood_mut(&mut self) -> Neighborhood<Self::ItemMut<'_>> {
        self.0.peek_neighborhood_mut().map(CartesianNode::data_with_mut_value)
    }
}
