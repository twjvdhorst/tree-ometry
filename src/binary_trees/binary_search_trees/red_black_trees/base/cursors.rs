use super::RbNode;
use crate::binary_trees::{
    Neighborhood, Side, binary_search_trees::red_black_trees::Color, binary_tree, binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursor,
        PeekingCursorMut,
    }, cursor_errors::CursorError
};

/// A cursor over a RedBlackTree.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
pub(in crate::binary_trees::binary_search_trees::red_black_trees) struct Cursor<'t, T>(binary_tree::Cursor<'t, RbNode<T>>);

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

    pub(super) fn into_inner(self) -> binary_tree::Cursor<'t, RbNode<T>> {
        self.0
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
    type Item = &'t T;

    fn get(&self) -> Option<Self::Item> {
        self.0.get().map(RbNode::data)
    }

    fn peek_up(&self) -> Option<Self::Item> {
        self.0.peek_up().map(RbNode::data)
    }

    fn peek_left(&self) -> Option<Self::Item> {
        self.0.peek_left().map(RbNode::data)
    }

    fn peek_right(&self) -> Option<Self::Item> {
        self.0.peek_right().map(RbNode::data)
    }

    fn peek_neighborhood(&self) -> Neighborhood<Self::Item> {
        self.0.peek_neighborhood().map(RbNode::data)
    }
}

/// A cursor over a RedBlackTree with editing operations.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
pub(in crate::binary_trees::binary_search_trees::red_black_trees) struct CursorMut<'t, T>(binary_tree::CursorMut<'t, RbNode<T>>);

impl<'t, T> CursorMut<'t, T> {
    pub(super) fn new(cursor: binary_tree::CursorMut<'t, RbNode<T>>) -> Self {
        Self(cursor)
    }
    pub(super) fn into_inner(self) -> binary_tree::CursorMut<'t, RbNode<T>> {
        self.0
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

    pub(super) fn child_color(&self, side: Side) -> Option<Color> {
        match side {
            Side::Left => self.left_color(),
            Side::Right => self.right_color(),
        }
    }

    pub(super) fn uncle_color(&self) -> Option<Color> {
        let mut cursor = self.as_cursor();
        let side = cursor.move_up()?;
        cursor.0.peek_side(side.opposite()).map(RbNode::color)
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

    pub(super) fn move_up_after_subtree_change<F>(&mut self, mut on_subtree_change: F) -> Option<Side>
    where 
        F: FnMut(&mut Self),
    {
        let side = self.move_up()?;
        on_subtree_change(self);
        Some(side)
    }

    pub(super) fn rotate<F>(&mut self, side: Side, mut on_subtree_change: F) -> Result<(), CursorError>
    where 
        F: FnMut(&mut Self),
    {
        self.0.rotate(side)?;
        on_subtree_change(self);
        Ok(())
    }

    pub(super) fn attach_child<F>(&mut self, node: RbNode<T>, side: Side, mut on_subtree_change: F) -> Result<(), CursorError>
    where 
        F: FnMut(&mut Self),
    {
        self.0.attach_child(node, side)?;
        on_subtree_change(self);
        Ok(())
    }

    pub(super) fn detach_node<F>(&mut self, mut on_subtree_change: F) -> Option<T>
    where 
        F: FnMut(&mut Self),
    {
        let data = self.0.detach_node()?.into_data();
        on_subtree_change(self);
        Some(data)
    }

    /// Removes the node pointed at by the cursor from the tree, assuming the node has exactly one child.
    /// Replaces the node by its child, after which the cursor points to this child.
    /// Does nothing if the node has zero or two children.
    /// Returns the removed node.
    pub(super) fn transplant_child(&mut self) -> Option<T> {
        // No need to use a callback function, as the child's subtree is not changed.
        self.0.transplant_child().map(RbNode::into_data)
    }

    /// Spawn N cursors and move them around the tree according to the supplied function.
    /// Reports mutable references to the nodes the cursors end up pointing at.
    /// Requires the cursors to end up pointing at distinct, existing nodes; else None is returned.
    pub(super) fn spawn_and_peek_nodes_mut<F, const N: usize>(&mut self, cursors_fn: F) -> Option<[&mut RbNode<T>; N]>
    where
        F: FnOnce(&mut [Cursor<'_, T>; N]),
    {
        // "Downgrade" cursors_fn to one that works on binary_tree::Cursor.
        let cursors_fn = |cursors: &mut [binary_tree::Cursor<'_, RbNode<T>>; N]| {
            let mut rb_cursors = std::array::from_fn(|i| Cursor(cursors[i]));
            cursors_fn(&mut rb_cursors);
            *cursors = rb_cursors.map(|cursor| cursor.0);
        };
        self.0.spawn_and_peek_mut(cursors_fn)
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
    type Item<'c> = &'c T where Self: 'c;
    type ItemMut<'c> = &'c mut T where Self: 'c;
    type AsCursor<'c> = Cursor<'c, T> where Self: 'c;

    fn get(&self) -> Option<Self::Item<'_>> {
        self.0.get().map(RbNode::data)
    }

    fn get_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.get_mut().map(RbNode::data_mut)
    }

    fn as_cursor(&self) -> Self::AsCursor<'_> {
        Cursor::new(self.0.as_cursor())
    }

    fn peek_up(&self) -> Option<Self::Item<'_>> {
        self.0.peek_up().map(RbNode::data)
    }

    fn peek_left(&self) -> Option<Self::Item<'_>> {
        self.0.peek_left().map(RbNode::data)
    }

    fn peek_right(&self) -> Option<Self::Item<'_>> {
        self.0.peek_right().map(RbNode::data)
    }

    fn peek_neighborhood(&self) -> Neighborhood<Self::Item<'_>> {
        self.0.peek_neighborhood().map(RbNode::data)
    }

    fn peek_up_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_up_mut().map(RbNode::data_mut)
    }

    fn peek_left_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_left_mut().map(RbNode::data_mut)
    }

    fn peek_right_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_right_mut().map(RbNode::data_mut)
    }

    fn peek_neighborhood_mut(&mut self) -> Neighborhood<Self::ItemMut<'_>> {
        self.0.peek_neighborhood_mut().map(RbNode::data_mut)
    }
}
