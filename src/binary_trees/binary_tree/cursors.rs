use derive_more::Debug;
use slotmap::Key;

use super::binary_tree::{BinaryTree, BinaryTreeNode, NodeId};
use crate::binary_trees::{
    Side,
    cursor_errors::CursorError,
    traits::binary_tree_cursor::{BinaryTreeCursor, BinaryTreeCursorMut},
};

/// A cursor over a BinaryTree.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
#[derive(Debug)]
pub struct Cursor<'tree, T> {
    tree: &'tree BinaryTree<T>,
    node_id: NodeId,
}

impl<'tree, T> Cursor<'tree, T> {
    pub(super) fn new(tree: &'tree BinaryTree<T>, node_id: NodeId) -> Self {
        Self {
            tree,
            node_id,
        }
    }
}

impl<'tree, T> Clone for Cursor<'tree, T> {
    fn clone(&self) -> Self {
        Self::new(self.tree, self.node_id)
    }
}

impl<'tree, T> Copy for Cursor<'tree, T> {}

impl<'tree, T> BinaryTreeCursor for Cursor<'tree, T> {
    type Node = BinaryTreeNode<T>;

    fn node(&self) -> Option<&Self::Node> {
        self.tree.node(self.node_id)
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

    /// Advances the cursor to the parent node.
    /// If the cursor is already at the root of the tree, None is returned and the cursor is not moved.
    fn move_up(&mut self) -> Option<&Self::Node> {
        let parent_id = self.node()?.parent_id();
        if !parent_id.is_null() {
            self.node_id = parent_id;
            self.node()
        } else { None }
    }
    
    /// Advances the cursor to the left child node.
    /// If the cursor is already at a leaf of the tree, None is returned and the cursor is not moved.
    fn move_left(&mut self) -> Option<&Self::Node> {
        let left_id = self.node()?.left_id();
        if !left_id.is_null() {
            self.node_id = left_id;
            self.node()
        } else { None }
    }
    
    /// Advances the cursor to the right child node.
    /// If the cursor is already at a leaf of the tree, None is returned and the cursor is not moved.
    fn move_right(&mut self) -> Option<&Self::Node> {
        let right_id = self.node()?.right_id();
        if !right_id.is_null() {
            self.node_id = right_id;
            self.node()
        } else { None }
    }
}

/// A cursor over a BinaryTree with editing operations.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
/// Cursors maintain the invariant that as long as the tree has a node, the cursor points to a node.
#[derive(Debug)]
pub struct CursorMut<'tree, T> {
    tree: &'tree mut BinaryTree<T>,
    node_id: NodeId,
}

impl<'tree, T> CursorMut<'tree, T> {
    pub(super) fn new(tree: &'tree mut BinaryTree<T>, node_id: NodeId) -> Self {
        Self {
            tree,
            node_id,
        }
    }

    /// Creates a new root node, if the tree is empty.
    pub fn create_root(&mut self, data: T) -> Result<(), CursorError> {
        if !self.node_id.is_null() {
            Err(CursorError::RootCreationError)
        } else {
            self.node_id = self.tree.new_node(data);
            Ok(())
        }
    }

    /// Creates a new node and attaches it as a child to the node pointed at by the cursor.
    pub fn attach_child(&mut self, data: T, side: Side) -> Result<(), CursorError> {
        let Some(node) = self.node() else { return Err(CursorError::NullError); };

        if node.has_child(side) {
            return Err(CursorError::AttachError(side));
        }

        let curr_id = self.node_id;
        let new_id = self.tree.new_node(data);
        self.tree.add_edge(curr_id, new_id, side);
        Ok(())
    }

    /// Detaches the child of the node pointed at by the cursor from the tree.
    /// The cursor stays in place.
    /// Does nothing if the child node is not a leaf.
    /// Returns the detached node.
    pub fn detach_child(&mut self, side: Side) -> Option<BinaryTreeNode<T>> {
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
    pub fn detach_node(&mut self) -> Option<BinaryTreeNode<T>> {
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
    
    /// Performs a left rotation around the node pointed at by the cursor.
    pub fn rotate_left(&mut self) -> Result<(), CursorError> {
        // Gather ids of the relevant nodes.
        // Right child must exist for a right rotation to work.
        let node_id = self.node_id;
        let node = self.node().ok_or(CursorError::RotateLeftError)?;

        let right_id = node.right_id();
        let right_node = self.tree.node(right_id).ok_or(CursorError::RotateRightError)?;

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
        
        // Move the cursor to the new "root".
        self.move_up();
        Ok(())
    }

    /// Performs a right rotation around the node pointed at by the cursor.
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

        // Move the cursor to the new "root".
        self.move_up();
        Ok(())
    }
}

impl<'tree, T> BinaryTreeCursor for CursorMut<'tree, T> {
    type Node = BinaryTreeNode<T>;

    fn node(&self) -> Option<&Self::Node> {
        self.tree.node(self.node_id)
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

    /// Advances the cursor to the parent node.
    /// If the cursor is already at the root of the tree, None is returned and the cursor is not moved.
    fn move_up(&mut self) -> Option<&Self::Node> {
        let parent_id = self.node()?.parent_id();
        if !parent_id.is_null() {
            self.node_id = parent_id;
            self.node()
        } else { None }
    }
    
    /// Advances the cursor to the left child node.
    /// If the cursor is already at a leaf of the tree, None is returned and the cursor is not moved.
    fn move_left(&mut self) -> Option<&Self::Node> {
        let left_id = self.node()?.left_id();
        if !left_id.is_null() {
            self.node_id = left_id;
            self.node()
        } else { None }
    }
    
    /// Advances the cursor to the right child node.
    /// If the cursor is already at a leaf of the tree, None is returned and the cursor is not moved.
    fn move_right(&mut self) -> Option<&Self::Node> {
        let right_id = self.node()?.right_id();
        if !right_id.is_null() {
            self.node_id = right_id;
            self.node()
        } else { None }
    }
}

impl<'tree, T> BinaryTreeCursorMut for CursorMut<'tree, T> {
    fn node_mut(&mut self) -> Option<&mut Self::Node> {
        self.tree.node_mut(self.node_id)
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
    
    /// Advances the cursor to the parent node.
    /// If the cursor is already at the root of the tree, None is returned and the cursor is not moved.
    fn move_up_mut(&mut self) -> Option<&mut Self::Node> {
        let parent_id = self.node()?.parent_id();
        if !parent_id.is_null() {
            self.node_id = parent_id;
            self.node_mut()
        } else { None }
    }
    
    /// Advances the cursor to the left child node.
    /// If the cursor is already at a leaf of the tree, None is returned and the cursor is not moved.
    fn move_left_mut(&mut self) -> Option<&mut Self::Node> {
        let left_id = self.node()?.left_id();
        if !left_id.is_null() {
            self.node_id = left_id;
            self.node_mut()
        } else { None }
    }
    
    /// Advances the cursor to the right child node.
    /// If the cursor is already at a leaf of the tree, None is returned and the cursor is not moved.
    fn move_right_mut(&mut self) -> Option<&mut Self::Node> {
        let right_id = self.node()?.right_id();
        if !right_id.is_null() {
            self.node_id = right_id;
            self.node_mut()
        } else { None }
    }
}
