use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt,
};

use crate::binary_search_trees::binary_search_tree_node::{
    BinarySearchTreeNode,
    BinaryTreeNode,
    Side,
};
use crate::binary_search_trees::tree_errors::StructureError;

pub struct RedBlackWrapper<N> {
    node: N,
    color: Color,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub(crate) enum Color {
    Red,
    Black,
}

impl<N> BinaryTreeNode for RedBlackWrapper<N>
where 
    N: BinaryTreeNode<Wrapper = Self>,
{
    type Wrapper = Self;
    type Edge = N::Edge;

    fn get_left_edge(&self) -> Option<&Self::Edge> { self.node.get_left_edge() }
    fn get_right_edge(&self) -> Option<&Self::Edge> { self.node.get_right_edge() }
    fn get_left_edge_mut(&mut self) -> Option<&mut Self::Edge> { self.node.get_left_edge_mut() }
    fn get_right_edge_mut(&mut self) -> Option<&mut Self::Edge> { self.node.get_right_edge_mut() }
    fn attach_left(&mut self, tree: impl Into<Self::Edge>) -> bool { self.node.attach_left(tree) }
    fn attach_right(&mut self, tree: impl Into<Self::Edge>) -> bool { self.node.attach_right(tree)}
    fn detach_left(&mut self) -> Option<Self::Edge> { self.node.detach_left() }
    fn detach_right(&mut self) -> Option<Self::Edge> { self.node.detach_right() }
    fn replace_left(&mut self, tree: impl Into<Self::Edge>) -> Option<Self::Edge> { self.node.replace_left(tree) }
    fn replace_right(&mut self, tree: impl Into<Self::Edge>) -> Option<Self::Edge> { self.node.replace_right(tree) }
}

impl<N> BinarySearchTreeNode for RedBlackWrapper<N>
where 
    N: BinarySearchTreeNode<Wrapper = Self>,
{
    type Key = N::Key;
    type Value = N::Value;

    fn new(key: Self::Key, value: Self::Value) -> Self {
        Self {
            node: N::new(key, value),
            color: Color::Black,
        }
    }

    fn key(&self) -> &Self::Key { self.node.key() }
    fn value(&self) -> &Self::Value { self.node.value() }
    fn replace_value(&mut self, value: Self::Value) -> Self::Value { self.node.replace_value(value) }
}

/// Construction methods.
impl<N> RedBlackWrapper<N>
where 
    N: BinarySearchTreeNode,
{
    fn new_with_color(key: N::Key, value: N::Value, color: Color) -> Self {
        Self {
            node: N::new(key, value),
            color,
        }
    }
}

/// Tree access.
impl<N> RedBlackWrapper<N>
where 
    N: BinarySearchTreeNode<Wrapper = Self>,
{
    pub(crate) fn color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

/// Queries.
impl<N> RedBlackWrapper<N>
where 
    N: BinarySearchTreeNode<Wrapper = Self>
{
    fn pick_branch<Q>(&self, value: &Q) -> Option<Side>
    where
        N::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match Q::cmp(value, self.key().borrow()) {
            Ordering::Less => Some(Side::Left),
            Ordering::Greater => Some(Side::Right),
            Ordering::Equal => None,
        }
    }

    /// Finds the predecessor of the given value, if it exists.
    /// Time complexity: O(log n).
    pub fn predecessor<Q>(&self, value: &Q) -> Option<&N::Key>
    where
        N::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match Q::cmp(value, self.key().borrow()) {
            Ordering::Equal => Some(self.key()),
            Ordering::Less => self.get_left_node()
                .and_then(|left| left.predecessor(value)),
            Ordering::Greater => self.get_right_node()
                .and_then(|right| right.predecessor(value))
                .or(Some(self.key())),
        }
    }

    /// Finds the successor of the given value, if it exists.
    /// Time complexity: O(log n).
    pub fn successor<Q>(&self, value: &Q) -> Option<&N::Key>
    where
        N::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match Q::cmp(value, self.key().borrow()) {
            Ordering::Equal => Some(self.key()),
            Ordering::Greater => self.get_right_node()
                .and_then(|right| right.successor(value)),
            Ordering::Less => self.get_left_node()
                .and_then(|left| left.successor(value))
                .or(Some(self.key())),
        }
    }

    /// Finds the stored value that equals the given value, if it exists.
    /// Time complexity: O(log n).
    pub fn get<Q>(&self, value: &Q) -> Option<&N::Key>
    where
        N::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match Q::cmp(value, self.key().borrow()) {
            Ordering::Equal => Some(self.key()),
            Ordering::Greater => self.get_right_node().and_then(|right| right.get(value)),
            Ordering::Less => self.get_left_node().and_then(|left| left.get(value)),
        }
    }
}

/// Tree operations.
impl<N> RedBlackWrapper<N>
where 
    N: BinarySearchTreeNode<Wrapper = Self>,
{
    /// Performs a left tree rotation, changing self to point to the new root.
    /// The function returns an error if the tree has an incorrect shape (i.e., is a leaf or has no right subtree).
    fn rotate_left(&mut self) -> Result<(), StructureError> {
        let mut new_root = self.detach_right().ok_or(StructureError::EmptyTree)?;

        // Change colors to keep red-black properties satisfied after rotation
        self.set_color(Color::Red);
        new_root.set_color(Color::Black);

        // Perform the rotation
        if let Some(rotating_subtree) = new_root.detach_left() {
            // There is a non-empty rotating subtree
            self.replace_right(rotating_subtree);
        }
        std::mem::swap(self, &mut new_root);
        self.replace_left(new_root);
        Ok(())
    }

    /// Performs a right tree rotation, changing self to point to the new root.
    /// The function returns an error if the tree has an incorrect shape (i.e., is a leaf or has no right subtree).
    fn rotate_right(&mut self) -> Result<(), StructureError> {
        let mut new_root = self.detach_left().ok_or(StructureError::EmptyTree)?;

        // Change colors to keep red-black properties satisfied after rotation
        self.set_color(Color::Red);
        new_root.set_color(Color::Black);

        // Perform the rotation
        if let Some(rotating_subtree) = new_root.detach_right() {
            // There is a non-empty rotating subtree
            self.replace_left(rotating_subtree);
        }
        std::mem::swap(self, &mut new_root);
        self.replace_right(new_root);
        Ok(())
    }

    fn double_rotate_left(&mut self) -> Result<(), StructureError> {
        let mut new_right = self.detach_right().ok_or(StructureError::EmptyTree)?;
        new_right.rotate_right()?;
        self.replace_right(new_right);
        self.rotate_left()
    }

    fn double_rotate_right(&mut self) -> Result<(), StructureError> {
        let mut new_left = self.detach_left().ok_or(StructureError::EmptyTree)?;
        new_left.rotate_left()?;
        self.replace_left(new_left);
        self.rotate_right()
    }
}

/// Insertions.
impl<N> RedBlackWrapper<N>
where 
    N: BinarySearchTreeNode<Wrapper = Self>,
{
    /// Swaps the colors of self and its children if both children (exist and) are red.
    fn color_swap(&mut self) {
        if let Some(left) = self.get_left_node() && let Some(right) = self.get_right_node()
            && left.color() == Color::Red && right.color() == Color::Red
        {
            self.set_color(Color::Red);
            self.get_left_node_mut().unwrap().set_color(Color::Black);
            self.get_right_node_mut().unwrap().set_color(Color::Black);
        }
    }

    fn fix_local_violation(&mut self, side1: Side, side2: Side) -> bool {
        if let Some(child) = self.get_child_node(side1) && let Some(grandchild) = child.get_child_node(side2)
            && child.color() == Color::Red && grandchild.color() == Color::Red
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
    pub fn insert(&mut self, key: N::Key, value: N::Value) -> Option<N::Value> {
        // Traverse the tree, keeping track of three nodes: the current node, its parent, and its grandparent.
        // To not need multiple mutable references into self, keep track of only the grandparent, together with the sides to take to get to parent and current.
        // We first handle the cases where there is no parent or no current, i.e., the path to the leaf where we put value is too short.

        // Check for a color swap at every node we come accross.
        self.color_swap();

        let Some(mut side1) = self.pick_branch(&key) else { 
            // self has the same key as the given key
            let old_value = self.replace_value(value);
            return Some(old_value);
        };
        if !self.has_child(side1) {
            // Insert the value in place of grandparent's child
            self.attach_child(side1, RedBlackWrapper::new_with_color(key, value, Color::Red));
            self.color_swap(); // Might need to color swap due to the insertion.
            self.set_color(Color::Black); // Maintain the invariant that the root is black.
            return None;
        }

        // Walk down the tree, updating the tree as we go.
        let mut grandparent = &mut *self;
        loop {
            let Some(child) = grandparent.get_child_node_mut(side1) else {
                grandparent.attach_child(side1, RedBlackWrapper::new_with_color(key, value, Color::Red));
                return None;
            };
            child.color_swap();

            let Some(side2) = child.pick_branch(&key) else {
                // child has the same key as the given key
                let old_value = child.replace_value(value);
                return Some(old_value);
            };
            let Some(grandchild) = child.get_child_node_mut(side2) else {
                child.attach_child(side2, RedBlackWrapper::new_with_color(key, value, Color::Red));
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
                    let old_value = grandparent.replace_value(value);
                    return Some(old_value);
                }
            } else { 
                // Structure of the tree is unchanged, can safely continue the search in a subtree of grandparent.
                grandparent = grandparent.get_child_node_mut(side1).unwrap();
                side1 = side2;
            }
        }

        // Reset the root color to black
        self.set_color(Color::Black);
        None
    }
}
    
impl<N> fmt::Display for RedBlackWrapper<N>
where 
    N: BinarySearchTreeNode<Wrapper = Self>,
    N::Key: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recursive_fmt<T>(root: Option<&RedBlackWrapper<T>>, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result
        where
            T: BinarySearchTreeNode<Wrapper = RedBlackWrapper<T>>,
            T::Key: fmt::Display,
        {
            write!(f, "{prefix}")?;
            if is_left {
                write!(f, "├──")?;
            } else {
                write!(f, "└──")?;
            };
            if let Some(root) = root {
                let color_text = match root.color() {
                    Color::Black => "b",
                    Color::Red => "r",
                };
                write!(f, "N({}, {color_text})\n", root.key())?;
                let new_prefix = String::from(prefix) + if is_left { "│  " } else { "   " };
                recursive_fmt(root.get_left_node(), f, &new_prefix, true)?;
                recursive_fmt(root.get_right_node(), f, &new_prefix, false)?;
                Ok(())
            } else {
                write!(f, "L\n")
            }
        }

        recursive_fmt(Some(self), f, "", false)
    }
}
