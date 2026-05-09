use slotmap::{Key, SlotMap, new_key_type};

use crate::binary_trees::Side;
use super::binary_tree_cursors::{Cursor, CursorMut};

new_key_type! { pub(super) struct NodeId; }

pub struct BinaryTreeNode<T> {
    data: T,
    parent_id: NodeId,
    left_id: NodeId,
    right_id: NodeId,
}

impl<T> BinaryTreeNode<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            parent_id: NodeId::null(),
            left_id: NodeId::null(),
            right_id: NodeId::null(),
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub(super) fn has_parent(&self) -> bool {
        !self.parent_id.is_null()
    }

    pub(super) fn has_left(&self) -> bool {
        !self.left_id.is_null()
    }

    pub(super) fn has_right(&self) -> bool {
        !self.right_id.is_null()
    }

    pub(super) fn has_child(&self, side: Side) -> bool {
        match side {
            Side::Left => self.has_left(),
            Side::Right => self.has_right(),
        }
    }

    pub(super) fn parent_id(&self) -> NodeId {
        self.parent_id
    }

    pub(super) fn left_id(&self) -> NodeId {
        self.left_id
    }

    pub(super) fn right_id(&self) -> NodeId {
        self.right_id
    }

    pub(super) fn child_id(&self, side: Side) -> NodeId {
        match side {
            Side::Left => self.left_id(),
            Side::Right => self.right_id(),
        }
    }

    pub(super) fn side_of(&self, child_id: NodeId) -> Option<Side> {
        if child_id.is_null() { return None; }
        if self.left_id == child_id {
            Some(Side::Left)
        } else if self.right_id == child_id {
            Some(Side::Right)
        } else {
            None
        }
    }

    pub(super) fn set_parent_id(&mut self, new_id: NodeId) {
        self.parent_id = new_id;
    }

    pub(super) fn set_left_id(&mut self, new_id: NodeId) {
        self.left_id = new_id;
    }

    pub(super) fn set_right_id(&mut self, new_id: NodeId) {
        self.right_id = new_id;
    }

    pub(super) fn set_child_id(&mut self, new_id: NodeId, side: Side) {
        match side {
            Side::Left => self.set_left_id(new_id),
            Side::Right => self.set_right_id(new_id),
        }
    }

    pub(super) fn nullify_parent_id(&mut self) -> NodeId {
        let old_parent_id = self.parent_id;
        self.parent_id = NodeId::null();
        old_parent_id
    }

    pub(super) fn nullify_left_id(&mut self) -> NodeId {
        let old_left_id = self.left_id;
        self.left_id = NodeId::null();
        old_left_id
    }

    pub(super) fn nullify_right_id(&mut self) -> NodeId {
        let old_right_id = self.right_id;
        self.right_id = NodeId::null();
        old_right_id
    }

    pub(super) fn nullify_child_id(&mut self, side: Side) -> NodeId {
        match side {
            Side::Left => self.nullify_left_id(),
            Side::Right => self.nullify_right_id(),
        }
    }

    pub(super) fn detach_parent(&mut self) {
        self.parent_id = NodeId::null();
    }

    pub(super) fn detach_child(&mut self, child_id: NodeId) {
        if self.left_id == child_id {
            self.left_id = NodeId::null();
        } else if self.right_id == child_id {
            self.right_id = NodeId::null();
        }
    }
}

pub struct BinaryTree<T> {
    nodes: SlotMap<NodeId, BinaryTreeNode<T>>,
    root_id: NodeId,
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self {
            nodes: SlotMap::with_key(),
            root_id: NodeId::null(),
        }
    }
}

impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub(super) fn new_node(&mut self, data: T) -> NodeId {
        self.nodes.insert(BinaryTreeNode::new(data))
    }

    pub fn root(&self) -> Option<&BinaryTreeNode<T>> {
        self.nodes.get(self.root_id)
    }

    pub fn root_mut(&mut self) -> Option<&mut BinaryTreeNode<T>> {
        self.nodes.get_mut(self.root_id)
    }

    pub(super) fn node(&self, node_id: NodeId) -> Option<&BinaryTreeNode<T>> {
        self.nodes.get(node_id)
    }

    pub(super) fn node_mut(&mut self, node_id: NodeId) -> Option<&mut BinaryTreeNode<T>> {
        self.nodes.get_mut(node_id)
    }

    /// Returns a reference to the node corresponding to the id without version or bounds checking.
    /// Safety: This should only be used if there is a node with the given id. Otherwise it is potentially unsafe
    pub(super) unsafe fn node_unchecked(&self, node_id: NodeId) -> &BinaryTreeNode<T> {
        unsafe { self.nodes.get_unchecked(node_id) }
    }

    /// Returns a mutable reference to the node corresponding to the id without version or bounds checking.
    /// Safety: This should only be used if there is a node with the given id. Otherwise it is potentially unsafe
    pub(super) unsafe fn node_unchecked_mut(&mut self, node_id: NodeId) -> &mut BinaryTreeNode<T> {
        unsafe { self.nodes.get_unchecked_mut(node_id) }
    }

    /// Returns mutable references to the nodes with the given ids.
    /// All ids must be valid and disjoint, otherwise None is returned.
    pub(super) fn disjoint_nodes_mut<const N: usize>(&mut self, node_ids: [NodeId; N]) -> Option<[&mut BinaryTreeNode<T>; N]> {
        self.nodes.get_disjoint_mut(node_ids)
    }

    pub fn parent(&self, node: &BinaryTreeNode<T>) -> Option<&BinaryTreeNode<T>> {
        self.nodes.get(node.parent_id)
    }
    
    pub fn parent_mut(&mut self, node: &BinaryTreeNode<T>) -> Option<&mut BinaryTreeNode<T>> {
        self.nodes.get_mut(node.parent_id)
    }

    pub fn left_child(&self, node: &BinaryTreeNode<T>) -> Option<&BinaryTreeNode<T>> {
        self.nodes.get(node.left_id)
    }

    pub fn left_child_mut(&mut self, node: &BinaryTreeNode<T>) -> Option<&mut BinaryTreeNode<T>> {
        self.nodes.get_mut(node.left_id)
    }

    pub fn right_child(&self, node: &BinaryTreeNode<T>) -> Option<&BinaryTreeNode<T>> {
        self.nodes.get(node.right_id)
    }

    pub fn right_child_mut(&mut self, node: &BinaryTreeNode<T>) -> Option<&mut BinaryTreeNode<T>> {
        self.nodes.get_mut(node.right_id)
    }
    
    pub(super) fn remove_node(&mut self, node_id: NodeId) -> Option<BinaryTreeNode<T>> {
        if self.root_id == node_id {
            self.root_id = NodeId::null();
        }
        self.nodes.remove(node_id)
    }

    pub(super) fn add_edge(&mut self, parent_id: NodeId, child_id: NodeId, side: Side) -> bool {
        let Some([parent, child]) = self.disjoint_nodes_mut([parent_id, child_id]) else { return false; };
        if !parent.has_child(side) && !child.has_parent() {
            parent.set_child_id(child_id, side);
            child.set_parent_id(parent_id);
            true
        } else { false }
    }

    pub(super) fn remove_edge(&mut self, parent_id: NodeId, child_id: NodeId) -> bool {
        let Some([parent, child]) = self.disjoint_nodes_mut([parent_id, child_id]) else { return false; };
        if child.parent_id != parent_id { return false; }
        if parent.left_id == child_id {
            parent.nullify_left_id();
            child.nullify_parent_id();
            true
        } else if parent.right_id == child_id {
            parent.nullify_right_id();
            child.nullify_parent_id();
            true
        } else {
            false
        }
    }

    pub fn cursor(&self) -> Option<Cursor<'_, T>> {
        if !self.root_id.is_null() {
            Some(Cursor::new(self, self.root_id))
        } else { None }
    }

    pub fn cursor_mut(&mut self) -> Option<CursorMut<'_, T>> {
        if !self.root_id.is_null() {
            Some(CursorMut::new(self, self.root_id))
        } else { None }
    }
}
