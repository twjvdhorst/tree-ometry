use serde::{Serialize, Deserialize};

use crate::binary_trees::{
    Side, 
    binary_tree::{
        Cursor, 
        NodeId,
    },
    binary_tree_cursor::{
        BinaryTreeCursor, 
        PeekingCursor,
    },
};
use super::BinaryTree;

impl<T> Serialize for BinaryTree<T>
where 
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        SerializationTree::new(self).serialize(serializer)
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializationNode<T> {
    pub data: T,
    pub left: Option<Box<SerializationNode<T>>>,
    pub right: Option<Box<SerializationNode<T>>>,
}

impl<'t, T> SerializationNode<&'t T> {
    fn new(cursor: Cursor<'t, T>) -> Option<Self> {
        let data = cursor.get()?;

        let mut left_cursor = cursor.spawn_cursor();
        let mut right_cursor = cursor.spawn_cursor();
        left_cursor.move_left();
        right_cursor.move_right();

        Some(Self {
            data,
            left: Self::new(left_cursor).map(Box::new),
            right: Self::new(right_cursor).map(Box::new),
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializationTree<T>(pub Option<SerializationNode<T>>);

impl<'t, T> SerializationTree<&'t T> {
    pub fn new(tree: &'t BinaryTree<T>) -> Self {
        Self(SerializationNode::new(tree.cursor()))
    }
}

impl<T> From<SerializationTree<T>> for BinaryTree<T> {
    fn from(value: SerializationTree<T>) -> Self {
        fn from_recursive<T>(tree: &mut BinaryTree<T>, node: SerializationNode<T>, parent_id: NodeId, side: Side) {
            let SerializationNode { data, left, right } = node;
            let new_id = tree.new_node(data);
            tree.add_edge_unchecked(parent_id, new_id, side);
            if let Some(left) = left {
                from_recursive(tree, *left, new_id, Side::Left);
            }

            if let Some(right) = right {
                from_recursive(tree, *right, new_id, Side::Right);
            }
        }

        let mut tree = Self::default();
        if let Some(SerializationNode { data, left, right }) = value.0 {
            let node_id = tree.new_node(data);
            if let Some(left) = left {
                from_recursive(&mut tree, *left, node_id, Side::Left);
            }

            if let Some(right) = right {
                from_recursive(&mut tree, *right, node_id, Side::Right);
            }
        }

        tree
    }
}
