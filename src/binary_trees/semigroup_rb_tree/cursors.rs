use derive_more::Debug;

use super::{Color, SemigroupRbNode};
use crate::binary_trees::{
    Neighborhood,
    Side, 
    binary_tree, 
    cursor_errors::CursorError, 
    semigroup_rb_tree::TreeSemigroup,
};

/// A cursor over a SemigroupRbTree.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
#[derive(Debug)]
pub struct Cursor<'t, K, V, S>(binary_tree::Cursor<'t, SemigroupRbNode<K, V, S>>);

/// Make own implementation of Clone, so K, V, and S don't have to be Clone.
impl<'t, K, V, S> Clone for Cursor<'t, K, V, S> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'t, K, V, S> Copy for Cursor<'t, K, V, S> {}

impl<'t, K, V, S> Cursor<'t, K, V, S> {
    pub(super) fn new(cursor: binary_tree::Cursor<'t, SemigroupRbNode<K, V, S>>) -> Self {
        Self(cursor)
    }
    
    pub(super) fn child(&self, side: Side) -> Option<&SemigroupRbNode<K, V, S>> {
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
    
    pub fn get(&self) -> Option<(&'t K, &'t V, &'t S)> {
        self.0.get().map(SemigroupRbNode::data)
    }

    pub fn side_of_parent(&self) -> Option<crate::binary_trees::Side> {
        self.0.side_of_parent()
    }

    pub fn peek_up(&self) -> Option<(&'t K, &'t V, &'t S)> {
        self.0.peek_up().map(SemigroupRbNode::data)
    }

    pub fn peek_left(&self) -> Option<(&'t K, &'t V, &'t S)> {
        self.0.peek_left().map(SemigroupRbNode::data)
    }

    pub fn peek_right(&self) -> Option<(&'t K, &'t V, &'t S)> {
        self.0.peek_right().map(SemigroupRbNode::data)
    }

    pub fn peek_side(&self, side: Side) -> Option<(&'t K, &'t V, &'t S)> {
        self.0.peek_side(side).map(SemigroupRbNode::data)
    }

    pub fn peek_neighborhood(&self) -> Neighborhood<(&'t K, &'t V, &'t S)> {
        self.0.peek_neighborhood().map(SemigroupRbNode::data)
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

    pub(super) fn set_color(&mut self, color: Color) {
        if let Some(node) = self.0.get() {
            node.set_color(color);
        }
    }

    pub(super) fn set_semigroup_value(&mut self, semigroup_value: S) {
        if let Some(node) = self.0.get() {
            node.set_semigroup_value(semigroup_value);
        }
    }

    pub(super) fn node(&mut self) -> Option<&mut SemigroupRbNode<K, V, S>> {
        self.0.get()
    }

    pub(super) fn parent(&mut self) -> Option<&mut SemigroupRbNode<K, V, S>> {
        self.0.peek_up()
    }

    pub(super) fn left(&mut self) -> Option<&mut SemigroupRbNode<K, V, S>> {
        self.0.peek_left()
    }

    pub(super) fn right(&mut self) -> Option<&mut SemigroupRbNode<K, V, S>> {
        self.0.peek_right()
    }

    pub(super) fn child(&mut self, side: Side) -> Option<&mut SemigroupRbNode<K, V, S>> {
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
    
    pub fn get(&mut self) -> Option<(&K, &mut V, &S)> {
        self.0.get().map(SemigroupRbNode::data_with_mut_value)
    }

    pub fn as_cursor(&self) -> Cursor<'_, K, V, S> {
        Cursor::new(self.0.as_cursor())
    }

    pub fn side_of_parent(&self) -> Option<crate::binary_trees::Side> {
        self.0.side_of_parent()
    }

    pub fn peek_up(&mut self) -> Option<(&K, &mut V, &S)> {
        self.0.peek_up().map(SemigroupRbNode::data_with_mut_value)
    }

    pub fn peek_left(&mut self) -> Option<(&K, &mut V, &S)> {
        self.0.peek_left().map(SemigroupRbNode::data_with_mut_value)
    }

    pub fn peek_right(&mut self) -> Option<(&K, &mut V, &S)> {
        self.0.peek_right().map(SemigroupRbNode::data_with_mut_value)
    }

    pub fn peek_side(&mut self, side: Side) -> Option<(&K, &mut V, &S)> {
        self.0.peek_side(side).map(SemigroupRbNode::data_with_mut_value)
    }

    pub fn peek_neighborhood(&mut self) -> Neighborhood<(&K, &mut V, &S)> {
        self.0.peek_neighborhood().map(SemigroupRbNode::data_with_mut_value)
    }

    /// Spawn N cursors and move them around the tree according to the supplied function.
    /// Reports mutable references to the nodes the cursors end up pointing at.
    /// Requires the cursors to end up pointing at distinct, existing nodes; else None is returned.
    pub fn spawn_and_peek<F, const N: usize>(&mut self, cursors_fn: F) -> Option<[&mut SemigroupRbNode<K, V, S>; N]>
    where
        F: FnOnce(&mut [Cursor<'_, K, V, S>; N]),
    {
        // "Downgrade" cursors_fn to one that works on binary_tree::Cursor.
        let cursors_fn = |cursors: &mut [binary_tree::Cursor<'_, SemigroupRbNode<K, V, S>>; N]| {
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
}

impl<'t, K, V, S> CursorMut<'t, K, V, S>
where 
    S: TreeSemigroup<K>,
{
    pub(super) fn recompute_semigroup_value(&mut self) {
        let new_semigroup_value = {
            //let Some(node) = self.get() else { return; };
            let Neighborhood { node: Some((key, ..)), left, right, .. } = self.peek_neighborhood() else { return; };
            S::op(
                key,
                left.map(|(.., s)| s),
                right.map(|(.., s)| s),
            )
        };
        self.node().unwrap().set_semigroup_value(new_semigroup_value);
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
