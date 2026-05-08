use crate::binary_trees::traits::binary_tree_cursor::{BinaryTreeCursor, BinaryTreeCursorMut};
use super::red_black_tree::{NodeId, RedBlackNode, RedBlackTree};

pub struct RedBlackTreeCursor<'t, K, V> {
    tree: &'t RedBlackTree<K, V>,
    node_id: NodeId,
}

impl<'t, K, V> RedBlackTreeCursor<'t, K, V> {
    pub(super) fn new(tree: &'t RedBlackTree<K, V>, node_id: NodeId) -> Self {
        Self {
            tree,
            node_id,
        }
    }
}

impl<'t, K, V> BinaryTreeCursor for RedBlackTreeCursor<'t, K, V> {
    type Node = RedBlackNode<K, V>;

    fn node(&self) -> Option<&Self::Node> {
        self.tree.node(self.node_id)
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

pub(crate) struct RedBlackTreeCursorMut<'t, K, V> {
    tree: &'t mut RedBlackTree<K, V>,
    node_id: NodeId,
}

impl<'t, K, V> RedBlackTreeCursorMut<'t, K, V> {
    pub(super) fn new(tree: &'t mut RedBlackTree<K, V>, node_id: NodeId) -> Self {
        Self {
            tree,
            node_id,
        }
    }
}

impl<'t, K, V> BinaryTreeCursor for RedBlackTreeCursorMut<'t, K, V> {
    type Node = RedBlackNode<K, V>;

    fn node(&self) -> Option<&Self::Node> {
        self.tree.node(self.node_id)
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

impl<'t, K, V> BinaryTreeCursorMut for RedBlackTreeCursorMut<'t, K, V> {
    fn node_mut(&mut self) -> Option<&mut Self::Node> {
        self.tree.node_mut(self.node_id)
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
