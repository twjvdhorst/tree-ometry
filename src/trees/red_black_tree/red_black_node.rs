/// The internals of the red-black tree. The tree stores only unique keys.

use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt,
};
use crate::trees::tree_errors::StructureError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RBNode<T> {
    key: T,
    color: Color,
    left: Option<RBPointer<T>>,
    right: Option<RBPointer<T>>,
}

pub type RBPointer<T> = Box<RBNode<T>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

impl<T> RBNode<T> {
    /// Creates a new root node
    pub fn new(key: T) -> Self {
        Self {
            key,
            color: Color::Black,
            left: None,
            right: None,
        }
    }

    fn new_with_color(key: T, color: Color) -> Self {
        Self {
            key,
            color,
            left: None,
            right: None,
        }
    }
}

/// Tree access
impl<T> RBNode<T> {
    pub fn key(&self) -> &T {
        &self.key
    }

    fn color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color
    }

    pub fn get_child(&self, side: Side) -> Option<&Self> {
        match side {
            Side::Left => self.get_left(),
            Side::Right => self.get_right(),
        }
    }

    pub fn get_left(&self) -> Option<&Self> {
        self.left.as_ref().map(|left| left.as_ref())
    }

    pub fn get_right(&self) -> Option<&Self> {
        self.right.as_ref().map(|right| right.as_ref())
    }

    fn get_child_mut(&mut self, side: Side) -> Option<&mut Self> {
        match side {
            Side::Left => self.get_left_mut(),
            Side::Right => self.get_right_mut(),
        }
    }

    fn get_left_mut(&mut self) -> Option<&mut Self> {
        self.left.as_mut().map(|left| left.as_mut())
    }

    fn get_right_mut(&mut self) -> Option<&mut Self> {
        self.right.as_mut().map(|right| right.as_mut())
    }

    pub fn has_child(&self, side: Side) -> bool {
        match side {
            Side::Left => self.has_left(),
            Side::Right => self.has_right(),
        }
    }

    pub fn has_left(&self) -> bool {
        self.left.is_none()
    }

    pub fn has_right(&self) -> bool {
        self.right.is_none()
    }
}

/// Queries
impl<T> RBNode<T> {
    /// Finds the predecessor of the given value, if it exists.
    /// Time complexity: O(log n).
    pub fn predecessor<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match Q::cmp(value, self.key().borrow()) {
            Ordering::Equal => Some(self.key()),
            Ordering::Less => self.get_left()
                .and_then(|left| left.predecessor(value)),
            Ordering::Greater => self.get_right()
                .and_then(|right| right.predecessor(value))
                .or(Some(self.key())),
        }
    }

    /// Finds the successor of the given value, if it exists.
    /// Time complexity: O(log n).
    pub fn successor<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match Q::cmp(value, self.key().borrow()) {
            Ordering::Equal => Some(self.key()),
            Ordering::Greater => self.get_right()
                .and_then(|right| right.successor(value)),
            Ordering::Less => self.get_left()
                .and_then(|left| left.successor(value))
                .or(Some(self.key())),
        }
    }

    /// Finds the stored value that equals the given value, if it exists.
    /// Time complexity: O(log n).
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match Q::cmp(value, self.key().borrow()) {
            Ordering::Equal => Some(self.key()),
            Ordering::Greater => self.get_right().and_then(|right| right.get(value)),
            Ordering::Less => self.get_left().and_then(|left| left.get(value)),
        }
    }
}

/// Insertions
impl<T: Ord> RBNode<T> {
    /// Swaps the colors of self and its children if both children (exist and) are red.
    fn color_swap(&mut self) {
        if let Some(left) = self.get_left() && let Some(right) = self.get_right()
            && left.color() == Color::Red && right.color() == Color::Red
        {
            self.set_color(Color::Red);
            self.get_left_mut().unwrap().set_color(Color::Black);
            self.get_right_mut().unwrap().set_color(Color::Black);
        }
    }

    fn fix_local_violation(&mut self, side1: Side, side2: Side) -> bool {
        if let Some(child) = self.get_child(side1) && let Some(grandchild) = child.get_child(side2)
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

    /// Returns the subtree that the given value belongs to.
    /// Returns None if the value is equal to the root's key.
    fn choose_side<Q>(&self, value: &Q) -> Option<Side>
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match Q::cmp(value, self.key().borrow()) {
            Ordering::Less => Some(Side::Left),
            Ordering::Greater => Some(Side::Right),
            Ordering::Equal => None,
        }
    }

    /// Adds a value to the tree. If there was an equal value already in the tree, nothing happens.
    /// Returns a Boolean indicating if the value was inserted or not.
    /// Time complexity: O(log n).
    pub fn insert(&mut self, value: T) -> bool {
        // Traverse the tree, keeping track of three nodes: the current node, its parent, and its grandparent.
        // To not need multiple mutable references into self, keep track of only the grandparent, together with the sides to take to get to parent and current.
        // We first handle the cases where there is no parent or no current, i.e., the path to the leaf where we put value is too short.

        // Check for a color swap at every node we come accross.
        self.color_swap();

        let Some(mut side1) = self.choose_side(&value) else { return false; };
        if !self.has_child(side1) {
            // Insert the value in place of grandparent's child
            self.attach_child(side1, RBNode::new_with_color(value, Color::Red));
            self.color_swap(); // Might need to color swap due to the insertion.
            self.set_color(Color::Black); // Maintain the invariant that the root is black.
            return true;
        }

        // Walk down the tree, updating the tree as we go.
        let mut grandparent = &mut *self;
        loop {
            let Some(child) = grandparent.get_child_mut(side1) else {
                grandparent.attach_child(side1, RBNode::new_with_color(value, Color::Red));
                return true;
            };
            child.color_swap();

            let Some(side2) = child.choose_side(&value) else { return false; };
            let Some(grandchild) = child.get_child_mut(side2) else {
                child.attach_child(side2, RBNode::new_with_color(value, Color::Red));
                grandparent.fix_local_violation(side1, side2);
                break;
            };
            grandchild.color_swap();

            let has_changed = grandparent.fix_local_violation(side1, side2);
            if has_changed {
                // Need to do comparison again, grandparent has been changed, and side1 and side2 might have been changed with it.
                if let Some(side) = grandparent.choose_side(&value) {
                    side1 = side;
                } else { return false; }
            } else { 
                // Structure of the tree is unchanged, can safely continue the search in a subtree of grandparent.
                grandparent = grandparent.get_child_mut(side1).unwrap();
                side1 = side2;
            }
        }

        // Reset the root color to black
        self.set_color(Color::Black);
        true
    }
}

// Tree operations
impl<T> RBNode<T> {
    /// Takes the left subtree, leaving a leaf in its place.
    /// Returns None if there was no left subtree.
    fn detach_left(&mut self) -> Option<RBPointer<T>> {
        self.left.take()
    }

    /// Takes the right subtree, leaving a leaf in its place.
    /// Returns None if there was no right subtree.
    fn detach_right(&mut self) -> Option<RBPointer<T>> {
        self.right.take()
    }

    /// Replaces the left subtree of self with a given tree.
    /// Returns the original left subtree if it was present.
    fn replace_left(&mut self, tree: impl Into<Box<Self>>) -> Option<RBPointer<T>> {
        self.left.replace(tree.into())
    }

    /// Replaces the right subtree of self with a given tree.
    /// Returns the original right subtree if it was present.
    fn replace_right(&mut self, tree: impl Into<Box<Self>>) -> Option<RBPointer<T>> {
        self.right.replace(tree.into())
    }
    
    /// Attaches a tree in place of the left child of self.
    /// If a tree was already present, nothing is done.
    /// Returns a Boolean indicating if the tree was attached or not.
    fn attach_left(&mut self, tree: impl Into<Box<Self>>) -> bool {
        if self.get_left().is_none() {
            self.left = Some(tree.into());
            true
        } else { false }
    }

    /// Attaches a tree in place of the right child of self.
    /// If a tree was already present, nothing is done.
    /// Returns a Boolean indicating if the tree was attached or not.
    fn attach_right(&mut self, tree: impl Into<Box<Self>>) -> bool {
        if self.get_right().is_none() {
            self.right = Some(tree.into());
            true
        } else { false }
    }

    fn attach_child(&mut self, side: Side, tree: impl Into<Box<Self>>) -> bool {
        match side {
            Side::Left => self.attach_left(tree),
            Side::Right => self.attach_right(tree),
        }
    }

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
        let old_root = std::mem::replace(self, *new_root);
        self.replace_left(old_root);
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
        let old_root = std::mem::replace(self, *new_root);
        self.replace_right(old_root);
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

impl<T: fmt::Display> fmt::Display for RBNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recursive_fmt<T: fmt::Display>(root: Option<&RBNode<T>>, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result {
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
                recursive_fmt(root.get_left(), f, &new_prefix, true)?;
                recursive_fmt(root.get_right(), f, &new_prefix, false)?;
                Ok(())
            } else {
                write!(f, "L\n")
            }
        }

        recursive_fmt(Some(self), f, "", false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    fn assert_binary_search_tree<T: Clone + Ord>(root: &RBNode<T>) {
        fn assert_binary_search_tree_recursive<T: Clone + Ord>(root: Option<&RBNode<T>>) -> Option<(T, T)> {
            let Some(root) = root else { return None; };
            if let Some(max_left) = assert_binary_search_tree_recursive(root.get_left()).map(|(_, max)| max) {
                assert_eq!(T::cmp(root.key(), &max_left), Ordering::Greater);
            }
            if let Some(min_right) = assert_binary_search_tree_recursive(root.get_right()).map(|(min, _)| min) {
                assert_eq!(T::cmp(root.key(), &min_right), Ordering::Less);
            }
            Some((
                assert_binary_search_tree_recursive(root.get_left()).map_or(root.key().clone(), |(min, _)| min),
                assert_binary_search_tree_recursive(root.get_right()).map_or(root.key().clone(), |(_, max)| max)
            ))
        }
        assert_binary_search_tree_recursive(Some(root));
    }

    /// Asserts the given tree is a valid red-black tree
    fn assert_valid_tree<T: Clone + Ord>(root: &RBNode<T>) {
        // Asserts the given tree is a valid red-black tree, and returns the number of black nodes on any root-to-leaf path in the tree
        fn assert_valid_tree_recursive<T: Clone + Ord>(root: Option<&RBNode<T>>) -> usize {
            // Leaves are considered black
            let Some(root) = root else { return 1; };

            // Assert no consecutive red nodes
            if root.color() == Color::Red {
                assert_ne!(root.get_left().map(|left| left.color()), Some(Color::Red));
                assert_ne!(root.get_right().map(|right| right.color()), Some(Color::Red));
            }

            // Assert validity of subtrees
            let num_black_left = assert_valid_tree_recursive(root.get_left());
            let num_black_right = assert_valid_tree_recursive(root.get_right());

            // Assert black counts match
            assert_eq!(num_black_left, num_black_right);

            // Return number of black nodes on any root-to-leaf path
            if root.color() == Color::Red {
                num_black_left
            } else {
                1 + num_black_left
            }
        }

        assert_eq!(root.color(), Color::Black);
        assert_binary_search_tree(root);
        assert_valid_tree_recursive(Some(root));
    }

    #[test]
    fn test_insertion() {
        // Test inserting values in order
        let mut tree = RBNode::new(0);
        for value in 1..=30 {
            tree.insert(value);
        }
        assert_valid_tree(&tree);

        // Test inserting values in random order
        let mut rng = rand::rng();
        for _ in 0..5 {
            let mut tree = RBNode::new(0);
            let mut values = (1..=30).collect::<Vec<_>>();
            values.shuffle(&mut rng);
            for value in values {
                tree.insert(value);
            }
            assert_valid_tree(&tree);
        }
    }
}
