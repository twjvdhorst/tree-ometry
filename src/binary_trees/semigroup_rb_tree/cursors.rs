use derive_more::Debug;

use super::{Color, SemigroupRbNode};
use crate::binary_trees::{
    Side, 
    binary_tree, 
    cursor_errors::CursorError, 
    semigroup_rb_tree::TreeSemigroup, 
    binary_tree_cursor::{
        Neighborhood,
        NeighborhoodMut,
        BinaryTreeCursor,
        PeekingCursor, 
        PeekingCursorMut,
    }
};

/// A cursor over a SemigroupRbTree.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
#[derive(Debug)]
pub struct Cursor<'t, K, V, S>(binary_tree::Cursor<'t, SemigroupRbNode<K, V, S>>);

impl<'t, K, V, S> Cursor<'t, K, V, S> {
    pub(super) fn new(cursor: binary_tree::Cursor<'t, SemigroupRbNode<K, V, S>>) -> Self {
        Self(cursor)
    }
}

impl<'t, K, V, S> Clone for Cursor<'t, K, V, S> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'t, K, V, S> Copy for Cursor<'t, K, V, S> {}

impl<'t, K, V, S> BinaryTreeCursor for Cursor<'t, K, V, S> {
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

impl<'t, K, V, S> PeekingCursor<'t> for Cursor<'t, K, V, S> {
    type Item = SemigroupRbNode<K, V, S>;

    fn get(&self) -> Option<&'t Self::Item> {
        self.0.get()
    }

    fn spawn_cursor(&self) -> Self {
        self.clone()
    }

    fn side_of_parent(&self) -> Option<crate::binary_trees::Side> {
        self.0.side_of_parent()
    }

    fn peek_up(&self) -> Option<&'t Self::Item> {
        self.0.peek_up()
    }

    fn peek_left(&self) -> Option<&'t Self::Item> {
        self.0.peek_left()
    }

    fn peek_right(&self) -> Option<&'t Self::Item> {
        self.0.peek_right()
    }

    fn peek_neighborhood(&self) -> Neighborhood<'t, Self::Item> {
        self.0.peek_neighborhood()
    }
}

/// A cursor over a SemigroupRbTree with editing operations.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
/// Cursors maintain the invariant that as long as the tree has a node, the cursor points to a node.
#[derive(Debug)]
pub struct CursorMut<'t, K, V, S>(binary_tree::CursorMut<'t, SemigroupRbNode<K, V, S>>);

impl<'t, K, V, S> CursorMut<'t, K, V, S> {
    pub(super) fn new(cursor: binary_tree::CursorMut<'t, SemigroupRbNode<K, V, S>>) -> Self {
        Self(cursor)
    }

    /// Spawn N cursors and move them around the tree according to the supplied function.
    /// Reports mutable references to the nodes the cursors end up pointing at.
    /// Requires the cursors to end up pointing at distinct, existing nodes; else None is returned.
    pub fn spawn_and_peek_mut<F, const N: usize>(&mut self, cursors_fn: F) -> Option<[&mut SemigroupRbNode<K, V, S>; N]>
    where
        F: FnOnce(&mut [Cursor<'_, K, V, S>; N]),
    {
        // "Downgrade" cursors_fn to one that works on binary_tree::Cursor.
        let cursors_fn = |cursors: &mut [binary_tree::Cursor<'_, SemigroupRbNode<K, V, S>>; N]| {
            let mut rb_cursors = std::array::from_fn(|i| Cursor(cursors[i]));
            cursors_fn(&mut rb_cursors);
            *cursors = rb_cursors.map(|cursor| cursor.0);
        };
        self.0.spawn_and_peek_mut(cursors_fn)
    }

    pub(super) fn set_color(&mut self, color: Color) {
        if let Some(node) = self.get_mut() {
            node.set_color(color);
        }
    }

    /// Removes the node pointed at by the cursor from the tree, assuming the node has exactly one child.
    /// Replaces the node by its child, after which the cursor points to this child.
    /// Does nothing if the node has zero or two children.
    /// Returns the removed node.
    pub(super) fn transplant_child(&mut self) -> Option<(K, V)> {
        // No need to fix semigroup values for the cursor node, as the subtree of the child is unchanged.
        self.0.transplant_child().map(Into::into)
    }
}

impl<'t, K, V, S> CursorMut<'t, K, V, S>
where 
    S: TreeSemigroup<K>,
{
    pub(super) fn recompute_semigroup_value(&mut self) {
        let new_semigroup_value = {
            let Some(node) = self.get() else { return; };
            let Neighborhood { left, right, .. } = self.peek_neighborhood();
            S::op(
                node.key(),
                left.map(SemigroupRbNode::semigroup_value),
                right.map(SemigroupRbNode::semigroup_value),
            )
        };
        self.get_mut().unwrap().set_semigroup_value(new_semigroup_value);
    }

    pub(super) fn move_up_and_recompute_semigroup_value(&mut self) -> Option<Side> {
        let side = self.move_up()?;
        self.recompute_semigroup_value();
        Some(side)
    }

    
    pub(super) fn try_move_up_and_recompute_semigroup_value(&mut self) -> Option<Side> {
        let side = self.try_move_up()?;
        self.recompute_semigroup_value();
        Some(side)
    }

    /// Creates a new node and attaches it as a child to the node pointed at by the cursor.
    /// Ensures the subtree rooted at the cursor remains a valid semigroup tree.
    pub(super) fn attach_child(&mut self, node: SemigroupRbNode<K, V, S>, side: Side) -> Result<(), CursorError> {
        self.0.attach_child(node, side)?;
        self.recompute_semigroup_value();
        Ok(())
    }

    /// Detaches the node pointed at by the cursor from the tree, and moves the cursor up.
    /// Ensures the subtree rooted at the cursor remains a valid semigroup tree.
    /// Does nothing if the cursor does not point to a leaf.
    /// Returns the detached node.
    pub(super) fn detach_node(&mut self) -> Option<(K, V)> {
        let data = self.0.detach_node().map(Into::into)?;
        self.recompute_semigroup_value();
        Some(data)
    }

    /// Performs a tree rotation.
    /// The cursor keeps pointing to the node it originally pointed to.
    /// Ensures the subtree rooted at the cursor remains a valid semigroup tree.
    pub(super) fn rotate_and_fix_semigroup_value(&mut self, side: Side) -> Result<(), CursorError> {
        self.0.rotate(side)?;
        self.recompute_semigroup_value();
        Ok(())
    }
}

impl<'t, K, V, S> BinaryTreeCursor for CursorMut<'t, K, V, S> {
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

impl<'t, K, V, S> PeekingCursorMut for CursorMut<'t, K, V, S> {
    type Item = SemigroupRbNode<K, V, S>;
    type SpawnedCursor<'c> = Cursor<'c, K, V, S>
    where Self: 'c;

    fn get(&self) -> Option<&Self::Item> {
        self.0.get()
    }

    fn get_mut(&mut self) -> Option<&mut Self::Item> {
        self.0.get_mut()
    }

    fn spawn_cursor(&self) -> Self::SpawnedCursor<'_> {
        Cursor::new(self.0.spawn_cursor())
    }

    fn side_of_parent(&self) -> Option<crate::binary_trees::Side> {
        self.0.side_of_parent()
    }

    fn peek_up(&self) -> Option<&Self::Item> {
        self.0.peek_up()
    }

    fn peek_left(&self) -> Option<&Self::Item> {
        self.0.peek_left()
    }

    fn peek_right(&self) -> Option<&Self::Item> {
        self.0.peek_right()
    }

    fn peek_neighborhood(&self) -> Neighborhood<'_, Self::Item> {
        self.0.peek_neighborhood()
    }

    fn peek_neighborhood_mut(&mut self) -> NeighborhoodMut<'_, Self::Item> {
        self.0.peek_neighborhood_mut()
    }

    fn peek_up_mut(&mut self) -> Option<&mut Self::Item> {
        self.0.peek_up_mut()
    }

    fn peek_left_mut(&mut self) -> Option<&mut Self::Item> {
        self.0.peek_left_mut()
    }

    fn peek_right_mut(&mut self) -> Option<&mut Self::Item> {
        self.0.peek_right_mut()
    }
}
