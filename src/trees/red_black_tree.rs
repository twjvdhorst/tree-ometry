use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt,
};

use crate::trees::binary_tree_node::{
    BinaryTreeNode,
    Side,
};
use crate::trees::tree_errors::StructureError;

pub struct RedBlackTree<N> {
    node: N,
    color: Color,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

/// Construction methods
impl<N> RedBlackTree<N>
where 
    N: BinaryTreeNode,
{
    pub fn new(key: N::Key, value: N::Value) -> Self {
        Self::new_with_color(key, value, Color::Black)
    }

    fn new_with_color(key: N::Key, value: N::Value, color: Color) -> Self {
        Self {
            color,
            node: N::new(key, value),
        }
    }
}

/// Tree access
impl<N> RedBlackTree<N>
where 
    N: BinaryTreeNode<Tree = Self>,
{
    pub fn tree(&self) -> &N {
        &self.node
    }

    fn tree_mut(&mut self) -> &mut N {
        &mut self.node
    }

    pub fn color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn key(&self) -> &N::Key {
        &self.node.key()
    }

    pub fn get_child(&self, side: Side) -> Option<&Self> {
        match side {
            Side::Left => self.get_left(),
            Side::Right => self.get_right(),
        }
    }

    pub fn get_left(&self) -> Option<&Self> {
        self.tree().get_left()
    }

    pub fn get_right(&self) -> Option<&Self> {
        self.tree().get_right()
    }

    fn get_child_mut(&mut self, side: Side) -> Option<&mut Self> {
        match side {
            Side::Left => self.get_left_mut(),
            Side::Right => self.get_right_mut(),
        }
    }

    fn get_left_mut(&mut self) -> Option<&mut Self> {
        self.tree_mut().get_left_mut()
    }

    fn get_right_mut(&mut self) -> Option<&mut Self> {
        self.tree_mut().get_right_mut()
    }
}

/// Queries
impl<N> RedBlackTree<N>
where 
    N: BinaryTreeNode<Tree = Self>
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
            Ordering::Less => self.get_left()
                .and_then(|left| left.predecessor(value)),
            Ordering::Greater => self.get_right()
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
            Ordering::Greater => self.get_right()
                .and_then(|right| right.successor(value)),
            Ordering::Less => self.get_left()
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
            Ordering::Greater => self.get_right().and_then(|right| right.get(value)),
            Ordering::Less => self.get_left().and_then(|left| left.get(value)),
        }
    }
}

impl<N> RedBlackTree<N>
where 
    N: BinaryTreeNode<Tree = Self>,
{
    /// Performs a left tree rotation, changing self to point to the new root.
    /// The function returns an error if the tree has an incorrect shape (i.e., is a leaf or has no right subtree).
    fn rotate_left(&mut self) -> Result<(), StructureError> {
        let mut new_root = self.tree_mut().detach_right().ok_or(StructureError::EmptyTree)?;

        // Change colors to keep red-black properties satisfied after rotation
        self.set_color(Color::Red);
        new_root.set_color(Color::Black);

        // Perform the rotation
        if let Some(rotating_subtree) = new_root.tree_mut().detach_left() {
            // There is a non-empty rotating subtree
            self.tree_mut().replace_right(rotating_subtree);
        }
        std::mem::swap(self, &mut new_root);
        self.tree_mut().replace_left(new_root);
        Ok(())
    }

    /// Performs a right tree rotation, changing self to point to the new root.
    /// The function returns an error if the tree has an incorrect shape (i.e., is a leaf or has no right subtree).
    fn rotate_right(&mut self) -> Result<(), StructureError> {
        let mut new_root = self.tree_mut().detach_left().ok_or(StructureError::EmptyTree)?;

        // Change colors to keep red-black properties satisfied after rotation
        self.set_color(Color::Red);
        new_root.set_color(Color::Black);

        // Perform the rotation
        if let Some(rotating_subtree) = new_root.tree_mut().detach_right() {
            // There is a non-empty rotating subtree
            self.tree_mut().replace_left(rotating_subtree);
        }
        std::mem::swap(self, &mut new_root);
        self.tree_mut().replace_right(new_root);
        Ok(())
    }

    fn double_rotate_left(&mut self) -> Result<(), StructureError> {
        let mut new_right = self.tree_mut().detach_right().ok_or(StructureError::EmptyTree)?;
        new_right.rotate_right()?;
        self.tree_mut().replace_right(new_right);
        self.rotate_left()
    }

    fn double_rotate_right(&mut self) -> Result<(), StructureError> {
        let mut new_left = self.tree_mut().detach_left().ok_or(StructureError::EmptyTree)?;
        new_left.rotate_left()?;
        self.tree_mut().replace_left(new_left);
        self.rotate_right()
    }
}

/// Insertions
impl<N> RedBlackTree<N>
where 
    N: BinaryTreeNode<Tree = Self>,
{
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
            let old_value = self.tree_mut().replace_value(value);
            return Some(old_value);
        };
        if !self.tree().has_child(side1) {
            // Insert the value in place of grandparent's child
            self.tree_mut().attach_child(side1, RedBlackTree::new_with_color(key, value, Color::Red));
            self.color_swap(); // Might need to color swap due to the insertion.
            self.set_color(Color::Black); // Maintain the invariant that the root is black.
            return None;
        }

        // Walk down the tree, updating the tree as we go.
        let mut grandparent = &mut *self;
        loop {
            let Some(child) = grandparent.get_child_mut(side1) else {
                grandparent.tree_mut().attach_child(side1, RedBlackTree::new_with_color(key, value, Color::Red));
                return None;
            };
            child.color_swap();

            let Some(side2) = child.pick_branch(&key) else {
                // child has the same key as the given key
                let old_value = child.tree_mut().replace_value(value);
                return Some(old_value);
            };
            let Some(grandchild) = child.get_child_mut(side2) else {
                child.tree_mut().attach_child(side2, RedBlackTree::new_with_color(key, value, Color::Red));
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
                    let old_value = grandparent.tree_mut().replace_value(value);
                    return Some(old_value);
                }
            } else { 
                // Structure of the tree is unchanged, can safely continue the search in a subtree of grandparent.
                grandparent = grandparent.get_child_mut(side1).unwrap();
                side1 = side2;
            }
        }

        // Reset the root color to black
        self.set_color(Color::Black);
        None
    }
}
    
impl<N> fmt::Display for RedBlackTree<N>
where 
    N: BinaryTreeNode<Tree = Self>,
    N::Key: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recursive_fmt<T>(root: Option<&RedBlackTree<T>>, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result
        where
            T: BinaryTreeNode<Tree = RedBlackTree<T>>,
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
