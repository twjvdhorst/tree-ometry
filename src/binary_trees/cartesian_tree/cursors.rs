use derive_more::Debug;

use crate::binary_trees::{
    Side, binary_tree::{
        self, 
        BinaryTreeNode,
    }, cartesian_tree::CartesianTreeNode, cursor_errors::CursorError, traits::binary_tree_cursor::{
        BinaryTreeCursor, 
        BinaryTreeCursorMut,
    }
};

pub struct Neighborhood<'c, K, V> {
    pub parent: Option<&'c CartesianTreeNode<K, V>>,
    pub left: Option<&'c CartesianTreeNode<K, V>>,
    pub right: Option<&'c CartesianTreeNode<K, V>>,
}

/// A cursor over a SemigroupRbTree.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
#[derive(Debug)]
pub struct Cursor<'t, K, V>(binary_tree::Cursor<'t, CartesianTreeNode<K, V>>);

impl<'t, K, V> Cursor<'t, K, V> {
    pub(super) fn new(cursor: binary_tree::Cursor<'t, CartesianTreeNode<K, V>>) -> Self {
        Self(cursor)
    }

    pub fn peek_neighborhood(&self) -> Neighborhood<'_, K, V> {
        let binary_tree::Neighborhood { parent, left, right } = self.0.peek_neighborhood();
        Neighborhood {
            parent: parent.map(BinaryTreeNode::data),
            left: left.map(BinaryTreeNode::data),
            right: right.map(BinaryTreeNode::data),
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
    type Node = CartesianTreeNode<K, V>;
    type SpawnedCursor<'c> = Self
    where Self: 'c;

    fn node(&self) -> Option<&Self::Node> {
        self.0.node().map(BinaryTreeNode::data)
    }

    fn spawn_cursor(&self) -> Self::SpawnedCursor<'_> {
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

pub struct NeighborhoodMut<'c, K, V> {
    pub parent: Option<&'c mut CartesianTreeNode<K, V>>,
    pub left: Option<&'c mut CartesianTreeNode<K, V>>,
    pub right: Option<&'c mut CartesianTreeNode<K, V>>,
}

/// A cursor over a SemigroupRbTree with editing operations.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
/// Cursors maintain the invariant that as long as the tree has a node, the cursor points to a node.
#[derive(Debug)]
pub struct CursorMut<'t, K, V>(binary_tree::CursorMut<'t, CartesianTreeNode<K, V>>);

impl<'t, K, V> CursorMut<'t, K, V> {
    pub(super) fn new(cursor: binary_tree::CursorMut<'t, CartesianTreeNode<K, V>>) -> Self {
        Self(cursor)
    }

    pub fn peek_neighborhood(&self) -> Neighborhood<'_, K, V> {
        let binary_tree::Neighborhood { parent, left, right } = self.0.peek_neighborhood();
        Neighborhood {
            parent: parent.map(BinaryTreeNode::data),
            left: left.map(BinaryTreeNode::data),
            right: right.map(BinaryTreeNode::data),
        }
    }

    pub fn peek_neighborhood_mut(&mut self) -> NeighborhoodMut<'_, K, V> {
        let binary_tree::NeighborhoodMut { parent, left, right } = self.0.peek_neighborhood_mut();
        NeighborhoodMut {
            parent: parent.map(BinaryTreeNode::data_mut),
            left: left.map(BinaryTreeNode::data_mut),
            right: right.map(BinaryTreeNode::data_mut),
        }
    }

    /// Spawn N cursors and move them around the tree according to the supplied function.
    /// Reports mutable references to the nodes the cursors end up pointing at.
    /// Requires the cursors to end up pointing at distinct, existing nodes; else None is returned.
    pub fn spawn_and_peek_mut<F, const N: usize>(&mut self, cursors_fn: F) -> Option<[&mut CartesianTreeNode<K, V>; N]>
    where
        F: FnOnce(&mut [Cursor<'_, K, V>; N]),
    {
        // "Downgrade" cursors_fn to one that works on binary_tree::Cursor.
        let cursors_fn = |cursors: &mut [binary_tree::Cursor<'_, CartesianTreeNode<K, V>>; N]| {
            let mut rb_cursors = std::array::from_fn(|i| Cursor(cursors[i]));
            cursors_fn(&mut rb_cursors);
            *cursors = rb_cursors.map(|cursor| cursor.0);
        };
        self.0.spawn_and_peek_mut(cursors_fn).map(|nodes| nodes.map(BinaryTreeNode::data_mut))
    }

    pub(super) fn re_root_tree(&mut self, root: CartesianTreeNode<K, V>, side: Side) {
        self.0.re_root_tree(root, side);
    }

    pub(super) fn attach_or_insert_child(&mut self, node: CartesianTreeNode<K, V>, side: Side) -> Result<(), CursorError> {
        self.0.attach_or_insert_child(node, side)
    }

    pub fn swap_children(&mut self) -> Result<(), CursorError> {
        self.0.swap_children()
    }
}

impl<'t, K, V> BinaryTreeCursor for CursorMut<'t, K, V> {
    type Node = CartesianTreeNode<K, V>;
    type SpawnedCursor<'c> = Cursor<'c, K, V>
    where Self: 'c;

    fn node(&self) -> Option<&Self::Node> {
        self.0.node().map(BinaryTreeNode::data)
    }

    fn spawn_cursor(&self) -> Self::SpawnedCursor<'_> {
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

impl<'t, K, V> BinaryTreeCursorMut for CursorMut<'t, K, V> {
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
}
