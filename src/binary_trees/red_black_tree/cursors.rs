use derive_more::Debug;

use super::RedBlackNode;
use crate::binary_trees::{
    Side, 
    semigroup_rb_tree, 
    traits::binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursor,
        PeekingCursorMut,
    }
};

pub struct Neighborhood<'c, K, V> {
    pub node: Option<&'c RedBlackNode<K, V>>,
    pub parent: Option<&'c RedBlackNode<K, V>>,
    pub left: Option<&'c RedBlackNode<K, V>>,
    pub right: Option<&'c RedBlackNode<K, V>>,
}

/// A cursor over a RedBlackTree.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
#[derive(Debug)]
pub struct Cursor<'t, K, V>(semigroup_rb_tree::Cursor<'t, K, V, ()>);

impl<'t, K, V> Cursor<'t, K, V> {
    pub(super) fn new(cursor: semigroup_rb_tree::Cursor<'t, K, V, ()>) -> Self {
        Self(cursor)
    }

    pub fn peek_neighborhood(&self) -> Neighborhood<'t, K, V> {
        let semigroup_rb_tree::Neighborhood { node, parent, left, right } = self.0.peek_neighborhood();
        Neighborhood {
            node: node.map(AsRef::as_ref),
            parent: parent.map(AsRef::as_ref),
            left: left.map(AsRef::as_ref),
            right: right.map(AsRef::as_ref),
        }
    }
}

impl<'t, K, V> Clone for Cursor<'t, K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'t, K, V> Copy for Cursor<'t, K, V> {}

impl<'t, K, V> BinaryTreeCursor for Cursor<'t, K, V> {
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

impl<'t, K, V> PeekingCursor<'t> for Cursor<'t, K, V> {
    type Node = RedBlackNode<K, V>;
    type SpawnedCursor<'c> = Cursor<'c, K, V>
    where Self: 'c;

    fn node(&self) -> Option<&'t Self::Node> {
       self.0.node().map(AsRef::as_ref)
    }

    fn spawn_cursor(&self) -> Self::SpawnedCursor<'_> {
        self.clone()
    }

    fn side_of_parent(&self) -> Option<crate::binary_trees::Side> {
        self.0.side_of_parent()
    }

    fn peek_up(&self) -> Option<&'t Self::Node> {
        self.0.peek_up().map(AsRef::as_ref)
    }

    fn peek_left(&self) -> Option<&'t Self::Node> {
        self.0.peek_left().map(AsRef::as_ref)
    }

    fn peek_right(&self) -> Option<&'t Self::Node> {
        self.0.peek_right().map(AsRef::as_ref)
    }
}

pub struct NeighborhoodMut<'c, K, V> {
    pub node: Option<&'c mut RedBlackNode<K, V>>,
    pub parent: Option<&'c mut RedBlackNode<K, V>>,
    pub left: Option<&'c mut RedBlackNode<K, V>>,
    pub right: Option<&'c mut RedBlackNode<K, V>>,
}

/// A cursor over a RedBlackTree with editing operations.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
/// Cursors maintain the invariant that as long as the tree has a node, the cursor points to a node.
#[derive(Debug)]
pub struct CursorMut<'t, K, V>(semigroup_rb_tree::CursorMut<'t, K, V, ()>);

impl<'t, K, V> CursorMut<'t, K, V> {
    pub(super) fn new(cursor: semigroup_rb_tree::CursorMut<'t, K, V, ()>) -> Self {
        Self(cursor)
    }

    pub fn peek_neighborhood(&self) -> Neighborhood<'_, K, V> {
        let semigroup_rb_tree::Neighborhood { node, parent, left, right } = self.0.peek_neighborhood();
        Neighborhood {
            node: node.map(AsRef::as_ref),
            parent: parent.map(AsRef::as_ref),
            left: left.map(AsRef::as_ref),
            right: right.map(AsRef::as_ref),
        }
    }

    pub fn peek_neighborhood_mut(&mut self) -> NeighborhoodMut<'_, K, V> {
        let semigroup_rb_tree::NeighborhoodMut { node, parent, left, right } = self.0.peek_neighborhood_mut();
        NeighborhoodMut {
            node: node.map(AsMut::as_mut),
            parent: parent.map(AsMut::as_mut),
            left: left.map(AsMut::as_mut),
            right: right.map(AsMut::as_mut),
        }
    }

    /// Spawn N cursors and move them around the tree according to the supplied function.
    /// Reports mutable references to the nodes the cursors end up pointing at.
    /// Requires the cursors to end up pointing at distinct, existing nodes; else None is returned.
    pub fn spawn_and_peek_mut<F, const N: usize>(&mut self, cursors_fn: F) -> Option<[&mut RedBlackNode<K, V>; N]>
    where
        F: FnOnce(&mut [Cursor<'_, K, V>; N]),
    {
        // "Upgrade" cursors_fn to one that works on semigroup_rb_tree::Cursor.
        let cursors_fn = |cursors: &mut [semigroup_rb_tree::Cursor<'_, K, V, ()>; N]| {
            let mut rb_cursors = std::array::from_fn(|i| Cursor(cursors[i]));
            cursors_fn(&mut rb_cursors);
            *cursors = rb_cursors.map(|cursor| cursor.0);
        };
        self.0.spawn_and_peek_mut(cursors_fn).map(|nodes| nodes.map(AsMut::as_mut))
    }
}

impl<'t, K, V> BinaryTreeCursor for CursorMut<'t, K, V> {
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
    type Node = RedBlackNode<K, V>;
    type SpawnedCursor<'c> = Cursor<'c, K, V>
    where Self: 'c;

    fn node(&self) -> Option<&Self::Node> {
        self.0.node().map(AsRef::as_ref)
    }

    fn node_mut(&mut self) -> Option<&mut Self::Node> {
        self.0.node_mut().map(AsMut::as_mut)
    }

    fn spawn_cursor(&self) -> Self::SpawnedCursor<'_> {
        Cursor::new(self.0.spawn_cursor())
    }

    fn side_of_parent(&self) -> Option<crate::binary_trees::Side> {
        self.0.side_of_parent()
    }

    fn peek_up(&self) -> Option<&Self::Node> {
        self.0.peek_up().map(AsRef::as_ref)
    }

    fn peek_left(&self) -> Option<&Self::Node> {
        self.0.peek_left().map(AsRef::as_ref)
    }

    fn peek_right(&self) -> Option<&Self::Node> {
        self.0.peek_right().map(AsRef::as_ref)
    }

    fn peek_up_mut(&mut self) -> Option<&mut Self::Node> {
        self.0.peek_up_mut().map(AsMut::as_mut)
    }

    fn peek_left_mut(&mut self) -> Option<&mut Self::Node> {
        self.0.peek_left_mut().map(AsMut::as_mut)
    }

    fn peek_right_mut(&mut self) -> Option<&mut Self::Node> {
        self.0.peek_right_mut().map(AsMut::as_mut)
    }
}
