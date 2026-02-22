use std::{borrow::Borrow, cmp::Ordering};
use std::fmt;
use paste::paste;

use crate::binary_search_tree::tree_iterators::{
    inorder::{InorderIter, InorderIterMut},
    postorder::{PostorderIter, PostorderIterMut},
    preorder::{PreorderIter, PreorderIterMut},
};
use crate::binary_search_tree::tree_traits::{
    BinarySearchTree, BinaryTree, BinaryTreeMut
};

use super::Side;

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

pub struct RedBlackNode<K, V> {
    key: K,
    value: V,
    color: Color,
}

impl<K, V> RedBlackNode<K, V> {
    fn new(key: K, value: V, color: Color) -> Self {
        Self {
            key,
            value,
            color,
        }
    }

    fn into_data(self) -> (K, V) {
        (self.key, self.value)
    }
}

pub struct RBNodeRef<'tree, K, V> {
    pub key: &'tree K,
    pub value: &'tree V,
    pub left: &'tree RedBlackTree<K, V>,
    pub right: &'tree RedBlackTree<K, V>,
}

pub(crate) struct RBNodeRefMut<'tree, K, V> {
    pub(crate) key: &'tree K,
    pub(crate) value: &'tree mut V,
    pub(crate) left: &'tree RedBlackTree<K, V>,
    pub(crate) right: &'tree RedBlackTree<K, V>,
}

pub enum RedBlackTree<K, V> {
    Internal {
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
        Self::Internal {
            node: value,
            left: Default::default(),
            right: Default::default(),
            accessed_mut: false,
        }
    }
}

impl<K, V> FromIterator<(K, V)> for RedBlackTree<K, V>
where 
    K: Ord,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut tree = Self::default();
        for (key, value) in iter {
            tree.insert(key, value);
        }
        tree
    }
}

macro_rules! make_iter {
    ($vis: vis, $iter_name: ident, $iter_type: ident) => {
        paste!{
            $vis fn $iter_name(&mut self) -> $iter_type<'_, Self, impl Fn(&Self) -> bool> {
                self.[<$iter_name _filtered>](|_| true)
            }

            $vis fn [<$iter_name _filtered>]<F>(&'_ mut self, f: F) -> $iter_type<'_, Self, F>
            where
                F: Fn(&Self) -> bool,
            {
                $iter_type::new(self, f)
            }
        }
    };
}

impl<K, V> RedBlackTree<K, V> {
    make_iter!(pub, inorder_iter, InorderIter);
    make_iter!(pub(crate), inorder_iter_mut, InorderIterMut);
    make_iter!(pub, preorder_iter, PreorderIter);
    make_iter!(pub(crate), preorder_iter_mut, PreorderIterMut);
    make_iter!(pub, postorder_iter, PostorderIter);
    make_iter!(pub(crate), postorder_iter_mut, PostorderIterMut);
}

impl<K, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    fn node(&self) -> Option<&RedBlackNode<K, V>> {
        match self {
            Self::Internal { node, .. } => Some(node),
            Self::Leaf => None,
        }
    }

    fn node_mut(&mut self) -> Option<&mut RedBlackNode<K, V>> {
        match self {
            Self::Internal { node, accessed_mut, .. } => {
                *accessed_mut = true;
                Some(node)
            },
            Self::Leaf => None,
        }
    }

    fn into_node(self) -> Option<RedBlackNode<K, V>> {
        match self {
            Self::Internal { node, .. } => Some(node),
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
        if let Self::Internal { node, left, right, .. } = self {
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
            Self::Internal {..} => false,
            Self::Leaf => true,
        }
    }

    fn left_subtree(&self) -> Option<&Self> {
        match self {
            Self::Internal { left, .. } => Some(left),
            Self::Leaf => None,
        }
    }

    fn right_subtree(&self) -> Option<&Self> {
        match self {
            Self::Internal { right, .. } => Some(right),
            Self::Leaf => None,
        }
    }
}

impl<K, V> BinaryTreeMut for RedBlackTree<K, V> {
    type NodeRefMut<'tree> = RBNodeRefMut<'tree, K, V>
    where Self: 'tree;

    fn node_ref_mut(&'_ mut self) -> Option<Self::NodeRefMut<'_>> {
        if let Self::Internal { node, left, right, .. } = self {
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
            Self::Internal { left, accessed_mut, .. } => {
                *accessed_mut = true;
                Some(left)
            },
            Self::Leaf => None,
        }
    }

    fn right_subtree_mut(&mut self) -> Option<&mut Self> {
        match self {
            Self::Internal { right, accessed_mut, .. } => {
                *accessed_mut = true;
                Some(right)
            },
            Self::Leaf => None,
        }
    }

    fn subtrees_mut(&mut self) -> Option<(&mut Self, &mut Self)> {
        match self {
            Self::Internal { left, right, .. } => Some((left, right)),
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
    fn rotate_left(&mut self) -> bool {
        if !<Self as BinaryTreeMut>::rotate_left(self) { return false; }
        self.set_root_color(Color::Black);
        self.left_subtree_mut().unwrap().set_root_color(Color::Red); // Can unwrap safely: left subtree exists since the rotation was successful.
        true
    }

    fn rotate_right(&mut self) -> bool {
        if !<Self as BinaryTreeMut>::rotate_right(self) { return false; }
        self.set_root_color(Color::Black);
        self.right_subtree_mut().unwrap().set_root_color(Color::Red); // Can unwrap safely: left subtree exists since the rotation was successful.
        true
    }

    fn double_rotate_left(&mut self) -> bool {
        let Some(mut right) = self.detach_right() else { return false; };
        if !right.rotate_right() { return false; }
        self.replace_right(right);
        self.rotate_left();
        true
    }

    fn double_rotate_right(&mut self) -> bool {
        let Some(mut left) = self.detach_left() else { return false; };
        if !left.rotate_left() { return false; }
        self.replace_left(left);
        self.rotate_right();
        true
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
                (Side::Left, Side::Left) => self.rotate_right(),
                (Side::Right, Side::Right) => self.rotate_left(),
                (Side::Left, Side::Right) => self.double_rotate_right(),
                (Side::Right, Side::Left) => self.double_rotate_left(),
            }
        } else { false }
    }

    /// Inserts the key-value pair into the tree.
    /// If the key was not present in the tree yet, None is returned.
    /// Otherwise, the value stored at the given key is updated, and the old value is returned.
    /// Time complexity: O(log n).
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // Traverse the tree, keeping track of three nodes: the current node, its parent, and its grandparent.
        // To not need multiple mutable references into self, keep track of only the grandparent, together with the sides to take to get to parent and current.
        // We first handle the cases where there is no parent or no current, i.e., the path to the leaf where we put value is too short.
        
        let mut side1 = if let Some(root_key) = self.key() {
            match K::cmp(&key, root_key) {
                Ordering::Less => Side::Left,
                Ordering::Greater => Side::Right,
                Ordering::Equal => return Some(std::mem::replace(self.value_mut().unwrap(), value)),
            }
        } else {
            // Tree is empty.
            *self = Self::from(RedBlackNode::new(key, value, Color::Black));
            return None;
        };

        // Check for a color swap at every node we come accross.
        self.color_swap();

        // Walk down the tree, updating the tree as we go.
        let mut current = &mut *self;
        loop {
            let child = current.subtree_mut(side1).unwrap(); // Can unwrap safely, we ensure that current is not a leaf.
            child.color_swap();

            let side2 = if let Some(child_key) = child.key() {
                match K::cmp(&key, child_key) {
                    Ordering::Less => Side::Left,
                    Ordering::Greater => Side::Right,
                    Ordering::Equal => return Some(std::mem::replace(child.value_mut().unwrap(), value)),
                }
            } else {
                // Child tree is empty.
                *child = Self::from(RedBlackNode::new(key, value, Color::Red));
                break;
            };

            let grandchild = child.subtree_mut(side2).unwrap(); // Can unwrap safely, we ensure that child is not a leaf.
            if grandchild.is_leaf() {
                *grandchild = Self::from(RedBlackNode::new(key, value, Color::Red));
                current.fix_local_violation(side1, side2);
                break;
            }
            grandchild.color_swap();

            if current.fix_local_violation(side1, side2) {
                // Need to do comparison again, since tree structure has been changed.
                // In particular, grandparent has been changed, and side1 and side2 might have been changed with it.
                side1 = match K::cmp(&key, current.key().unwrap()) {
                    Ordering::Less => Side::Left,
                    Ordering::Greater => Side::Right,
                    Ordering::Equal => return Some(std::mem::replace(current.value_mut().unwrap(), value)),
                };
            } else { 
                // Structure of the tree is unchanged, can safely continue the search in a subtree of grandparent.
                current = current.subtree_mut(side1).unwrap();
                side1 = side2;
            }
        }

        // Reset the root color to black.
        self.set_root_color(Color::Black);
        None
    }
}

/// Deletions.
impl<K, V> RedBlackTree<K, V>
where 
    K: Ord,
{
    fn left_color(&self) -> Option<Color> {
        self.left_subtree()?.root_color()
    }

    fn right_color(&self) -> Option<Color> {
        self.right_subtree()?.root_color()
    }

    fn subtree_color(&self, side: Side) -> Option<Color> {
        match side {
            Side::Left => self.left_color(),
            Side::Right => self.right_color(),
        }
    }

    fn flip_root_color(&mut self) {
        if let Some(root_color) = self.root_color() {
            self.set_root_color(root_color.opposite());
        }
    }

    fn color_flip(&mut self) {
        self.flip_root_color();
        if let Self::Internal { left, right, .. } = self {
            left.flip_root_color();
            right.flip_root_color();
        }
    }

    fn rotate(&mut self, side: Side) -> bool {
        let Some(col_root) = self.root_color() else { return false; };
        if <Self as BinaryTreeMut>::rotate(self, side.opposite()) { // TODO: Change rotation code to be about rotating edges, so this opposite side is not needed.
            self.set_root_color(col_root);
            self.subtree_mut(side.opposite()).unwrap().set_root_color(Color::Red);
            true
        } else { false }
    }

    fn swap_root_with_predecessor(&mut self) {
        let Some(mut left) = self.detach_left() else { return; };

        let mut current = &mut left;
        while let Some(child) = current.right_subtree()
            && !child.is_leaf()
        {
            current = current.right_subtree_mut().unwrap();
        }

        std::mem::swap(self.node_mut().unwrap(), current.node_mut().unwrap());
        std::mem::swap(&mut self.node_mut().unwrap().color, &mut current.node_mut().unwrap().color);
        self.attach_left(left);
    }

    /// Helper function for removing roots with at most one subtree attached.
    /// Returns None if the root has both subtrees attached.
    /// Colors the new root black.
    fn remove_root_single_child(&mut self) -> Option<(K, V)> {
        let Self::Internal { left, right, .. } = self else { return None; };
        if left.is_leaf() {
            let mut right = self.detach_right().unwrap();
            right.set_root_color(Color::Black);
            let old = std::mem::replace(self, right);
            old.into_node().map(RedBlackNode::into_data)
        } else if right.is_leaf() {
            let mut left = self.detach_left().unwrap();
            left.set_root_color(Color::Black);
            let old = std::mem::replace(self, left);
            old.into_node().map(RedBlackNode::into_data)
        } else {
            None
        }
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where 
        K: Borrow<Q>,
        Q: Ord,
    {
        // Top-down deletion algorithm is for deleting nodes without subtrees.
        // To handle deleting any node, we can swap the to-be-deleted node with its predecessor.
        // We have to wait with swapping until we move out of the to-be-deleted node, since after the swap, its subtree is invalid.
        // By advancing down one level, the subtree becomes valid again.

        let mut current = &mut *self;
        while let Self::Internal { node, .. } = current {
            let (side, found) = match Q::cmp(key, node.key.borrow()) {
                Ordering::Less => (Side::Left, false),
                Ordering::Greater => (Side::Right, false),
                Ordering::Equal => {
                    // If the node has at most one subtree, we can easily remove it.
                    // The remove_root_single_child function handles the no-child case as well, as by this point,
                    // the tree has been restructured sufficiently to be valid after deleting the node.
                    if let Some(data) = current.remove_root_single_child() {
                        self.set_root_color(Color::Black);
                        return Some(data);
                    } else {
                        (Side::Left, true)
                    }
                }
            };

            // First phase.
            if current.subtree_color(side) == Some(Color::Red) {
                // We advance without changing the location of the to-be-deleted current node.
                // The subtree we advance to contains the predecessor of current, and not current itself.
                // Thus, swapping current with its predecessor keeps the subtree valid.
                if found {
                    current.swap_root_with_predecessor();
                }

                current = current.subtree_mut(side)?;
                continue;
            }

            // Second phase.
            if current.subtree_color(side.opposite()) == Some(Color::Red) {
                // Current stays in the same node, since the rotation is of the opposite edge.
                current.rotate(side.opposite());
                current = current.subtree_mut(side)?;
            }

            // Third phase.
            let child = current.subtree_mut(side)?;
            if child.left_color() == Some(Color::Red) || child.right_color() == Some(Color::Red)
            {
                // We advance without changing the location of the to-be-deleted current node.
                // The subtree we advance to contains the predecessor of current, and not current itself.
                // Thus, swapping current with its predecessor keeps the subtree valid.
                if found {
                    current.swap_root_with_predecessor();
                }

                current = current.subtree_mut(side)?;
                continue;
            }

            current.color_flip();
            let sibling_child = current.subtree_mut(side.opposite())?;
            if sibling_child.left_color() == Some(Color::Red) || sibling_child.right_color() == Some(Color::Red) {
                if sibling_child.subtree_color(side.opposite()) != Some(Color::Red) {
                    sibling_child.rotate(side);
                }

                // Current stays in the same node, since the rotation is of the opposite edge.
                current.rotate(side.opposite());
                current.color_flip();
                current = current.subtree_mut(side)?;
            }
        }

        self.set_root_color(Color::Black);
        None
    }
}

impl<K, V> fmt::Debug for RedBlackTree<K, V>
where 
    K: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recursive_fmt<K, V>(tree: &RedBlackTree<K, V>, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result
        where
            K: fmt::Debug,
        {
            write!(f, "{prefix}")?;
            if is_left {
                write!(f, "├──")?;
            } else {
                write!(f, "└──")?;
            };
            if let RedBlackTree::Internal { node, .. } = tree {
                let c = match node.color {
                    Color::Red => "r",
                    Color::Black => "b",
                };
                write!(f, "N({:?}, {c})\n", node.key)?;
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
        
        write!(f, "\n")?;
        recursive_fmt(self, f, "", false)
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
            if let RedBlackTree::Internal { node, .. } = tree {
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
            let RedBlackTree::Internal { node, left, right, .. } = tree else { return None; };
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
            let RedBlackTree::Internal { node, left, right, .. } = tree else { return 1; };

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

        if let RedBlackTree::Internal { node, .. } = tree {
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
        for _ in 0..50 {
            let mut tree = RedBlackTree::new();
            let mut keys = (1..=30).collect::<Vec<_>>();
            keys.shuffle(&mut rng);
            for key in keys {
                tree.insert(key, ());
            }
            assert_valid_tree(&tree);
        }

        // Test inserting and updating data.
        for _ in 0..50 {
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

    #[test]
    fn test_deletion() {
        // Test deleting values in random order.
        let mut rng = rand::rng();
        for _ in 0..50 {
            let mut keys = (1..=30).collect::<Vec<_>>();
            keys.shuffle(&mut rng);
            let data = keys.clone().into_iter()
                .map(|i| (i, i % 10));
            let mut tree = data.clone().collect::<RedBlackTree<_, _>>();
            let mut map = data.collect::<HashMap<_, _>>();

            keys.shuffle(&mut rng);
            for key in keys {
                let entry_tree = tree.remove_entry(&key);
                let entry_map = map.remove_entry(&key);
                assert_eq!(entry_tree, entry_map);
                assert_valid_tree(&tree);
            }
        }
    }
}
