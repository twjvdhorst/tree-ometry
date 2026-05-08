use std::{borrow::Borrow, collections::HashSet};

use slotmap::{SlotMap, new_key_type};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

impl Color {
    fn opposite(&self) -> Color {
        match self {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        }
    }
}

new_key_type! { pub(super) struct NodeId; }

pub struct RedBlackNode<K, V> {
    key: K,
    value: V,
    parent_id: Option<NodeId>,
    left_id: Option<NodeId>,
    right_id: Option<NodeId>,
    color: Color,
}

pub struct RedBlackTree<K, V> {
    root_id: Option<NodeId>,
    nodes: SlotMap<NodeId, RedBlackNode<K, V>>,
}

impl<K, V> RedBlackNode<K, V> {
    fn new_with_color(key: K, value: V, color: Color) -> Self {
        Self {
            key,
            value,
            parent_id: Option::None,
            left_id: Option::None,
            right_id: Option::None,
            color,
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
}

impl<K, V> Default for RedBlackTree<K, V> {
    fn default() -> Self {
        Self {
            root_id: Option::None,
            nodes: SlotMap::with_key(),
        }
    }
}

impl<K, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub(super) fn node(&self, node_id: NodeId) -> Option<&RedBlackNode<K, V>> {
        self.nodes.get(node_id)
    }

    pub(super) fn node_mut(&mut self, node_id: NodeId) -> Option<&mut RedBlackNode<K, V>> {
        self.nodes.get_mut(node_id)
    }

    pub fn root(&self) -> Option<&RedBlackNode<K, V>> {
        self.node(self.root_id?)
    }

    pub(super) fn left_child(&self, node: &RedBlackNode<K, V>) -> Option<&RedBlackNode<K, V>> {
        self.nodes.get(node.left_id?)
    }

    pub(super) fn left_child_mut(&mut self, node: &RedBlackNode<K, V>) -> Option<&mut RedBlackNode<K, V>> {
        self.nodes.get_mut(node.left_id?)
    }

    pub(super) fn right_child(&self, node: &RedBlackNode<K, V>) -> Option<&RedBlackNode<K, V>> {
        self.nodes.get(node.right_id?)
    }

    pub(super) fn right_child_mut(&mut self, node: &RedBlackNode<K, V>) -> Option<&mut RedBlackNode<K, V>> {
        self.nodes.get_mut(node.right_id?)
    }

    pub(super) fn parent(&self, node: &RedBlackNode<K, V>) -> Option<&RedBlackNode<K, V>> {
        self.nodes.get(node.parent_id?)
    }
    
    pub(super) fn parent_mut(&mut self, node: &RedBlackNode<K, V>) -> Option<&mut RedBlackNode<K, V>> {
        self.nodes.get_mut(node.parent_id?)
    }
}

impl<K, V> RedBlackTree<K, V>
where 
    K: Ord,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert_and_get_access_list(key, value).0
    }

    pub(crate) fn insert_and_get_access_list(&mut self, key: K, value: V) -> (Option<V>, HashSet<NodeId>) {
        todo!("Implement insertion, and maintain list of nodes accessed during the traversal.")
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.remove_and_get_access_list(key).0
    }

    pub(crate) fn remove_and_get_access_list<Q>(&mut self, key: &Q) -> (Option<(K, V)>, HashSet<NodeId>)
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        todo!("Implement deletion, and maintain list of nodes accessed during the traversal.")
    }
}
