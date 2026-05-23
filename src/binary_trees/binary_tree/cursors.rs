use derive_more::Debug;
use slotmap::Key;

use super::{BinaryTree, BinaryTreeNode, NodeId};
use crate::binary_trees::{
    Neighborhood,
    Side,
    cursor_errors::CursorError,
};

/// A cursor over a BinaryTree.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
#[derive(Debug)]
pub struct Cursor<'t, T> {
    tree: &'t BinaryTree<T>,
    node_id: NodeId,
}

/// Make own implementation of Clone, so T doesn't have to be Clone.
impl<'t, T> Clone for Cursor<'t, T> {
    fn clone(&self) -> Self {
        Self::new(self.tree, self.node_id)
    }
}

impl<'t, T> Copy for Cursor<'t, T> {}

impl<'t, T> Cursor<'t, T> {
    pub(super) fn new(tree: &'t BinaryTree<T>, node_id: NodeId) -> Self {
        Self {
            tree,
            node_id,
        }
    }

    fn node(&self) -> Option<&'t BinaryTreeNode<T>> {
        self.tree.node(self.node_id)
    }

    pub(super) fn node_id(&self) -> NodeId {
        self.node_id
    }
    
    pub fn try_move_up(&mut self) -> Option<Side> {
        let node_id = self.node_id;
        let parent_id = self.tree.parent_id(self.node_id)?;
        if !parent_id.is_null() {
            self.node_id = parent_id;
            self.tree.node(parent_id)?.side_of(node_id)
        } else { None }
    }
    
    pub fn try_move_left(&mut self) -> bool {
        let left_id = self.tree.left_id(self.node_id);
        if let Some(left_id) = left_id && !left_id.is_null() {
            self.node_id = left_id;
            true
        } else { false }
    }
    
    pub fn try_move_right(&mut self) -> bool {
        let right_id = self.tree.right_id(self.node_id);
        if let Some(right_id) = right_id && !right_id.is_null() {
            self.node_id = right_id;
            true
        } else { false }
    }

    pub fn try_move_side(&mut self, side: Side) -> bool {
        match side {
            Side::Left => self.try_move_left(),
            Side::Right => self.try_move_right(),
        }
    }

    pub fn move_up(&mut self) -> Option<Side> {
        let node_id = self.node_id;
        self.node_id = self.tree.parent_id(node_id).unwrap_or(NodeId::null());
        self.node().and_then(|parent| parent.side_of(node_id))
    }

    pub fn move_left(&mut self) {
        self.node_id = self.tree.left_id(self.node_id).unwrap_or(NodeId::null());
    }

    pub fn move_right(&mut self) {
        self.node_id = self.tree.right_id(self.node_id).unwrap_or(NodeId::null());
    }

    pub fn move_side(&mut self, side: Side) {
        match side {
            Side::Left => self.move_left(),
            Side::Right => self.move_right(),
        }
    }

    /// Moves the cursor to the inorder predecessor of the current node.
    /// If no predecessor exists, false is returned and the cursor is not moved.
    pub fn try_move_prev(&mut self) -> bool {
        // Inorder predecessor is either the rightmost node in the left subtree, or the first ancestor that we are a right descendant of.
        if self.try_move_left() {
            while self.try_move_right() {}
            return true;
        }
        
        while let Some(side) = self.try_move_up() {
            if side == Side::Right {
                return true;
            }
        }

        false
    }

    /// Moves the cursor to the inorder successor of the current node.
    /// If no successor exists, false is returned and the cursor is not moved.
    pub fn try_move_next(&mut self) -> bool {
        // Inorder successor is either the leftmost node in the right subtree, or the first ancestor that we are a left descendant of.
        if self.try_move_right() {
            while self.try_move_left() {}
            return true;
        }
        
        while let Some(side) = self.try_move_up() {
            if side == Side::Left {
                return true;
            }
        }

        false
    }

    /// Moves the cursor to the inorder predecessor of the current node.
    /// If no predecessor exists, the cursor is moved to a "null" node.
    pub fn move_prev(&mut self) {
        // Inorder predecessor is either the rightmost node in the left subtree, or the first ancestor that we are a right descendant of.
        if self.try_move_left() {
            while self.try_move_right() {}
        } else {
            while let Some(side) = self.move_up() && side != Side::Right {}
        }
    }

    /// Moves the cursor to the inorder successor of the current node.
    /// If no successor exists, the cursor is moved to a "null" node.
    pub fn move_next(&mut self) {
        // Inorder successor is either the leftmost node in the right subtree, or the first ancestor that we are a left descendant of.
        if self.try_move_right() {
            while self.try_move_left() {}
        } else {
            while let Some(side) = self.move_up() && side != Side::Left {}
        }
    }

    pub fn get(&self) -> Option<&'t T> {
        self.node().map(BinaryTreeNode::data)
    }

    pub fn side_of_parent(&self) -> Option<Side> {
        self.tree.parent(self.node_id)?.side_of(self.node_id)
    }

    pub fn peek_up(&self) -> Option<&'t T> {
        self.tree.parent(self.node_id).map(BinaryTreeNode::data)
    }

    pub fn peek_left(&self) -> Option<&'t T> {
        self.tree.left(self.node_id).map(BinaryTreeNode::data)
    }

    pub fn peek_right(&self) -> Option<&'t T> {
        self.tree.right(self.node_id).map(BinaryTreeNode::data)
    }

    pub fn peek_side(&self, side: Side) -> Option<&'t T> {
        match side {
            Side::Left => self.peek_left(),
            Side::Right => self.peek_right(),
        }
    }

    pub fn peek_neighborhood(&self) -> Neighborhood<&'t T> {
        Neighborhood {
            node: self.get(),
            parent: self.peek_up(),
            left: self.peek_left(),
            right: self.peek_right()
        }
    }
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

    fn node(&self) -> Option<&BinaryTreeNode<T>> {
        self.tree.node(self.node_id)
    }

    fn node_mut(&mut self) -> Option<&mut BinaryTreeNode<T>> {
        self.tree.node_mut(self.node_id)
    }

    pub fn as_cursor(&self) -> Cursor<'_, T> {
        Cursor::new(self.tree, self.node_id)
    }
    
    pub fn try_move_up(&mut self) -> Option<Side> {
        let node_id = self.node_id;
        let parent_id = self.tree.parent_id(self.node_id)?;
        if !parent_id.is_null() {
            self.node_id = parent_id;
            self.tree.node(parent_id)?.side_of(node_id)
        } else { None }
    }
    
    pub fn try_move_left(&mut self) -> bool {
        let left_id = self.tree.left_id(self.node_id);
        if let Some(left_id) = left_id && !left_id.is_null() {
            self.node_id = left_id;
            true
        } else { false }
    }
    
    pub fn try_move_right(&mut self) -> bool {
        let right_id = self.tree.right_id(self.node_id);
        if let Some(right_id) = right_id && !right_id.is_null() {
            self.node_id = right_id;
            true
        } else { false }
    }

    pub fn try_move_side(&mut self, side: Side) -> bool {
        match side {
            Side::Left => self.try_move_left(),
            Side::Right => self.try_move_right(),
        }
    }

    pub fn move_up(&mut self) -> Option<Side> {
        let node_id = self.node_id;
        self.node_id = self.tree.parent_id(node_id).unwrap_or(NodeId::null());
        self.node().and_then(|parent| parent.side_of(node_id))
    }

    pub fn move_left(&mut self) {
        self.node_id = self.tree.left_id(self.node_id).unwrap_or(NodeId::null());
    }

    pub fn move_right(&mut self) {
        self.node_id = self.tree.right_id(self.node_id).unwrap_or(NodeId::null());
    }

    pub fn move_side(&mut self, side: Side) {
        match side {
            Side::Left => self.move_left(),
            Side::Right => self.move_right(),
        }
    }

    /// Moves the cursor to the inorder predecessor of the current node.
    /// If no predecessor exists, false is returned and the cursor is not moved.
    pub fn try_move_prev(&mut self) -> bool {
        // Inorder predecessor is either the rightmost node in the left subtree, or the first ancestor that we are a right descendant of.
        if self.try_move_left() {
            while self.try_move_right() {}
            return true;
        }
        
        while let Some(side) = self.try_move_up() {
            if side == Side::Right {
                return true;
            }
        }

        false
    }

    /// Moves the cursor to the inorder successor of the current node.
    /// If no successor exists, false is returned and the cursor is not moved.
    pub fn try_move_next(&mut self) -> bool {
        // Inorder successor is either the leftmost node in the right subtree, or the first ancestor that we are a left descendant of.
        if self.try_move_right() {
            while self.try_move_left() {}
            return true;
        }
        
        while let Some(side) = self.try_move_up() {
            if side == Side::Left {
                return true;
            }
        }

        false
    }

    /// Moves the cursor to the inorder predecessor of the current node.
    /// If no predecessor exists, the cursor is moved to a "null" node.
    pub fn move_prev(&mut self) {
        // Inorder predecessor is either the rightmost node in the left subtree, or the first ancestor that we are a right descendant of.
        if self.try_move_left() {
            while self.try_move_right() {}
        } else {
            while let Some(side) = self.move_up() && side != Side::Right {}
        }
    }

    /// Moves the cursor to the inorder successor of the current node.
    /// If no successor exists, the cursor is moved to a "null" node.
    pub fn move_next(&mut self) {
        // Inorder successor is either the leftmost node in the right subtree, or the first ancestor that we are a left descendant of.
        if self.try_move_right() {
            while self.try_move_left() {}
        } else {
            while let Some(side) = self.move_up() && side != Side::Left {}
        }
    }
    
    pub fn get(&mut self) -> Option<&mut T> {
        self.node_mut().map(BinaryTreeNode::data_mut)
    }

    pub fn side_of_parent(&self) -> Option<Side> {
        self.tree.parent(self.node_id)?.side_of(self.node_id)
    }

    pub fn peek_up(&mut self) -> Option<&mut T> {
        self.tree.parent_mut(self.node_id).map(BinaryTreeNode::data_mut)
    }

    pub fn peek_left(&mut self) -> Option<&mut T> {
        self.tree.left_mut(self.node_id).map(BinaryTreeNode::data_mut)
    }

    pub fn peek_right(&mut self) -> Option<&mut T> {
        self.tree.right_mut(self.node_id).map(BinaryTreeNode::data_mut)
    }

    pub fn peek_side(&mut self, side: Side) -> Option<&mut T> {
        match side {
            Side::Left => self.peek_left(),
            Side::Right => self.peek_right(),
        }
    }
    
    pub fn peek_neighborhood(&mut self) -> Neighborhood<&mut T> {
        if self.get().is_none() {
            return Neighborhood {
                node: None,
                parent: None,
                left: None,
                right: None,
            };
        }

        // Safety: the cursor points to a node, so the following ids exist (though are possibly null).
        let parent_id = self.tree.parent_id(self.node_id).unwrap();
        let left_id = self.tree.left_id(self.node_id).unwrap();
        let right_id = self.tree.right_id(self.node_id).unwrap();
        match (parent_id, left_id, right_id) {
            (parent_id, left_id, right_id) 
                if !parent_id.is_null() && !left_id.is_null() && !right_id.is_null() => 
            {
                let [node, parent, left, right] = unsafe { self.tree.get_disjoint_nodes_unchecked_mut([self.node_id, parent_id, left_id, right_id]) };
                Neighborhood {
                    node: Some(node.data_mut()),
                    parent: Some(parent.data_mut()),
                    left: Some(left.data_mut()),
                    right: Some(right.data_mut()),
                }
            },
            (_, left_id, right_id) 
                if !left_id.is_null() && !right_id.is_null() => 
            {
                let [node, left, right] = unsafe { self.tree.get_disjoint_nodes_unchecked_mut([self.node_id, left_id, right_id]) };
                Neighborhood {
                    node: Some(node.data_mut()),
                    parent: None,
                    left: Some(left.data_mut()),
                    right: Some(right.data_mut()),
                }
            },
            (parent_id, _, right_id) 
                if !parent_id.is_null() && !right_id.is_null() => 
            {
                let [node, parent, right] = unsafe { self.tree.get_disjoint_nodes_unchecked_mut([self.node_id, parent_id, right_id]) };
                Neighborhood {
                    node: Some(node.data_mut()),
                    parent: Some(parent.data_mut()),
                    left: None,
                    right: Some(right.data_mut()),
                }
            },
            (parent_id, left_id, _) 
                if !parent_id.is_null() && !left_id.is_null() => 
            {
                let [node, parent, left] = unsafe { self.tree.get_disjoint_nodes_unchecked_mut([self.node_id, parent_id, left_id]) };
                Neighborhood {
                    node: Some(node.data_mut()),
                    parent: Some(parent.data_mut()),
                    left: Some(left.data_mut()),
                    right: None,
                }
            },
            (parent_id, _, _) if !parent_id.is_null() => {
                let [node, parent] = unsafe { self.tree.get_disjoint_nodes_unchecked_mut([self.node_id, parent_id]) };
                Neighborhood {
                    node: Some(node.data_mut()),
                    parent: Some(parent.data_mut()),
                    left: None,
                    right: None,
                }
            },
            (_, left_id, _) if !left_id.is_null() => {
                let [node, left] = unsafe { self.tree.get_disjoint_nodes_unchecked_mut([self.node_id, left_id]) };
                Neighborhood {
                    node: Some(node.data_mut()),
                    parent: None,
                    left: Some(left.data_mut()),
                    right: None,
                }
            },
            (_, _, right_id) if !right_id.is_null() => {
                let [node, right] = unsafe { self.tree.get_disjoint_nodes_unchecked_mut([self.node_id, right_id]) };
                Neighborhood {
                    node: Some(node.data_mut()),
                    parent: None,
                    left: None,
                    right: Some(right.data_mut()),
                }
            },
            _ => Neighborhood { 
                    node: self.get(),
                    parent: None, 
                    left: None, 
                    right: None 
                },
        }
    }
    
    /// Spawn N cursors and move them around the tree according to the supplied function.
    /// Reports mutable references to the nodes the cursors end up pointing at.
    /// Requires the cursors to end up pointing at distinct, existing nodes; else None is returned.
    pub fn spawn_and_peek<F, const N: usize>(&mut self, cursors_fn: F) -> Option<[&mut T; N]>
    where
        F: FnOnce(&mut [Cursor<'_, T>; N]),
    {
        let mut cursors = [self.as_cursor(); N];
        cursors_fn(&mut cursors);
        let ids = cursors.map(|cursor| cursor.node_id);
        Some(self.tree.get_disjoint_nodes_mut(ids)?.map(BinaryTreeNode::data_mut))
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
        self.tree.add_edge_unchecked(new_root_id, old_root_id, side);
        self.node_id = new_root_id;
    }

    /// Creates a new node and attaches the current node as a child to the new node.
    pub fn attach_parent(&mut self, data: T, side: Side) -> Result<(), CursorError> {
        let node = self.node().ok_or(CursorError::NullError)?;

        if node.has_parent() {
            return Err(CursorError::AttachParentError);
        }

        let curr_id = self.node_id;
        let new_id = self.tree.new_node(data);
        self.tree.add_edge_unchecked(new_id, curr_id, side);
        Ok(())
    }

    /// Creates a new node and attaches it as a child to the node pointed at by the cursor.
    pub fn attach_child(&mut self, data: T, side: Side) -> Result<(), CursorError> {
        let node = self.node().ok_or(CursorError::NullError)?;

        if node.has_child(side) {
            return Err(CursorError::AttachChildError(side));
        }

        let curr_id = self.node_id;
        let new_id = self.tree.new_node(data);
        self.tree.add_edge_unchecked(curr_id, new_id, side);
        Ok(())
    }

    /// Creates a new node and attaches the current node as a child to the new node.
    /// If the cursor already had a parent, the new node is "inserted" onto the edge.
    pub fn attach_or_insert_parent(&mut self, data: T, side: Side) -> Result<(), CursorError> {
        let node = self.node().ok_or(CursorError::NullError)?;

        let curr_id = self.node_id;
        let parent_id = node.parent_id();
        let new_id = self.tree.new_node(data);

        if !parent_id.is_null() {
            self.tree.remove_edge(parent_id, curr_id);
            self.tree.add_edge_unchecked(parent_id, new_id, side);
        }
        self.tree.add_edge_unchecked(new_id, curr_id, side);
        Ok(())
    }

    /// Creates a new node and attaches it as a child to the node pointed at by the cursor.
    /// If the cursor already had a child on the assigned side, the new node is "inserted" onto the edge.
    pub fn attach_or_insert_child(&mut self, data: T, side: Side) -> Result<(), CursorError> {
        let node = self.node().ok_or(CursorError::NullError)?;

        let curr_id = self.node_id;
        let child_id = node.child_id(side);
        let new_id = self.tree.new_node(data);

        if !child_id.is_null() {
            self.tree.remove_edge(curr_id, child_id);
            self.tree.add_edge_unchecked(new_id, child_id, side);
        }
        self.tree.add_edge_unchecked(curr_id, new_id, side);
        Ok(())
    }

    /// Detaches the child of the node pointed at by the cursor from the tree.
    /// The cursor stays in place.
    /// Does nothing if the child node is not a leaf.
    /// Returns the detached node.
    pub fn detach_child(&mut self, side: Side) -> Option<T> {
        let node_id = self.node_id;
        let child_id = self.tree.child_id(self.node_id, side)?;
        let child_node = self.tree.node(child_id)?;
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
        let node = self.node_mut().ok_or(CursorError::NullError)?;
        node.swap_children();
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
        let left_id = node.left_id();
        let right_id = node.right_id();
        let parent_id = node.parent_id();
        let Neighborhood { left, right, .. } = self.peek_neighborhood();

        match (left, right) {
            (Some(_), None) => {
                self.tree.remove_edge(self.node_id, left_id);
                if let Some(side_of_parent) = side_of_parent {
                    self.tree.remove_edge(parent_id, self.node_id);
                    self.tree.add_edge_unchecked(parent_id, left_id, side_of_parent);
                } else {
                    // Removed node was root.
                    self.tree.set_root_id(left_id);
                }
                self.node_id = left_id;
                self.tree.remove_node(node_id)
            },
            (None, Some(_)) => {
                self.tree.remove_edge(self.node_id, right_id);
                if let Some(side_of_parent) = side_of_parent {
                    self.tree.remove_edge(parent_id, self.node_id);
                    self.tree.add_edge_unchecked(parent_id, right_id, side_of_parent);
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
        self.tree.add_edge_unchecked(node_id, rotating_id, Side::Right);

        if let Some(parent) = self.tree.node(parent_id) {
            let side = parent.side_of(node_id).unwrap();
            self.tree.remove_edge(parent_id, node_id);
            self.tree.add_edge_unchecked(parent_id, right_id, side);
        } else {
            self.tree.set_root_id(right_id);
        }

        self.tree.add_edge_unchecked(right_id, node_id, Side::Left);
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
        self.tree.add_edge_unchecked(node_id, rotating_id, Side::Left);

        if let Some(parent) = self.tree.node(parent_id) {
            let side = parent.side_of(node_id).unwrap();
            self.tree.remove_edge(parent_id, node_id);
            self.tree.add_edge_unchecked(parent_id, left_id, side);
        } else {
            self.tree.set_root_id(left_id);
        }
        
        self.tree.add_edge_unchecked(left_id, node_id, Side::Right);
        Ok(())
    }

    pub fn rotate(&mut self, side: Side) -> Result<(), CursorError> {
        match side {
            Side::Left => self.rotate_left(),
            Side::Right => self.rotate_right(),
        }
    }
}
