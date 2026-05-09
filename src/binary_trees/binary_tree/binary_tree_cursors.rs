use super::binary_tree::{BinaryTree, BinaryTreeNode, NodeId};
use crate::binary_trees::{
    Side,
    cursor_errors::CursorError,
    traits::binary_tree_cursor::{BinaryTreeCursor, BinaryTreeCursorMut},
};

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

    fn move_up(&mut self) -> Option<&Self::Node> {
        self.node_id = self.node()?.parent_id()?;
        self.node()
    }
    
    fn move_left(&mut self) -> Option<&Self::Node> {
        self.node_id = self.node()?.left_id()?;
        self.node()
    }
    
    fn move_right(&mut self) -> Option<&Self::Node> {
        self.node_id = self.node()?.right_id()?;
        self.node()
    }
}

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

    /// Creates a new node and attaches it as a child to the node pointed at by the cursor.
    pub fn attach_child(&mut self, data: T, side: Side) -> Result<(), CursorError> {
        let current_node = self.node().ok_or(CursorError::InvalidCursor)?;
        if current_node.has_child(side) {
            return Err(CursorError::CannotAttachChild(side));
        }

        let new_node = BinaryTreeNode::new_with_parent(data, self.node_id);
        let new_id = self.tree.add_node(new_node);
        self.node_mut().unwrap().set_child_id(new_id, side); // Can unwrap safely; the cursor is still valid.
        Ok(())
    }

    /// Detaches the node pointed at by the cursor from the tree, and moves the cursor up.
    /// Returns an error if the node pointed at is not a leaf.
    pub fn detach_node(&mut self) -> Result<BinaryTreeNode<T>, CursorError> {
        let node_id = self.node_id;
        let current_node = self.node_mut().ok_or(CursorError::InvalidCursor)?;
        if current_node.has_left() || current_node.has_right() {
            return Err(CursorError::CannotDetachNode);
        }

        current_node.take_parent_id();

        if let Some(parent) = self.move_up_mut() {
            parent.detach_child(node_id);
        }

        Ok(self.tree.remove_node(node_id).unwrap())
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

    fn move_up(&mut self) -> Option<&Self::Node> {
        self.node_id = self.node()?.parent_id()?;
        self.node()
    }
    
    fn move_left(&mut self) -> Option<&Self::Node> {
        self.node_id = self.node()?.left_id()?;
        self.node()
    }
    
    fn move_right(&mut self) -> Option<&Self::Node> {
        self.node_id = self.node()?.right_id()?;
        self.node()
    }
}

impl<'tree, T> BinaryTreeCursorMut for CursorMut<'tree, T> {
    fn node_mut(&mut self) -> Option<&mut Self::Node> {
        self.tree.node_mut(self.node_id)
    }

    fn peek_up_mut(&mut self) -> Option<&mut Self::Node> {
        self.tree.node_mut(self.node()?.parent_id()?)
    }

    fn peek_left_mut(&mut self) -> Option<&mut Self::Node> {
        self.tree.node_mut(self.node()?.left_id()?)
    }

    fn peek_right_mut(&mut self) -> Option<&mut Self::Node> {
        self.tree.node_mut(self.node()?.right_id()?)
    }

    fn move_up_mut(&mut self) -> Option<&mut Self::Node> {
        self.node_id = self.node()?.parent_id()?;
        self.node_mut()
    }
    
    fn move_left_mut(&mut self) -> Option<&mut Self::Node> {
        self.node_id = self.node()?.left_id()?;
        self.node_mut()
    }
    
    fn move_right_mut(&mut self) -> Option<&mut Self::Node> {
        self.node_id = self.node()?.right_id()?;
        self.node_mut()
    }
}
