use std::{
    cmp::Ordering,
    fmt,
};

use crate::binary_search_trees::binary_search_tree_node::{BSTNode, Side};
use crate::binary_search_trees::node_traits::{BinarySearchTreeNode, BinaryTree, BinaryTreeNode};
use crate::binary_search_trees::tree_errors::StructureError;

pub struct RedBlackNode<K, V, T> {
    node: BSTNode<K, V, T>,
    pub(super) color: Color,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

impl<K, V, T> RedBlackNode<K, V, T> 
where 
    T: BinaryTree,
{
    pub(super) fn new(key: K, value: V) -> Self {
        Self::new_with_color(key, value, Color::Black)
    }

    fn new_with_color(key: K, value: V, color: Color) -> Self {
        Self {
            node: BSTNode::new(key, value),
            color,
        }
    }
}

impl<K, V, T> BinaryTreeNode for RedBlackNode<K, V, T>
where 
    T: BinaryTree,
{
    type Tree = T;

    fn left_subtree(&self) -> &Self::Tree {
        self.node.left_subtree()
    }

    fn right_subtree(&self) -> &Self::Tree {
        self.node.right_subtree()
    }

    fn left_subtree_mut(&mut self) -> &mut Self::Tree {
        self.node.left_subtree_mut()
    }

    fn right_subtree_mut(&mut self) -> &mut Self::Tree {
        self.node.right_subtree_mut()
    }

    fn attach_left(&mut self, tree: impl Into<Self::Tree>) -> bool {
        self.node.attach_left(tree)
    }
    
    fn attach_right(&mut self, tree: impl Into<Self::Tree>) -> bool {
        self.node.attach_right(tree)
    }

    fn attach_subtree(&mut self, side: Side, tree: impl Into<Self::Tree>) -> bool {
        self.node.attach_subtree(side, tree)
    }
    
    fn detach_left(&mut self) -> Self::Tree {
        self.node.detach_left()
    }
    
    fn detach_right(&mut self) -> Self::Tree {
        self.node.detach_right()
    }

    fn detach_subtree(&mut self, side: Side) -> Self::Tree {
        self.node.detach_subtree(side)
    }
    
    fn replace_left(&mut self, tree: impl Into<Self::Tree>) -> Self::Tree {
        self.node.replace_left(tree)
    }
    
    fn replace_right(&mut self, tree: impl Into<Self::Tree>) -> Self::Tree {
        self.node.replace_right(tree)
    }

    fn replace_subtree(&mut self, side: Side, tree: impl Into<Self::Tree>) -> Self::Tree {
        self.node.replace_subtree(side, tree)
    }
}

impl<K, V, T> BinarySearchTreeNode for RedBlackNode<K, V, T>
where 
    K: Ord,
{
    type Key = K;
    type Value = V;

    fn key(&self) -> &Self::Key {
        self.node.key()
    }

    fn value(&self) -> &Self::Value {
        self.node.value()
    }

    fn value_mut(&mut self) -> &mut Self::Value {
        self.node.value_mut()
    }
}

/// Tree operations.
impl<K, V, T> RedBlackNode<K, V, T>
where 
    T: BinaryTree<Node = Self>,
{
    /// Performs a left tree rotation, changing self to point to the new root.
    /// The function returns an error if the tree has an incorrect shape (i.e., is a leaf or has no right subtree).
    fn rotate_left(&mut self) -> Result<(), StructureError> {
        let mut right = self.detach_right();
        let mut new_root = right.root_mut().ok_or(StructureError::EmptyTree)?;

        // Change colors to keep red-black properties satisfied after rotation
        self.color = Color::Red;
        new_root.color = Color::Black;

        // Perform the rotation
        self.replace_right(new_root.detach_left());
        //if let Some(rotating_subtree) = new_root.detach_left() {
            // There is a non-empty rotating subtree
        //    self.replace_right(rotating_subtree);
        //}
        std::mem::swap(self, &mut new_root);
        self.replace_left(right);
        Ok(())
    }

    /// Performs a right tree rotation, changing self to point to the new root.
    /// The function returns an error if the tree has an incorrect shape (i.e., is a leaf or has no right subtree).
    fn rotate_right(&mut self) -> Result<(), StructureError> {
        let mut left = self.detach_left();
        let mut new_root = left.root_mut().ok_or(StructureError::EmptyTree)?;

        // Change colors to keep red-black properties satisfied after rotation
        self.color = Color::Red;
        new_root.color = Color::Black;

        // Perform the rotation
        self.replace_left(new_root.detach_right());
        //if let Some(rotating_subtree) = new_root.detach_left() {
            // There is a non-empty rotating subtree
        //    self.replace_right(rotating_subtree);
        //}
        std::mem::swap(self, &mut new_root);
        self.replace_right(left);
        Ok(())
    }

    fn double_rotate_left(&mut self) -> Result<(), StructureError> {
        let mut right = self.detach_right();
        right.root_mut().ok_or(StructureError::EmptyTree)?.rotate_right()?;
        self.replace_right(right);
        self.rotate_left()
    }

    fn double_rotate_right(&mut self) -> Result<(), StructureError> {
        let mut left = self.detach_left();
        left.root_mut().ok_or(StructureError::EmptyTree)?.rotate_left()?;
        self.replace_left(left);
        self.rotate_right()
    }
}

/// Insertions.
impl<K, V, T> RedBlackNode<K, V, T>
where 
    K: Ord,
    T: BinaryTree<Node = Self>,
{
    fn pick_branch(&self, value: &K) -> Option<Side> {
        match K::cmp(value, self.key()) {
            Ordering::Less => Some(Side::Left),
            Ordering::Greater => Some(Side::Right),
            Ordering::Equal => None,
        }
    }

    /// Swaps the colors of self and its children if both children (exist and) are red.
    fn color_swap(&mut self) {
        if let Some(left) = self.left_subtree().root() && let Some(right) = self.right_subtree().root()
            && left.color == Color::Red && right.color == Color::Red
        {
            self.color = Color::Red;
            self.left_subtree_mut().root_mut().unwrap().color = Color::Black;
            self.right_subtree_mut().root_mut().unwrap().color = Color::Black;
        }
    }

    fn fix_local_violation(&mut self, side1: Side, side2: Side) -> bool {
        if let Some(child) = self.subtree(side1).root() && let Some(grandchild) = child.subtree(side2).root()
            && child.color == Color::Red && grandchild.color == Color::Red
        {
            match (side1, side2) {
                // Can safely ignore the result of the performed rotation, as the existing child and grandchild nodes imply the rotation won't fail
                (Side::Left, Side::Left) => self.rotate_right().unwrap(),
                (Side::Right, Side::Right) => self.rotate_left().unwrap(),
                (Side::Left, Side::Right) => self.double_rotate_right().unwrap(),
                (Side::Right, Side::Left) => self.double_rotate_left().unwrap(),
            }
            true
        } else { false }
    }

    /// Inserts the key-data pair into the tree.
    /// If the key was not present in the tree yet, None is returned.
    /// Otherwise, the data stored at the given key is updated, and the old data is returned.
    /// Time complexity: O(log n).
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // Traverse the tree, keeping track of three nodes: the current node, its parent, and its grandparent.
        // To not need multiple mutable references into self, keep track of only the grandparent, together with the sides to take to get to parent and current.
        // We first handle the cases where there is no parent or no current, i.e., the path to the leaf where we put value is too short.

        // Check for a color swap at every node we come accross.
        self.color_swap();

        let Some(mut side1) = self.pick_branch(&key) else { 
            // self has the same key as the given key
            let old_value = std::mem::replace(self.value_mut(), value);
            return Some(old_value);
        };
        if !self.has_child(side1) {
            // Insert the value in place of grandparent's child
            self.attach_subtree(side1, T::new(Self::new_with_color(key, value, Color::Red)));
            self.color_swap(); // Might need to color swap due to the insertion.
            self.color = Color::Black; // Maintain the invariant that the root is black.
            return None;
        }

        // Walk down the tree, updating the tree as we go.
        let mut grandparent = &mut *self;
        loop {
            let Some(child) = grandparent.subtree_mut(side1).root_mut() else {
                grandparent.attach_subtree(side1, T::new(Self::new_with_color(key, value, Color::Red)));
                return None;
            };
            child.color_swap();

            let Some(side2) = child.pick_branch(&key) else {
                // child has the same key as the given key
                let old_value = std::mem::replace(child.value_mut(), value);
                return Some(old_value);
            };
            let Some(grandchild) = child.subtree_mut(side2).root_mut() else {
                child.attach_subtree(side2, T::new(Self::new_with_color(key, value, Color::Red)));
                grandparent.fix_local_violation(side1, side2);
                break;
            };
            grandchild.color_swap();

            let has_changed = grandparent.fix_local_violation(side1, side2);
            if has_changed {
                // Need to do comparison again, grandparent has been changed, and side1 and side2 might have been changed with it.
                if let Some(side) = grandparent.pick_branch(&key) {
                    side1 = side;
                } else {
                    // grandparent has the same key as the given key
                    let old_value = std::mem::replace(grandparent.value_mut(), value);
                    return Some(old_value);
                }
            } else { 
                // Structure of the tree is unchanged, can safely continue the search in a subtree of grandparent.
                grandparent = grandparent.subtree_mut(side1).root_mut().unwrap();
                side1 = side2;
            }
        }

        // Reset the root color to black
        self.color = Color::Black;
        None
    }
}
