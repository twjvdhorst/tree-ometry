use std::cmp::Ordering;
use std::fmt;

use crate::binary_search_tree::tree_traits::{
    BinarySearchTree, BinaryTree, BinaryTreeMut
};

use super::{
    Side,
    tree_errors::StructureError,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

pub struct RedBlackNode<K, V> {
    key: K,
    value: V,
    color: Color,
}

impl<K, V> RedBlackNode<K, V> {
    fn new(key: K, value: V) -> Self {
        Self::new_with_color(key, value, Color::Black)
    }

    fn new_with_color(key: K, value: V, color: Color) -> Self {
        Self {
            key,
            value,
            color,
        }
    }
}

pub struct RBNodeRef<'tree, K, V> {
    key: &'tree K,
    value: &'tree V,
    left: &'tree RedBlackTree<K, V>,
    right: &'tree RedBlackTree<K, V>,
}

pub struct RBNodeRefMut<'tree, K, V> {
    key: &'tree K,
    value: &'tree mut V,
    left: &'tree RedBlackTree<K, V>,
    right: &'tree RedBlackTree<K, V>,
}

pub enum RedBlackTree<K, V> {
    Node {
        node: RedBlackNode<K, V>,
        left: Box<Self>,
        right: Box<Self>,
        accessed_mut: bool,
    },
    Leaf,
}

impl<K, V> Default for RedBlackTree<K, V> {
    fn default() -> Self {
        Self::Leaf
    }
}

impl<K, V> From<RedBlackNode<K, V>> for RedBlackTree<K, V> {
    fn from(value: RedBlackNode<K, V>) -> Self {
        Self::Node {
            node: value,
            left: Default::default(),
            right: Default::default(),
            accessed_mut: false,
        }
    }
}

impl<K, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        Self::Leaf
    }

    fn node(&self) -> Option<&RedBlackNode<K, V>> {
        match self {
            Self::Node { node, .. } => Some(node),
            Self::Leaf => None,
        }
    }

    fn node_mut(&mut self) -> Option<&mut RedBlackNode<K, V>> {
        match self {
            Self::Node { node, accessed_mut, .. } => {
                *accessed_mut = true;
                Some(node)
            },
            Self::Leaf => None,
        }
    }

    fn value_mut(&mut self) -> Option<&mut V> {
        Some(&mut self.node_mut()?.value)
    }

    fn root_color(&self) -> Option<Color> {
        Some(self.node()?.color)
    }

    fn set_root_color(&mut self, color: Color) {
        if let Some(node) = self.node_mut() {
            node.color = color
        }
    }
}

impl<K, V> BinaryTree for RedBlackTree<K, V> {
    type NodeRef<'tree> = RBNodeRef<'tree, K, V>
    where Self: 'tree;

    fn node_ref(&'_ self) -> Option<Self::NodeRef<'_>> {
        if let Self::Node { node, left, right, .. } = self {
            Some(Self::NodeRef {
                key: &node.key,
                value: &node.value,
                left,
                right,
            })
        } else { None }
    }

    fn is_leaf(&self) -> bool {
        match self {
            Self::Node {..} => false,
            Self::Leaf => true,
        }
    }

    fn left_subtree(&self) -> Option<&Self> {
        match self {
            Self::Node { left, .. } => Some(left),
            Self::Leaf => None,
        }
    }

    fn right_subtree(&self) -> Option<&Self> {
        match self {
            Self::Node { right, .. } => Some(right),
            Self::Leaf => None,
        }
    }
}

impl<K, V> BinaryTreeMut for RedBlackTree<K, V> {
    type NodeRefMut<'tree> = RBNodeRefMut<'tree, K, V>
    where Self: 'tree;

    fn node_ref_mut(&'_ mut self) -> Option<Self::NodeRefMut<'_>> {
        if let Self::Node { node, left, right, .. } = self {
            Some(Self::NodeRefMut {
                key: &node.key,
                value: &mut node.value,
                left,
                right,
            })
        } else { None }
    }

    fn left_subtree_mut(&mut self) -> Option<&mut Self> {
        match self {
            Self::Node { left, accessed_mut, .. } => {
                *accessed_mut = true;
                Some(left)
            },
            Self::Leaf => None,
        }
    }

    fn right_subtree_mut(&mut self) -> Option<&mut Self> {
        match self {
            Self::Node { right, accessed_mut, .. } => {
                *accessed_mut = true;
                Some(right)
            },
            Self::Leaf => None,
        }
    }

    fn attach_left(&mut self, tree: impl Into<Self>) -> bool {
        if let Some(left) = self.left_subtree_mut() && left.is_leaf() {
            *left = tree.into();
            true
        } else { false }
    }

    fn attach_right(&mut self, tree: impl Into<Self>) -> bool {
        if let Some(right) = self.right_subtree_mut() && right.is_leaf() {
            *right = tree.into();
            true
        } else { false }
    }

    fn detach_left(&mut self) -> Option<Self> {
        Some(std::mem::take(self.left_subtree_mut()?))
    }

    fn detach_right(&mut self) -> Option<Self> {
        Some(std::mem::take(self.right_subtree_mut()?))
    }
    
    fn replace_left(&mut self, tree: impl Into<Self>) -> Option<Self> {
        Some(std::mem::replace(self.left_subtree_mut()?, tree.into()))
    }
    
    fn replace_right(&mut self, tree: impl Into<Self>) -> Option<Self> {
        Some(std::mem::replace(self.right_subtree_mut()?, tree.into()))
    }
}

impl<K, V> BinarySearchTree for RedBlackTree<K, V>
where
    K: Ord,
{
    type Key = K;
    type Value = V;

    fn key(&self) -> Option<&Self::Key> {
        Some(&self.node()?.key)
    }

    fn value(&self) -> Option<&Self::Value> {
        Some(&self.node()?.value)
    }
}

/// Insertions.
impl<K, V> RedBlackTree<K, V>
where 
    K: Ord,
{
    fn rotate_left(&mut self) -> Result<(), StructureError> {
        <Self as BinaryTreeMut>::rotate_left(self)?;
        self.set_root_color(Color::Black);
        self.left_subtree_mut().unwrap().set_root_color(Color::Red); // Can unwrap safely: left subtree exists since the rotation was successful.
        Ok(())
    }

    fn rotate_right(&mut self) -> Result<(), StructureError> {
        <Self as BinaryTreeMut>::rotate_right(self)?;
        self.set_root_color(Color::Black);
        self.right_subtree_mut().unwrap().set_root_color(Color::Red); // Can unwrap safely: left subtree exists since the rotation was successful.
        Ok(())
    }

    fn double_rotate_left(&mut self) -> Result<(), StructureError> {
        let mut right = self.detach_right().ok_or(StructureError::EmptyTree)?;
        right.rotate_right()?;
        self.replace_right(right);
        self.rotate_left()
    }

    fn double_rotate_right(&mut self) -> Result<(), StructureError> {
        let mut left = self.detach_left().ok_or(StructureError::EmptyTree)?;
        left.rotate_left()?;
        self.replace_left(left);
        self.rotate_right()
    }

    fn pick_branch(&self, value: &K) -> Option<Side> {
        match K::cmp(value, self.key()?) {
            Ordering::Less => Some(Side::Left),
            Ordering::Greater => Some(Side::Right),
            Ordering::Equal => None,
        }
    }

    /// Swaps the colors of self and its children if both children (exist and) are red.
    fn color_swap(&mut self) {
        if let Some(left) = self.left_subtree() && let Some(right) = self.right_subtree()
            && left.root_color() == Some(Color::Red) && right.root_color() == Some(Color::Red)
        {
            self.set_root_color(Color::Red);
            self.left_subtree_mut().unwrap().set_root_color(Color::Black);
            self.right_subtree_mut().unwrap().set_root_color(Color::Black);
        }
    }

    fn fix_local_violation(&mut self, side1: Side, side2: Side) -> bool {
        if let Some(child) = self.subtree(side1) && let Some(grandchild) = child.subtree(side2)
            && child.root_color() == Some(Color::Red) && grandchild.root_color() == Some(Color::Red)
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
        if self.is_leaf() {
            *self = Self::from(RedBlackNode::new(key, value));
            return None;
        }

        // Check for a color swap at every node we come accross.
        self.color_swap();

        let Some(mut side1) = self.pick_branch(&key) else { 
            // self has the same key as the given key
            return Some(std::mem::replace(self.value_mut()?, value));
        };
        if !self.has_subtree(side1) {
            // Insert the key-value pair as a child of self.
            self.attach_subtree(side1, Self::from(RedBlackNode::new_with_color(key, value, Color::Red)));
            self.color_swap(); // Might need to color swap due to the insertion.
            self.set_root_color(Color::Black); // Maintain the invariant that the root is black.
            return None;

        }

        // Walk down the tree, updating the tree as we go.
        let mut grandparent = &mut *self;
        loop {
            if !grandparent.has_subtree(side1) {
                grandparent.attach_subtree(side1, Self::from(RedBlackNode::new_with_color(key, value, Color::Red)));
                return None;
            }
            let child = grandparent.subtree_mut(side1).unwrap();
            child.color_swap();

            let Some(side2) = child.pick_branch(&key) else {
                // child has the same key as the given key
                return Some(std::mem::replace(child.value_mut()?, value));
            };
            if !child.has_subtree(side2) {
                child.attach_subtree(side2, Self::from(RedBlackNode::new_with_color(key, value, Color::Red)));
                grandparent.fix_local_violation(side1, side2);
                break;
            }
            let grandchild = child.subtree_mut(side2).unwrap();
            grandchild.color_swap();

            let has_changed = grandparent.fix_local_violation(side1, side2);
            if has_changed {
                // Need to do comparison again, grandparent has been changed, and side1 and side2 might have been changed with it.
                if let Some(side) = grandparent.pick_branch(&key) {
                    side1 = side;
                } else {
                    // grandparent has the same key as the given key
                    return Some(std::mem::replace(grandparent.value_mut()?, value));
                }
            } else { 
                // Structure of the tree is unchanged, can safely continue the search in a subtree of grandparent.
                grandparent = grandparent.subtree_mut(side1).unwrap();
                side1 = side2;
            }
        }

        // Reset the root color to black
        self.set_root_color(Color::Black);
        None
    }
}
    
impl<K, V> fmt::Display for RedBlackTree<K, V>
where 
    K: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recursive_fmt<K, V>(tree: &RedBlackTree<K, V>, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result
        where
            K: fmt::Display,
        {
            write!(f, "{prefix}")?;
            if is_left {
                write!(f, "├──")?;
            } else {
                write!(f, "└──")?;
            };
            if let RedBlackTree::Node { node, .. } = tree {
                write!(f, "N({})\n", node.key)?;
                let new_prefix = String::from(prefix) + if is_left { "│  " } else { "   " };
                if let Some(left) = tree.left_subtree() {
                    recursive_fmt(left, f, &new_prefix, true)?;
                }
                if let Some(right) = tree.right_subtree() {
                    recursive_fmt(right, f, &new_prefix, false)?;
                }
                Ok(())
            } else {
                write!(f, "L\n")
            }
        }

        recursive_fmt(self, f, "", false)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use rand::prelude::*;

    use super::*;

    fn assert_binary_search_tree<K, V>(root: &RedBlackTree<K, V>)
    where 
        K: Clone + Ord,
    {
        fn assert_binary_search_tree_recursive<K, V>(tree: &RedBlackTree<K, V>) -> Option<(K, K)>
        where
            K: Clone + Ord,
        {
            let RedBlackTree::Node { node, left, right, .. } = tree else { return None; };
            if let Some(max_left) = assert_binary_search_tree_recursive(left).map(|(_, max)| max) {
                assert_eq!(K::cmp(&node.key, &max_left), Ordering::Greater);
            }
            if let Some(min_right) = assert_binary_search_tree_recursive(right).map(|(min, _)| min) {
                assert_eq!(K::cmp(&node.key, &min_right), Ordering::Less);
            }
            Some((
                assert_binary_search_tree_recursive(left).map_or(node.key.clone(), |(min, _)| min),
                assert_binary_search_tree_recursive(right).map_or(node.key.clone(), |(_, max)| max)
            ))
        }
        assert_binary_search_tree_recursive(root);
    }

    /// Asserts the given tree is a valid red-black tree.
    fn assert_valid_tree<K, V>(tree: &RedBlackTree<K, V>)
    where 
        K: Clone + Ord,
    {
        // Asserts the given tree is a valid red-black tree, and returns the number of black nodes on any root-to-leaf path in the tree.
        fn assert_valid_tree_recursive<K, V>(tree: &RedBlackTree<K, V>) -> usize
        where
            K: Clone + Ord,
        {
            // Leaves are considered black.
            let RedBlackTree::Node { node, left, right, .. } = tree else { return 1; };

            // Assert no consecutive red nodes.
            if node.color == Color::Red {
                assert_ne!(left.root_color(), Some(Color::Red));
                assert_ne!(right.root_color(), Some(Color::Red));
            }

            // Assert validity of subtrees.
            let num_black_left = assert_valid_tree_recursive(left);
            let num_black_right = assert_valid_tree_recursive(right);

            // Assert black counts match.
            assert_eq!(num_black_left, num_black_right);

            // Return number of black nodes on any root-to-leaf path.
            if node.color == Color::Red {
                num_black_left
            } else {
                1 + num_black_left
            }
        }

        if let RedBlackTree::Node { node, .. } = tree {
            assert_eq!(node.color, Color::Black);
        }
        assert_binary_search_tree(tree);
        assert_valid_tree_recursive(tree);
    }

    #[test]
    fn test_insertion() {
        // Test inserting values in order.
        let mut tree = RedBlackTree::new();
        for key in 1..=30 {
            tree.insert(key, ());
        }
        assert_valid_tree(&tree);

        // Test inserting values in random order.
        let mut rng = rand::rng();
        for _ in 0..5 {
            let mut tree = RedBlackTree::new();
            let mut keys = (1..=30).collect::<Vec<_>>();
            keys.shuffle(&mut rng);
            for key in keys {
                tree.insert(key, ());
            }
            assert_valid_tree(&tree);
        }

        // Test inserting and updating data.
        for _ in 0..5 {
            let keys = (1..=5).cycle();
            let mut values = (1..=30).collect::<Vec<_>>();
            values.shuffle(&mut rng);

            let mut tree = RedBlackTree::new();
            let mut key_data_map = HashMap::new();
            for (key, value) in Iterator::zip(keys, values) {
                let old_value_tree = tree.insert(key.clone(), value.clone());
                let old_value_map = key_data_map.insert(key.clone(), value.clone());
                assert_eq!(old_value_tree, old_value_map);
            }
        }
    }
}
