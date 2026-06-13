use std::{collections::HashMap, fmt::{self, Debug, Display}};

use slotmap::{Key, SlotMap, new_key_type};

use super::{
    InorderIter,
    InorderIterMut,
    IntoInorderIter,
};
use crate::binary_trees::{
    Side,
    binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursor,
        PeekingCursorMut,
    },
};
use super::cursors::{
    Cursor,
    CursorMut,
};

new_key_type! { pub(crate) struct NodeId; }

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct BinaryTreeNode<T> {
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

    pub(crate) fn data(&self) -> &T {
        &self.data
    }

    pub(crate) fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    fn into_data(self) -> T {
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

    pub fn new_singleton(data: T) -> Self {
        Self::new_singleton_with_id(|_| data)
    }

    pub(crate) fn new_singleton_with_id<F>(f: F) -> Self
    where 
        F: FnOnce(NodeId) -> T,
    {
        let mut tree = Self::default();
        tree.new_node_with_id(f);
        tree
    }

    pub(super) fn new_node(&mut self, data: T) -> NodeId {
        self.new_node_with_id(|_| data)
    }

    pub(super) fn new_node_with_id<F>(&mut self, f: F) -> NodeId
    where 
        F: FnOnce(NodeId) -> T,
    {
        let node_id = self.nodes.insert_with_key(|id| BinaryTreeNode::new(f(id)));
        if self.root_id.is_null() {
            self.root_id = node_id;
        }
        node_id
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: SlotMap::with_capacity_and_key(capacity),
            root_id: NodeId::null(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root_id.is_null()
    }

    pub(crate) fn root_mut(&mut self) -> Option<&mut BinaryTreeNode<T>> {
        self.nodes.get_mut(self.root_id)
    }

    pub(super) fn root_id(&self) -> NodeId {
        self.root_id
    }

    pub(super) fn set_root_id(&mut self, root_id: NodeId) {
        self.root_id = root_id;
    }

    pub(crate) fn node(&self, node_id: NodeId) -> Option<&BinaryTreeNode<T>> {
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

    pub(super) fn parent_id(&self, node_id: NodeId) -> Option<NodeId> {
        self.node(node_id).map(|node| node.parent_id)
    }

    pub(super) fn left_id(&self, node_id: NodeId) -> Option<NodeId> {
        self.node(node_id).map(|node| node.left_id)
    }

    pub(super) fn right_id(&self, node_id: NodeId) -> Option<NodeId> {
        self.node(node_id).map(|node| node.right_id)
    }

    pub(super) fn child_id(&self, node_id: NodeId, side: Side) -> Option<NodeId> {
        match side {
            Side::Left => self.left_id(node_id),
            Side::Right => self.right_id(node_id),
        }
    }

    pub(super) fn parent(&self, node_id: NodeId) -> Option<&BinaryTreeNode<T>> {
        self.parent_id(node_id).and_then(|parent_id| self.node(parent_id))
    }

    pub(super) fn left(&self, node_id: NodeId) -> Option<&BinaryTreeNode<T>> {
        self.left_id(node_id).and_then(|left_id| self.node(left_id))
    }

    pub(super) fn right(&self, node_id: NodeId) -> Option<&BinaryTreeNode<T>> {
        self.right_id(node_id).and_then(|right_id| self.node(right_id))
    }

    pub(super) fn parent_mut(&mut self, node_id: NodeId) -> Option<&mut BinaryTreeNode<T>> {
        self.parent_id(node_id).and_then(|parent_id| self.node_mut(parent_id))
    }

    pub(super) fn left_mut(&mut self, node_id: NodeId) -> Option<&mut BinaryTreeNode<T>> {
        self.left_id(node_id).and_then(|left_id| self.node_mut(left_id))
    }

    pub(super) fn right_mut(&mut self, node_id: NodeId) -> Option<&mut BinaryTreeNode<T>> {
        self.right_id(node_id).and_then(|right_id| self.node_mut(right_id))
    }
    
    pub(super) fn remove_node(&mut self, node_id: NodeId) -> Option<T> {
        if self.root_id == node_id {
            self.root_id = NodeId::null();
        }
        // Remove all edges incident to the removed node.
        if let Some(parent_id) = self.parent_id(node_id) {
            self.remove_edge(parent_id, node_id);
        }
        if let Some(left_id) = self.left_id(node_id) {
            self.remove_edge(node_id, left_id);
        }
        if let Some(right_id) = self.right_id(node_id) {
            self.remove_edge(node_id, right_id);
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
    
    /// Helper method that turns the tree into a path where every node has at most one child, on their right.
    fn to_path(&mut self) {
        let mut cursor = self.cursor_mut();
        while cursor.get().is_some() {
            if cursor.peek_left().is_some() {
                cursor.rotate_right().unwrap();
                cursor.move_up();
            } else {
                cursor.move_right();
            }
        }
    }

    fn compress(&mut self, count: usize) {
        let mut cursor = self.cursor_mut();
        for _ in 0..count {
            cursor.rotate_left().unwrap();
            cursor.move_up();
            cursor.move_right();
        }
    }

    /// Rebalances the tree in place, making the tree perfectly balanced (all leaves differ in depth by at most one).
    /// Time complexity: O(n).
    pub fn rebalance(&mut self) {
        // Day-Stout-Warren algorithm.
        // Transform the tree into a path, and unfold this path to a balanced binary search tree.
        self.to_path();
        let size = self.len();
        let mut k = (1 << usize::ilog2(size + 1)) - 1;
        self.compress(size - k);
        
        while k > 1 {
            k /= 2;
            self.compress(k);
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn map<U, F>(self, mut f: F) -> BinaryTree<U>
    where 
        F: FnMut(T) -> U,
    {
        let old_root_id = self.root_id;
        if old_root_id.is_null() {
            return BinaryTree::new();
        }

        let mut new_nodes = SlotMap::with_capacity_and_key(self.nodes.len());
        let mut keys_map = HashMap::with_capacity(self.nodes.len());

        // First map the function over the nodes, and move the nodes to a new map.
        // The keys stored in nodes are incorrect; fix these afterwards.
        for (key, node) in self.nodes.into_iter() {
            let new_node = BinaryTreeNode {
                data: f(node.data),
                parent_id: node.parent_id,
                left_id: node.left_id,
                right_id: node.right_id,
            };
            let new_key = new_nodes.insert(new_node);
            keys_map.insert(key, new_key);
        }

        // Fix the keys.
        for node in new_nodes.values_mut() {
            if !node.parent_id.is_null() {
                node.parent_id = keys_map[&node.parent_id];
            }

            if !node.left_id.is_null() {
                node.left_id = keys_map[&node.left_id];
            }

            if !node.right_id.is_null() {
                node.right_id = keys_map[&node.right_id];
            }
        }

        BinaryTree {
            root_id: keys_map[&old_root_id],
            nodes: new_nodes,
        }
    }
    
    pub fn cursor(&self) -> Cursor<'_, T> {
        Cursor::new(self, self.root_id)
    }

    pub fn cursor_mut(&mut self) -> CursorMut<'_, T> {
        CursorMut::new(self, self.root_id)
    }
}

impl<'t, T> IntoIterator for &'t BinaryTree<T> {
    type Item = &'t T;
    type IntoIter = InorderIter<'t, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder_iter()
    }
}

impl<'t, T> IntoIterator for &'t mut BinaryTree<T> {
    type Item = &'t mut T;
    type IntoIter = InorderIterMut<'t, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder_iter_mut()
    }
}

impl<T> IntoIterator for BinaryTree<T> {
    type Item = T;
    type IntoIter = IntoInorderIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_inorder_iter()
    }
}

impl<T> Debug for BinaryTreeNode<T>
where 
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}

impl<T> Debug for BinaryTree<T>
where 
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recursive_fmt<'t, T>(cursor: Cursor<'t, T>, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result
        where
            T: Debug,
        {
            write!(f, "{prefix}")?;
            if is_left {
                write!(f, "├──")?;
            } else {
                write!(f, "└──")?;
            };
            if let Some(node) = cursor.get() {
                node.fmt(f)?;
                writeln!(f, "")?;
                let new_prefix = String::from(prefix) + if is_left { "│  " } else { "   " };
                let mut left_cursor = cursor.clone();
                let mut right_cursor = cursor.clone();
                if left_cursor.try_move_left() {
                    recursive_fmt(left_cursor, f, &new_prefix, true)?;
                }
                if right_cursor.try_move_right() {
                    recursive_fmt(right_cursor, f, &new_prefix, false)?;
                }
                Ok(())
            } else {
                write!(f, "L\n")
            }
        }
            
        write!(f, "\n")?;
        recursive_fmt(self.cursor(), f, "", false)
    }
}

impl<T> Display for BinaryTreeNode<T>
where 
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}

impl<T> Display for BinaryTree<T>
where 
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recursive_fmt<'t, T>(cursor: Cursor<'t, T>, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result
        where
            T: Display,
        {
            write!(f, "{prefix}")?;
            if is_left {
                write!(f, "├──")?;
            } else {
                write!(f, "└──")?;
            };
            if let Some(node) = cursor.get() {
                node.fmt(f)?;
                writeln!(f, "")?;
                let new_prefix = String::from(prefix) + if is_left { "│  " } else { "   " };
                let mut left_cursor = cursor.clone();
                let mut right_cursor = cursor.clone();
                if left_cursor.try_move_left() {
                    recursive_fmt(left_cursor, f, &new_prefix, true)?;
                }
                if right_cursor.try_move_right() {
                    recursive_fmt(right_cursor, f, &new_prefix, false)?;
                }
                Ok(())
            } else {
                write!(f, "L\n")
            }
        }
            
        write!(f, "\n")?;
        recursive_fmt(self.cursor(), f, "", false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
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
        assert_eq!(cursor.get(), Some(&1));
        assert_eq!(cursor.peek_left(), Some(&2));
        assert_eq!(cursor.peek_right(), Some(&5));
        cursor.move_left();
        assert_eq!(cursor.peek_left(), Some(&3));
        assert_eq!(cursor.peek_right(), Some(&4));
        cursor.move_up();
        cursor.move_right();
        assert_eq!(cursor.get(), Some(&5));

        // Test rotations.
        let mut cursor = tree.cursor_mut();
        cursor.rotate_right().unwrap();
        cursor.move_up();

        assert_eq!(cursor.get(), Some(&2));
        assert_eq!(cursor.peek_left(), Some(&3));
        assert_eq!(cursor.peek_right(), Some(&1));
        cursor.move_right();
        assert_eq!(cursor.peek_left(), Some(&4));
        assert_eq!(cursor.peek_right(), Some(&5));
    }

    fn get_iter_test_tree() -> BinaryTree<i32> {
        // Makes the following test tree:
        // └──1
        //    ├──2
        //    │  ├──3
        //    │  └──4
        //    │     └──5
        //    └──6
        //       ├──7
        //       └──8
        //          └──9
        let mut tree = BinaryTree::new();
        let mut cursor = tree.cursor_mut();
        cursor.root_tree(1).unwrap();
        cursor.attach_child(2, Side::Left).unwrap();
        cursor.move_left();
        cursor.attach_child(3, Side::Left).unwrap();
        cursor.attach_child(4, Side::Right).unwrap();
        cursor.move_right();
        cursor.attach_child(5, Side::Left).unwrap();
        let mut cursor = tree.cursor_mut();
        cursor.attach_child(6, Side::Right).unwrap();
        cursor.move_right();
        cursor.attach_child(7, Side::Left).unwrap();
        cursor.attach_child(8, Side::Right).unwrap();
        cursor.move_right();
        cursor.attach_child(9, Side::Left).unwrap();
        tree
    }

    #[test]
    fn test_inorder_iters() {
        let mut tree = get_iter_test_tree();
        let mut inorder_sequence = [3, 2, 5, 4, 1, 7, 6, 9, 8];

        // Testing iters without mutations.
        assert!(Iterator::eq(inorder_sequence.iter(), tree.inorder_iter()));
        assert!(Iterator::eq(inorder_sequence.iter_mut(), tree.inorder_iter_mut()));
        assert!(Iterator::eq(inorder_sequence.clone().into_iter(), tree.into_inorder_iter()));

        // Test mutating elements during iteration.
        let mut tree = get_iter_test_tree();
        for i in tree.inorder_iter_mut() {
            *i = 2 * *i;
        }
        assert!(Iterator::eq(
            inorder_sequence.map(|i| 2 * i).iter_mut(),
            tree.inorder_iter_mut()
        ));

        // Test iterating the subtree rooted at 2.
        let mut tree = get_iter_test_tree();
        let mut inorder_subtree_sequence = [3, 2, 5, 4];

        let mut cursor = tree.cursor();
        cursor.move_left();
        assert!(Iterator::eq(
            inorder_subtree_sequence.iter(), 
            cursor.inorder_subtree_iter(),
        ));

        let mut cursor = tree.cursor_mut();
        cursor.move_left();
        assert!(Iterator::eq(
            inorder_subtree_sequence.iter_mut(),
            cursor.inorder_subtree_iter_mut(),
        ));
        
        let mut cursor = tree.cursor_mut();
        cursor.move_left();
        assert!(Iterator::eq(
            inorder_subtree_sequence.clone().into_iter(),
            cursor.drain_subtree_inorder(),
        ));
    }

    #[test]
    fn test_preorder_iters() {
        let mut tree = get_iter_test_tree();
        let mut preorder_sequence = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        // Testing iters without mutations.
        assert!(Iterator::eq(preorder_sequence.iter(), tree.preorder_iter()));
        assert!(Iterator::eq(preorder_sequence.iter_mut(), tree.preorder_iter_mut()));
        assert!(Iterator::eq(preorder_sequence.clone().into_iter(), tree.into_preorder_iter()));

        // Test mutating elements during iteration.
        let mut tree = get_iter_test_tree();
        for i in tree.preorder_iter_mut() {
            *i = 2 * *i;
        }
        assert!(Iterator::eq(
            preorder_sequence.map(|i| 2 * i).iter_mut(),
            tree.preorder_iter_mut()
        ));

        // Test iterating the subtree rooted at 2.
        let mut tree = get_iter_test_tree();
        let mut preorder_subtree_sequence = [2, 3, 4, 5];

        let mut cursor = tree.cursor();
        cursor.move_left();
        assert!(Iterator::eq(
            preorder_subtree_sequence.iter(), 
            cursor.preorder_subtree_iter(),
        ));

        let mut cursor = tree.cursor_mut();
        cursor.move_left();
        assert!(Iterator::eq(
            preorder_subtree_sequence.iter_mut(),
            cursor.preorder_subtree_iter_mut(),
        ));
        
        let mut cursor = tree.cursor_mut();
        cursor.move_left();
        assert!(Iterator::eq(
            preorder_subtree_sequence.clone().into_iter(),
            cursor.drain_subtree_preorder(),
        ));
    }

    #[test]
    fn test_postorder_iters() {
        let mut tree = get_iter_test_tree();
        let mut postorder_sequence = [3, 5, 4, 2, 7, 9, 8, 6, 1];

        // Testing iters without mutations.
        assert!(Iterator::eq(postorder_sequence.iter(), tree.postorder_iter()));
        assert!(Iterator::eq(postorder_sequence.iter_mut(), tree.postorder_iter_mut()));
        assert!(Iterator::eq(postorder_sequence.clone().into_iter(), tree.into_postorder_iter()));

        // Test mutating elements during iteration.
        let mut tree = get_iter_test_tree();
        for i in tree.postorder_iter_mut() {
            *i = 2 * *i;
        }
        assert!(Iterator::eq(
            postorder_sequence.map(|i| 2 * i).iter_mut(),
            tree.postorder_iter_mut()
        ));

        // Test iterating the subtree rooted at 2.
        let mut tree = get_iter_test_tree();
        let mut postorder_subtree_sequence = [3, 5, 4, 2];

        let mut cursor = tree.cursor();
        cursor.move_left();
        assert!(Iterator::eq(
            postorder_subtree_sequence.iter(), 
            cursor.postorder_subtree_iter(),
        ));

        let mut cursor = tree.cursor_mut();
        cursor.move_left();
        assert!(Iterator::eq(
            postorder_subtree_sequence.iter_mut(),
            cursor.postorder_subtree_iter_mut(),
        ));
        
        let mut cursor = tree.cursor_mut();
        cursor.move_left();
        assert!(Iterator::eq(
            postorder_subtree_sequence.clone().into_iter(),
            cursor.drain_subtree_postorder(),
        ));
    }
}