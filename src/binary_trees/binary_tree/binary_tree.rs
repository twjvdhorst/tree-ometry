use slotmap::{SlotMap, new_key_type};

use crate::binary_trees::Side;
use super::binary_tree_cursors::{Cursor, CursorMut};

new_key_type! { pub(super) struct NodeId; }

pub struct BinaryTreeNode<T> {
    data: T,
    parent_id: Option<NodeId>,
    left_id: Option<NodeId>,
    right_id: Option<NodeId>,
}

impl<T> BinaryTreeNode<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            parent_id: None,
            left_id: None,
            right_id: None,
        }
    }

    pub(super) fn new_with_parent(data: T, parent_id: NodeId) -> Self {
        Self {
            data,
            parent_id: Some(parent_id),
            left_id: None,
            right_id: None,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub(super) fn has_parent(&self) -> bool {
        self.parent_id.is_some()
    }

    pub(super) fn has_left(&self) -> bool {
        self.left_id.is_some()
    }

    pub(super) fn has_right(&self) -> bool {
        self.right_id.is_some()
    }

    pub(super) fn has_child(&self, side: Side) -> bool {
        match side {
            Side::Left => self.has_left(),
            Side::Right => self.has_right(),
        }
    }

    pub(super) fn parent_id(&self) -> Option<NodeId> {
        self.parent_id
    }

    pub(super) fn left_id(&self) -> Option<NodeId> {
        self.left_id
    }

    pub(super) fn right_id(&self) -> Option<NodeId> {
        self.right_id
    }

    pub(super) fn child_id(&self, side: Side) -> Option<NodeId> {
        match side {
            Side::Left => self.left_id(),
            Side::Right => self.right_id(),
        }
    }

    pub(super) fn side_of(&self, child_id: NodeId) -> Option<Side> {
        if self.left_id == Some(child_id) {
            Some(Side::Left)
        } else if self.right_id == Some(child_id) {
            Some(Side::Right)
        } else {
            None
        }
    }

    pub(super) fn set_parent_id(&mut self, new_id: NodeId) {
        self.parent_id.insert(new_id);
    }

    pub(super) fn set_left_id(&mut self, new_id: NodeId) {
        self.left_id.insert(new_id);
    }

    pub(super) fn set_right_id(&mut self, new_id: NodeId) {
        self.right_id.insert(new_id);
    }

    pub(super) fn set_child_id(&mut self, new_id: NodeId, side: Side) {
        match side {
            Side::Left => self.set_left_id(new_id),
            Side::Right => self.set_right_id(new_id),
        }
    }

    pub(super) fn take_parent_id(&mut self) -> Option<NodeId> {
        self.parent_id.take()
    }

    pub(super) fn take_left_id(&mut self) -> Option<NodeId> {
        self.left_id.take()
    }

    pub(super) fn take_right_id(&mut self) -> Option<NodeId> {
        self.right_id.take()
    }

    pub(super) fn take_child_id(&mut self, side: Side) -> Option<NodeId> {
        match side {
            Side::Left => self.take_left_id(),
            Side::Right => self.take_right_id(),
        }
    }

    pub(super) fn detach_child(&mut self, child_id: NodeId) {
        if self.left_id == Some(child_id) {
            self.left_id = None;
        } else if self.right_id == Some(child_id) {
            self.right_id = None;
        }
    }
}

pub struct BinaryTree<T> {
    nodes: SlotMap<NodeId, BinaryTreeNode<T>>,
    root_id: Option<NodeId>,
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self {
            nodes: SlotMap::with_key(),
            root_id: Option::None,
        }
    }
}

impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub(super) fn node(&self, node_id: NodeId) -> Option<&BinaryTreeNode<T>> {
        self.nodes.get(node_id)
    }

    pub(super) fn node_mut(&mut self, node_id: NodeId) -> Option<&mut BinaryTreeNode<T>> {
        self.nodes.get_mut(node_id)
    }

    pub fn parent(&self, node: &BinaryTreeNode<T>) -> Option<&BinaryTreeNode<T>> {
        self.nodes.get(node.parent_id?)
    }
    
    pub fn parent_mut(&mut self, node: &BinaryTreeNode<T>) -> Option<&mut BinaryTreeNode<T>> {
        self.nodes.get_mut(node.parent_id?)
    }

    pub fn left_child(&self, node: &BinaryTreeNode<T>) -> Option<&BinaryTreeNode<T>> {
        self.nodes.get(node.left_id?)
    }

    pub fn left_child_mut(&mut self, node: &BinaryTreeNode<T>) -> Option<&mut BinaryTreeNode<T>> {
        self.nodes.get_mut(node.left_id?)
    }

    pub fn right_child(&self, node: &BinaryTreeNode<T>) -> Option<&BinaryTreeNode<T>> {
        self.nodes.get(node.right_id?)
    }

    pub fn right_child_mut(&mut self, node: &BinaryTreeNode<T>) -> Option<&mut BinaryTreeNode<T>> {
        self.nodes.get_mut(node.right_id?)
    }

    pub(super) fn add_node(&mut self, node: BinaryTreeNode<T>) -> NodeId {
        self.nodes.insert(node)
    }
    
    pub(super) fn remove_node(&mut self, node_id: NodeId) -> Option<BinaryTreeNode<T>> {
        self.nodes.remove(node_id)
    }

    pub(super) fn add_edge(&mut self, parent_id: NodeId, child_id: NodeId, side_of_parent: Side) -> bool {
        if let Some(parent) = self.node(parent_id) && !parent.has_child(side_of_parent)
            && let Some(child) = self.node(child_id) && !child.has_parent()
        {
            self.node_mut(parent_id).unwrap().set_child_id(child_id, side_of_parent);
            self.node_mut(child_id).unwrap().set_parent_id(parent_id);
            true
        } else {
            false
        }
    }

    pub(super) fn remove_edge(&mut self, parent_id: NodeId, child_id: NodeId) {
        if let Some(parent) = self.node_mut(parent_id) && let Some(side) = parent.side_of(child_id) {
            parent.take_child_id(side);
        }

        if let Some(child) = self.node_mut(child_id) && child.parent_id == Some(parent_id) {
            child.take_parent_id();
        }
    }

    pub fn cursor(&self) -> Option<Cursor<'_, T>> {
        Some(Cursor::new(self, self.root_id?))
    }

    pub fn cursor_mut(&mut self) -> Option<CursorMut<'_, T>> {
        Some(CursorMut::new(self, self.root_id?))
    }
}
