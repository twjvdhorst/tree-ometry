use std::fmt::{Debug, Display};
use paste::paste;

use slotmap::{Key, SlotMap, new_key_type};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::binary_trees::{
    Side, 
    traits::{
        self,
        binary_tree::{
            BinaryTree as BinaryTreeTrait, 
            BinaryTreeMut,
        },
    },
    tree_iterators::{self, *},
};
use super::cursors::{Cursor, CursorMut};

new_key_type! { pub(super) struct NodeId; }

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BinaryTreeNode<T> {
    data: T,
    #[cfg_attr(feature = "serde", serde(skip))]
    parent_id: NodeId,
    #[cfg_attr(feature = "serde", serde(skip))]
    left_id: NodeId,
    #[cfg_attr(feature = "serde", serde(skip))]
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

    pub fn into_data(self) -> T {
        self.data
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

    fn set_parent_id(&mut self, new_id: NodeId) {
        self.parent_id = new_id;
    }

    fn set_left_id(&mut self, new_id: NodeId) {
        self.left_id = new_id;
    }

    fn set_right_id(&mut self, new_id: NodeId) {
        self.right_id = new_id;
    }

    fn set_child_id(&mut self, new_id: NodeId, side: Side) {
        match side {
            Side::Left => self.set_left_id(new_id),
            Side::Right => self.set_right_id(new_id),
        }
    }

    pub(super) fn swap_children(&mut self) {
        std::mem::swap(&mut self.left_id, &mut self.right_id);
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize), serde(from = "super::serialization::SerializationTree<T>"))]
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

impl<T> BinaryTreeTrait for BinaryTree<T> {
    type Node = BinaryTreeNode<T>;
    type Cursor<'c> = Cursor<'c, T>
    where Self: 'c;

    fn cursor(&self) -> Self::Cursor<'_> {
        Cursor::new(self, self.root_id)
    }
}

impl<T> BinaryTreeMut for BinaryTree<T> {
    type CursorMut<'c> = CursorMut<'c, T>
    where Self: 'c;
    
    fn cursor_mut(&mut self) -> Self::CursorMut<'_> {
        CursorMut::new(self, self.root_id)
    }
}

impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_singleton(data: T) -> Self {
        let mut tree = Self::default();
        let root_id = tree.new_node(data);
        tree.root_id = root_id;
        tree
    }

    pub(super) fn new_node(&mut self, data: T) -> NodeId {
        let node_id = self.nodes.insert(BinaryTreeNode::new(data));
        if self.root_id.is_null() {
            self.root_id = node_id;
        }
        node_id
    }

    pub fn is_empty(&self) -> bool {
        self.root_id.is_null()
    }

    pub fn root(&self) -> Option<&BinaryTreeNode<T>> {
        self.nodes.get(self.root_id)
    }

    pub fn root_mut(&mut self) -> Option<&mut BinaryTreeNode<T>> {
        self.nodes.get_mut(self.root_id)
    }

    pub(super) fn root_id(&self) -> NodeId {
        self.root_id
    }

    pub(super) fn set_root_id(&mut self, root_id: NodeId) {
        self.root_id = root_id;
    }

    pub(super) fn node(&self, node_id: NodeId) -> Option<&BinaryTreeNode<T>> {
        self.nodes.get(node_id)
    }

    pub(super) fn node_mut(&mut self, node_id: NodeId) -> Option<&mut BinaryTreeNode<T>> {
        self.nodes.get_mut(node_id)
    }

    /// Returns mutable references to the nodes with the given ids.
    /// All ids must be valid and disjoint, otherwise None is returned.
    pub(super) fn get_disjoint_nodes_mut<const N: usize>(&mut self, node_ids: [NodeId; N]) -> Option<[&mut BinaryTreeNode<T>; N]> {
        self.nodes.get_disjoint_mut(node_ids)
    }

    /// Returns mutable references to the nodes with the given ids.
    /// All ids must be valid and disjoint, otherwise it is potentially unsafe.
    pub(super) unsafe fn get_disjoint_nodes_unchecked_mut<const N: usize>(&mut self, node_ids: [NodeId; N]) -> [&mut BinaryTreeNode<T>; N] {
        unsafe { self.nodes.get_disjoint_unchecked_mut(node_ids) }
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
    
    pub(super) fn remove_node(&mut self, node_id: NodeId) -> Option<T> {
        if self.root_id == node_id {
            self.root_id = NodeId::null();
        }
        self.nodes.remove(node_id).map(BinaryTreeNode::into_data)
    }

    /// Adds an edge to the tree without checking whether the tree remains a valid tree.
    /// The node ids are still checked to see if they are valid and disjoint.
    pub(super) fn add_edge_unchecked(&mut self, parent_id: NodeId, child_id: NodeId, side: Side) {
        let Some([parent, child]) = self.get_disjoint_nodes_mut([parent_id, child_id]) else { return; };
        parent.set_child_id(child_id, side);
        child.set_parent_id(parent_id);
    }

    pub(super) fn remove_edge(&mut self, parent_id: NodeId, child_id: NodeId) -> bool {
        let Some([parent, child]) = self.get_disjoint_nodes_mut([parent_id, child_id]) else { return false; };
        if child.parent_id != parent_id { return false; }
        if parent.left_id == child_id {
            parent.left_id = NodeId::null();
            child.parent_id = NodeId::null();
            true
        } else if parent.right_id == child_id {
            parent.right_id = NodeId::null();
            child.parent_id = NodeId::null();
            true
        } else {
            false
        }
    }

    tree_iterators::impl_iters!(pub, inorder, BinaryTreeNode<T>);
    tree_iterators::impl_iters!(pub, preorder, BinaryTreeNode<T>);
    tree_iterators::impl_iters!(pub, postorder, BinaryTreeNode<T>);
}

impl<T> Debug for BinaryTreeNode<T>
where 
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<T> Debug for BinaryTree<T>
where 
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        traits::binary_tree::fmt_debug_binary_tree(self, f)
    }
}

impl<T> Display for BinaryTreeNode<T>
where 
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<T> Display for BinaryTree<T>
where 
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        traits::binary_tree::fmt_display_binary_tree(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_trees::traits::binary_tree_cursor::{BinaryTreeCursor, PeekingCursor, PeekingCursorMut};
    
    #[test]
    fn test_cursors() {
        let mut tree = BinaryTree::new();
        let mut cursor = tree.cursor_mut();
        
        // Create the tree.
        cursor.root_tree(1).unwrap();
        cursor.attach_child(2, Side::Left).unwrap();
        cursor.attach_child(5, Side::Right).unwrap();
        cursor.move_left();
        cursor.attach_child(3, Side::Left).unwrap();
        cursor.attach_child(4, Side::Right).unwrap();

        // Check creation of tree went correctly.
        let mut cursor = tree.cursor();
        assert_eq!(cursor.node().map(BinaryTreeNode::data), Some(&1));
        assert_eq!(cursor.peek_left().map(BinaryTreeNode::data), Some(&2));
        assert_eq!(cursor.peek_right().map(BinaryTreeNode::data), Some(&5));
        cursor.move_left();
        assert_eq!(cursor.peek_left().map(BinaryTreeNode::data), Some(&3));
        assert_eq!(cursor.peek_right().map(BinaryTreeNode::data), Some(&4));
        cursor.move_up();
        cursor.move_right();
        assert_eq!(cursor.node().map(BinaryTreeNode::data), Some(&5));

        // Test rotations.
        let mut cursor = tree.cursor_mut();
        cursor.rotate_right().unwrap();
        cursor.move_up();

        assert_eq!(cursor.node().map(BinaryTreeNode::data), Some(&2));
        assert_eq!(cursor.peek_left().map(BinaryTreeNode::data), Some(&3));
        assert_eq!(cursor.peek_right().map(BinaryTreeNode::data), Some(&1));
        cursor.move_right();
        assert_eq!(cursor.peek_left().map(BinaryTreeNode::data), Some(&4));
        assert_eq!(cursor.peek_right().map(BinaryTreeNode::data), Some(&5));
    }
}

