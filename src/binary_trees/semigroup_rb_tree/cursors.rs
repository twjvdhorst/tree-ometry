use derive_more::Debug;

use super::{Color, SemigroupRbNode};
use crate::binary_trees::{
    Side, binary_tree::{self, BinaryTreeNode}, cursor_errors::CursorError, semigroup_rb_tree::TreeSemigroup, traits::binary_tree_cursor::{BinaryTreeCursor, BinaryTreeCursorMut}
};

/// A cursor over a SemigroupRbTree.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
#[derive(Debug)]
pub struct Cursor<'tree, K, V, S>(binary_tree::Cursor<'tree, SemigroupRbNode<K, V, S>>);

impl<'tree, K, V, S> Cursor<'tree, K, V, S> {
    pub(super) fn new(cursor: binary_tree::Cursor<'tree, SemigroupRbNode<K, V, S>>) -> Self {
        Self(cursor)
    }
}

impl<'tree, K, V, S> Clone for Cursor<'tree, K, V, S> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'tree, K, V, S> Copy for Cursor<'tree, K, V, S> {}

impl<'tree, K, V, S> BinaryTreeCursor for Cursor<'tree, K, V, S> {
    type Node = SemigroupRbNode<K, V, S>;
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

/// A cursor over a SemigroupRbTree with editing operations.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
/// Cursors maintain the invariant that as long as the tree has a node, the cursor points to a node.
#[derive(Debug)]
pub struct CursorMut<'tree, K, V, S>(binary_tree::CursorMut<'tree, SemigroupRbNode<K, V, S>>);

impl<'tree, K, V, S> CursorMut<'tree, K, V, S> {
    pub(super) fn new(cursor: binary_tree::CursorMut<'tree, SemigroupRbNode<K, V, S>>) -> Self {
        Self(cursor)
    }

    pub(super) fn color(&self) -> Option<Color> {
        self.node().map(SemigroupRbNode::color)
    }

    pub(super) fn set_color(&mut self, color: Color) {
        if let Some(node) = self.node_mut() {
            node.set_color(color);
        }
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
        self.0.spawn_and_peek_mut(cursors_fn).map(|nodes| nodes.map(BinaryTreeNode::data_mut))
    }

    /// Removes the node pointed at by the cursor from the tree, assuming the node has exactly one child.
    /// Replaces the node by its child, after which the cursor points to this child.
    /// Does nothing if the node has zero or two children.
    /// Returns the removed node.
    pub(super) fn transplant_child(&mut self) -> Option<(K, V)> {
        // No need to fix semigroup values for the cursor node, as the subtree of the child is unchanged.
        self.0.transplant_child().map(SemigroupRbNode::into_data)
    }
}

impl<'tree, K, V, S> CursorMut<'tree, K, V, S>
where 
    S: TreeSemigroup<K>,
{
    pub(super) fn recompute_semigroup_value(&mut self) {
        let new_semigroup_value = {
            let Some(node) = self.node() else { return; };
            let (left, right) = self.peek_both();
            S::op(
                node.key(),
                left.map(SemigroupRbNode::semigroup_value),
                right.map(SemigroupRbNode::semigroup_value),
            )
        };
        self.node_mut().unwrap().set_semigroup_value(new_semigroup_value);
    }

    pub(super) fn move_up_and_recompute_semigroup_value(&mut self) -> Option<Side> {
        let side = self.move_up()?;
        self.recompute_semigroup_value();
        Some(side)
    }

    /// Creates a new node and attaches it as a child to the node pointed at by the cursor.
    /// Recomputes the semigroup value for the cursor's node, but not of its ancestors.
    pub(super) fn attach_child(&mut self, node: SemigroupRbNode<K, V, S>, side: Side) -> Result<(), CursorError> {
        self.0.attach_child(node, side)?;
        self.recompute_semigroup_value();
        Ok(())
    }

    /// Detaches the node pointed at by the cursor from the tree, and moves the cursor up.
    /// Recomputes the semigroup value for the cursor's node, but not of its ancestors.
    /// Does nothing if the cursor does not point to a leaf.
    /// Returns the detached node.
    pub(super) fn detach_node(&mut self) -> Option<(K, V)> {
        let data = self.0.detach_node().map(SemigroupRbNode::into_data)?;
        self.recompute_semigroup_value();
        Some(data)
    }

    pub(super) fn rotate(&mut self, side: Side) -> Result<(), CursorError> {
        match side {
            Side::Left => {
                self.0.rotate_left()?;
                // Only the cursor node and its right child (now its parent) have their semigroup values changed.
                self.recompute_semigroup_value();
                self.move_up();
                self.recompute_semigroup_value();
                self.move_left();
            },
            Side::Right => {
                self.0.rotate_right()?;
                // Only the cursor node and its left child (now its parent) have their semigroup values changed.
                self.recompute_semigroup_value();
                self.move_up().unwrap();
                self.recompute_semigroup_value();
                self.move_right();
            },
        }
        Ok(())
    }
}

impl<'tree, K, V, S> BinaryTreeCursor for CursorMut<'tree, K, V, S> {
    type Node = SemigroupRbNode<K, V, S>;
    type Cursor<'c> = Cursor<'c, K, V, S>
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

impl<'tree, K, V, S> BinaryTreeCursorMut for CursorMut<'tree, K, V, S> {
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
