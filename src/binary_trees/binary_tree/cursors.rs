use derive_more::Debug;
use slotmap::Key;

use super::{BinaryTree, BinaryTreeNode, NodeId};
use crate::binary_trees::{
    Side,
    cursor_errors::CursorError,
    traits::binary_tree_cursor::{
        BinaryTreeCursor, PeekingCursor, PeekingCursorMut
    },
};

pub struct Neighborhood<'c, T> {
    pub node: Option<&'c BinaryTreeNode<T>>,
    pub parent: Option<&'c BinaryTreeNode<T>>,
    pub left: Option<&'c BinaryTreeNode<T>>,
    pub right: Option<&'c BinaryTreeNode<T>>,
}

/// A cursor over a BinaryTree.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
#[derive(Debug)]
pub struct Cursor<'t, T> {
    tree: &'t BinaryTree<T>,
    node_id: NodeId,
}

impl<'t, T> Cursor<'t, T> {
    pub(super) fn new(tree: &'t BinaryTree<T>, node_id: NodeId) -> Self {
        Self {
            tree,
            node_id,
        }
    }

    pub fn peek_neighborhood(&self) -> Neighborhood<'t, T> {
        Neighborhood {
            node: self.node(),
            parent: self.peek_up(),
            left: self.peek_left(),
            right: self.peek_right()
        }
    }
}

impl<'t, T> Clone for Cursor<'t, T> {
    fn clone(&self) -> Self {
        Self::new(self.tree, self.node_id)
    }
}

impl<'t, T> Copy for Cursor<'t, T> {}

impl<'t, T> BinaryTreeCursor for Cursor<'t, T> {
    fn try_move_up(&mut self) -> Option<Side> {
        let node_id = self.node_id;
        let parent_id = self.node()?.parent_id();
        if !parent_id.is_null() {
            self.node_id = parent_id;
            self.node()?.side_of(node_id)
        } else { None }
    }
    
    fn try_move_left(&mut self) -> bool {
        let left_id = self.node().map(BinaryTreeNode::left_id);
        if let Some(left_id) = left_id && !left_id.is_null() {
            self.node_id = left_id;
            true
        } else { false }
    }
    
    fn try_move_right(&mut self) -> bool {
        let right_id = self.node().map(BinaryTreeNode::right_id);
        if let Some(right_id) = right_id && !right_id.is_null() {
            self.node_id = right_id;
            true
        } else { false }
    }

    fn move_up(&mut self) -> Option<Side> {
        let node_id = self.node_id;
        self.node_id = self.node().map_or(NodeId::null(), BinaryTreeNode::parent_id);
        self.node().and_then(|parent| parent.side_of(node_id))
    }

    fn move_left(&mut self) {
        self.node_id = self.node().map_or(NodeId::null(), BinaryTreeNode::left_id);
    }

    fn move_right(&mut self) {
        self.node_id = self.node().map_or(NodeId::null(), BinaryTreeNode::right_id);
    }
}

impl<'t, T> PeekingCursor<'t> for Cursor<'t, T> {
    type Node = BinaryTreeNode<T>;
    type SpawnedCursor<'c> = Cursor<'c, T>
    where Self: 'c;

    fn node(&self) -> Option<&'t Self::Node> {
        self.tree.node(self.node_id)
    }

    fn spawn_cursor(&self) -> Self::SpawnedCursor<'_> {
        self.clone()
    }

    fn side_of_parent(&self) -> Option<Side> {
        self.peek_up()?.side_of(self.node_id)
    }

    fn peek_up(&self) -> Option<&'t Self::Node> {
        self.tree.parent(self.node()?)
    }

    fn peek_left(&self) -> Option<&'t Self::Node> {
        self.tree.left_child(self.node()?)
    }

    fn peek_right(&self) -> Option<&'t Self::Node> {
        self.tree.right_child(self.node()?)
    }
}

pub struct NeighborhoodMut<'c, T> {
    pub node: Option<&'c mut BinaryTreeNode<T>>,
    pub parent: Option<&'c mut BinaryTreeNode<T>>,
    pub left: Option<&'c mut BinaryTreeNode<T>>,
    pub right: Option<&'c mut BinaryTreeNode<T>>,
}

/// A cursor over a BinaryTree with editing operations.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
/// Cursors maintain the invariant that as long as the tree has a node, the cursor points to a node.
#[derive(Debug)]
pub struct CursorMut<'t, T> {
    tree: &'t mut BinaryTree<T>,
    node_id: NodeId,
}

impl<'t, T> CursorMut<'t, T> {
    pub(super) fn new(tree: &'t mut BinaryTree<T>, node_id: NodeId) -> Self {
        Self {
            tree,
            node_id,
        }
    }

    pub fn peek_neighborhood(&self) -> Neighborhood<'_, T> {
        Neighborhood {
            node: self.node(),
            parent: self.peek_up(),
            left: self.peek_left(),
            right: self.peek_right()
        }
    }

    pub fn peek_neighborhood_mut(&mut self) -> NeighborhoodMut<'_, T> {
        let Some(node) = self.node() else {
            return NeighborhoodMut {
                node: None,
                parent: None,
                left: None,
                right: None,
            };
        };

        match (node.parent_id(), node.left_id(), node.right_id()) {
            (parent_id, left_id, right_id) 
                if !parent_id.is_null() && !left_id.is_null() && !right_id.is_null() => 
            {
                let [node, parent, left, right] = unsafe { self.tree.get_disjoint_nodes_unchecked_mut([self.node_id, parent_id, left_id, right_id]) };
                NeighborhoodMut {
                    node: Some(node),
                    parent: Some(parent),
                    left: Some(left),
                    right: Some(right),
                }
            },
            (_, left_id, right_id) 
                if !left_id.is_null() && !right_id.is_null() => 
            {
                let [node, left, right] = unsafe { self.tree.get_disjoint_nodes_unchecked_mut([self.node_id, left_id, right_id]) };
                NeighborhoodMut {
                    node: Some(node),
                    parent: None,
                    left: Some(left),
                    right: Some(right),
                }
            },
            (parent_id, _, right_id) 
                if !parent_id.is_null() && !right_id.is_null() => 
            {
                let [node, parent, right] = unsafe { self.tree.get_disjoint_nodes_unchecked_mut([self.node_id, parent_id, right_id]) };
                NeighborhoodMut {
                    node: Some(node),
                    parent: Some(parent),
                    left: None,
                    right: Some(right),
                }
            },
            (parent_id, left_id, _) 
                if !parent_id.is_null() && !left_id.is_null() => 
            {
                let [node, parent, left] = unsafe { self.tree.get_disjoint_nodes_unchecked_mut([self.node_id, parent_id, left_id]) };
                NeighborhoodMut {
                    node: Some(node),
                    parent: Some(parent),
                    left: Some(left),
                    right: None,
                }
            },
            (parent_id, _, _) if !parent_id.is_null() => {
                let [node, parent] = unsafe { self.tree.get_disjoint_nodes_unchecked_mut([self.node_id, parent_id]) };
                NeighborhoodMut {
                    node: Some(node),
                    parent: Some(parent),
                    left: None,
                    right: None,
                }
            },
            (_, left_id, _) if !left_id.is_null() => {
                let [node, left] = unsafe { self.tree.get_disjoint_nodes_unchecked_mut([self.node_id, left_id]) };
                NeighborhoodMut {
                    node: Some(node),
                    parent: None,
                    left: Some(left),
                    right: None,
                }
            },
            (_, _, right_id) if !right_id.is_null() => {
                let [node, right] = unsafe { self.tree.get_disjoint_nodes_unchecked_mut([self.node_id, right_id]) };
                NeighborhoodMut {
                    node: Some(node),
                    parent: None,
                    left: None,
                    right: Some(right),
                }
            },
            _ => NeighborhoodMut { 
                    node: self.node_mut(),
                    parent: None, 
                    left: None, 
                    right: None 
                },
        }
    }

    /// Spawn N cursors and move them around the tree according to the supplied function.
    /// Reports mutable references to the nodes the cursors end up pointing at.
    /// Requires the cursors to end up pointing at distinct, existing nodes; else None is returned.
    pub fn spawn_and_peek_mut<F, const N: usize>(&mut self, cursors_fn: F) -> Option<[&mut BinaryTreeNode<T>; N]>
    where
        F: FnOnce(&mut [Cursor<'_, T>; N]),
    {
        let mut cursors = [self.spawn_cursor(); N];
        cursors_fn(&mut cursors);
        let ids = cursors.map(|cursor| cursor.node_id);
        self.tree.get_disjoint_nodes_mut(ids)
    }

    /// Creates a new root node, if the tree is empty.
    pub fn root_tree(&mut self, data: T) -> Result<(), CursorError> {
        if !self.tree.is_empty() {
            Err(CursorError::CreateRootError)
        } else {
            self.node_id = self.tree.new_node(data);
            Ok(())
        }
    }

    /// Creates a new root node.
    /// If the tree already had a root, the old tree is attached as a child subtree to the new root.
    /// Moves the cursor to the root of the new tree.
    pub fn re_root_tree(&mut self, data: T, side: Side) {
        let old_root_id = self.tree.root_id();
        let new_root_id = self.tree.new_node(data);
        self.tree.set_root_id(new_root_id);
        self.tree.add_edge(new_root_id, old_root_id, side);
        self.node_id = new_root_id;
    }

    /// Creates a new node and attaches the current node as a child to the new node.
    pub fn attach_parent(&mut self, data: T, side: Side) -> Result<(), CursorError> {
        let Some(node) = self.node() else { return Err(CursorError::NullError); };

        if node.has_parent() {
            return Err(CursorError::AttachParentError);
        }

        let curr_id = self.node_id;
        let new_id = self.tree.new_node(data);
        self.tree.add_edge(new_id, curr_id, side);
        Ok(())
    }

    /// Creates a new node and attaches it as a child to the node pointed at by the cursor.
    pub fn attach_child(&mut self, data: T, side: Side) -> Result<(), CursorError> {
        let Some(node) = self.node() else { return Err(CursorError::NullError); };

        if node.has_child(side) {
            return Err(CursorError::AttachChildError(side));
        }

        let curr_id = self.node_id;
        let new_id = self.tree.new_node(data);
        self.tree.add_edge(curr_id, new_id, side);
        Ok(())
    }

    /// Creates a new node and attaches the current node as a child to the new node.
    /// If the cursor already had a parent, the new node is "inserted" onto the edge.
    pub fn attach_or_insert_parent(&mut self, data: T, side: Side) -> Result<(), CursorError> {
        let Some(node) = self.node() else { return Err(CursorError::NullError); };

        let curr_id = self.node_id;
        let parent_id = node.parent_id();
        let new_id = self.tree.new_node(data);

        if !parent_id.is_null() {
            self.tree.remove_edge(parent_id, curr_id);
            self.tree.add_edge(parent_id, new_id, side);
        }
        self.tree.add_edge(new_id, curr_id, side);
        Ok(())
    }

    /// Creates a new node and attaches it as a child to the node pointed at by the cursor.
    /// If the cursor already had a child on the assigned side, the new node is "inserted" onto the edge.
    pub fn attach_or_insert_child(&mut self, data: T, side: Side) -> Result<(), CursorError> {
        let Some(node) = self.node() else { return Err(CursorError::NullError); };

        let curr_id = self.node_id;
        let child_id = node.child_id(side);
        let new_id = self.tree.new_node(data);

        if !child_id.is_null() {
            self.tree.remove_edge(curr_id, child_id);
            self.tree.add_edge(new_id, child_id, side);
        }
        self.tree.add_edge(curr_id, new_id, side);
        Ok(())
    }

    /// Detaches the child of the node pointed at by the cursor from the tree.
    /// The cursor stays in place.
    /// Does nothing if the child node is not a leaf.
    /// Returns the detached node.
    pub fn detach_child(&mut self, side: Side) -> Option<T> {
        let node_id = self.node_id;
        let child_id = self.node()?.child_id(side);
        let child_node = self.peek_side(side)?;
        if !child_node.has_left() && !child_node.has_right() {
            self.tree.remove_edge(node_id, child_id);
            self.tree.remove_node(child_id)
        } else {
            None
        }
    }

    /// Detaches the node pointed at by the cursor from the tree, and moves the cursor up.
    /// Does nothing if the cursor does not point to a leaf.
    /// Returns the detached node.
    pub fn detach_node(&mut self) -> Option<T> {
        let node_id = self.node_id;
        let node = self.node()?;
        if !node.has_left() && !node.has_right() {
            let parent_id = node.parent_id();
            self.node_id = parent_id;
            self.tree.remove_edge(parent_id, node_id);
            self.tree.remove_node(node_id)
        } else {
            None
        }
    }

    /// Swaps the left child of the cursor's node with the right child.
    pub fn swap_children(&mut self) -> Result<(), CursorError> {
        let Some(node) = self.node_mut() else { return Err(CursorError::NullError); };
        let left_id = node.left_id();
        let right_id = node.right_id();
        node.set_left_id(right_id);
        node.set_right_id(left_id);
        Ok(())
    }

    /// Removes the node pointed at by the cursor from the tree, assuming the node has exactly one child.
    /// Replaces the node by its child, after which the cursor points to this child.
    /// Does nothing if the node has zero or two children.
    /// Returns the removed node.
    pub fn transplant_child(&mut self) -> Option<T> {
        let node_id = self.node_id;
        let node = self.node()?;
        let side_of_parent = self.side_of_parent();
        let parent_id = node.parent_id();
        let Neighborhood { left, right, .. } = self.peek_neighborhood();
        match (left, right) {
            (Some(_), None) => {
                let left_id = node.left_id();
                self.tree.remove_edge(self.node_id, left_id);
                if let Some(side_of_parent) = side_of_parent {
                    self.tree.remove_edge(parent_id, self.node_id);
                    self.tree.add_edge(parent_id, left_id, side_of_parent);
                } else {
                    // Removed node was root.
                    self.tree.set_root_id(left_id);
                }
                self.node_id = left_id;
                self.tree.remove_node(node_id)
            },
            (None, Some(_)) => {
                let right_id = node.right_id();
                self.tree.remove_edge(self.node_id, right_id);
                if let Some(side_of_parent) = side_of_parent {
                    self.tree.remove_edge(parent_id, self.node_id);
                    self.tree.add_edge(parent_id, right_id, side_of_parent);
                } else {
                    // Removed node was root.
                    self.tree.set_root_id(right_id);
                }
                self.node_id = right_id;
                self.tree.remove_node(node_id)
            },
            _ => None,
        }
    }
    
    /// Performs a left rotation around the node pointed at by the cursor.
    /// The cursor keeps pointing to the same node, which moves during rotation.
    pub fn rotate_left(&mut self) -> Result<(), CursorError> {
        // Gather ids of the relevant nodes.
        // Right child must exist for a right rotation to work.
        let node_id = self.node_id;
        let node = self.node().ok_or(CursorError::RotateLeftError)?;

        let right_id = node.right_id();
        let right_node = self.tree.node(right_id).ok_or(CursorError::RotateLeftError)?;

        let parent_id = node.parent_id();
        let rotating_id = right_node.left_id();

        // Perform the rotation by adding and removing edges.
        self.tree.remove_edge(node_id, right_id);

        self.tree.remove_edge(right_id, rotating_id);
        self.tree.add_edge(node_id, rotating_id, Side::Right);

        if let Some(parent) = self.tree.node(parent_id) {
            let side = parent.side_of(node_id).unwrap();
            self.tree.remove_edge(parent_id, node_id);
            self.tree.add_edge(parent_id, right_id, side);
        } else {
            self.tree.set_root_id(right_id);
        }

        self.tree.add_edge(right_id, node_id, Side::Left);
        Ok(())
    }

    /// Performs a right rotation around the node pointed at by the cursor.
    /// The cursor keeps pointing to the same node, which moves during rotation.
    pub fn rotate_right(&mut self) -> Result<(), CursorError> {
        // Gather ids of the relevant nodes.
        // Left child must exist for a right rotation to work.
        let node_id = self.node_id;
        let node = self.node().ok_or(CursorError::RotateRightError)?;

        let left_id = node.left_id();
        let left_node = self.tree.node(left_id).ok_or(CursorError::RotateRightError)?;

        let parent_id = node.parent_id();
        let rotating_id = left_node.right_id();

        // Perform the rotation by adding and removing edges.
        self.tree.remove_edge(node_id, left_id);

        self.tree.remove_edge(left_id, rotating_id);
        self.tree.add_edge(node_id, rotating_id, Side::Left);

        if let Some(parent) = self.tree.node(parent_id) {
            let side = parent.side_of(node_id).unwrap();
            self.tree.remove_edge(parent_id, node_id);
            self.tree.add_edge(parent_id, left_id, side);
        } else {
            self.tree.set_root_id(left_id);
        }
        
        self.tree.add_edge(left_id, node_id, Side::Right);
        Ok(())
    }
}

impl<'t, T> BinaryTreeCursor for CursorMut<'t, T> {
    fn try_move_up(&mut self) -> Option<Side> {
        let node_id = self.node_id;
        let parent_id = self.node()?.parent_id();
        if !parent_id.is_null() {
            self.node_id = parent_id;
            self.node()?.side_of(node_id)
        } else { None }
    }
    
    fn try_move_left(&mut self) -> bool {
        let left_id = self.node().map(BinaryTreeNode::left_id);
        if let Some(left_id) = left_id && !left_id.is_null() {
            self.node_id = left_id;
            true
        } else { false }
    }
    
    fn try_move_right(&mut self) -> bool {
        let right_id = self.node().map(BinaryTreeNode::right_id);
        if let Some(right_id) = right_id && !right_id.is_null() {
            self.node_id = right_id;
            true
        } else { false }
    }

    fn move_up(&mut self) -> Option<Side> {
        let node_id = self.node_id;
        self.node_id = self.node().map_or(NodeId::null(), BinaryTreeNode::parent_id);
        self.node().and_then(|parent| parent.side_of(node_id))
    }

    fn move_left(&mut self) {
        self.node_id = self.node().map_or(NodeId::null(), BinaryTreeNode::left_id);
    }

    fn move_right(&mut self) {
        self.node_id = self.node().map_or(NodeId::null(), BinaryTreeNode::right_id);
    }
}

impl<'t, T> PeekingCursorMut for CursorMut<'t, T> {
    type Node = BinaryTreeNode<T>;
    type SpawnedCursor<'c> = Cursor<'c, T>
    where Self: 'c;

    fn node(&self) -> Option<&Self::Node> {
        self.tree.node(self.node_id)
    }

    fn node_mut(&mut self) -> Option<&mut Self::Node> {
        self.tree.node_mut(self.node_id)
    }

    fn spawn_cursor(&self) -> Self::SpawnedCursor<'_> {
        Cursor::new(self.tree, self.node_id)
    }

    fn side_of_parent(&self) -> Option<Side> {
        self.peek_up()?.side_of(self.node_id)
    }

    fn peek_up(&self) -> Option<&Self::Node> {
        self.tree.parent(self.node()?)
    }

    fn peek_left(&self) -> Option<&Self::Node> {
        self.tree.left_child(self.node()?)
    }

    fn peek_right(&self) -> Option<&Self::Node> {
        self.tree.right_child(self.node()?)
    }

    fn peek_up_mut(&mut self) -> Option<&mut Self::Node> {
        self.tree.node_mut(self.node()?.parent_id())
    }

    fn peek_left_mut(&mut self) -> Option<&mut Self::Node> {
        self.tree.node_mut(self.node()?.left_id())
    }

    fn peek_right_mut(&mut self) -> Option<&mut Self::Node> {
        self.tree.node_mut(self.node()?.right_id())
    }
}
