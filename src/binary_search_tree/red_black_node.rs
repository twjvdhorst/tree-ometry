use std::{borrow::Borrow, cmp::Ordering};
use std::fmt;
use paste::paste;

use crate::binary_search_tree::tree_traits::{
    BinaryTree, BinaryTreeNode, BinaryTreeNodeMut
};

use super::Side;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
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

pub struct RedBlackNode<K, V, T>  {
    key: K,
    value: V,
    left: Box<T>,
    right: Box<T>,
    color: Color,
}

impl<K, V, T> fmt::Debug for RedBlackNode<K, V, T>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self.color {
            Color::Red => "r",
            Color::Black => "b",
        };
        write!(f, "({:?}, {:?}, {c})", &self.key, &self.value)
    }
}

impl<K, V, T> fmt::Display for RedBlackNode<K, V, T>
where
    K: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", &self.key, &self.value)
    }
}

impl<K, V, T> RedBlackNode<K, V, T>
where 
    T: BinaryTree,
{
    pub fn new(key: K, value: V, color: Color) -> Self {
        Self {
            key,
            value,
            left: Box::new(T::new_leaf()),
            right: Box::new(T::new_leaf()),
            color,
        }
    }
}

impl<K, V, T> RedBlackNode<K, V, T> {
    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub(crate) fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    pub fn data(&self) -> (&K, &V) {
        (&self.key, &self.value)
    }

    fn into_data(self) -> (K, V) {
        (self.key, self.value)
    }

    pub fn left(&self) -> &T {
        &self.left
    }

    pub fn right(&self) -> &T {
        &self.right
    }
}

impl<K, V, T> BinaryTreeNode for RedBlackNode<K, V, T> {
    type Tree = T;

    fn left_subtree(&self) -> &Self::Tree {
        self.left.as_ref()
    }

    fn right_subtree(&self) -> &Self::Tree {
        self.right.as_ref()
    }
}

impl<K, V, T> BinaryTreeNodeMut for RedBlackNode<K, V, T>
where 
    T: BinaryTree<Node = Self>,
{
    fn left_subtree_mut(&mut self) -> &mut Self::Tree {
        self.left.as_mut()
    }

    fn right_subtree_mut(&mut self) -> &mut Self::Tree {
        self.right.as_mut()
    }

    fn subtrees_mut(&mut self) -> (&mut Self::Tree, &mut Self::Tree) {
        (self.left.as_mut(), self.right.as_mut())
    }

    fn attach_left(&mut self, tree: Self::Tree) -> bool {
        if self.left.is_leaf() {
            self.left = Box::new(tree);
            true
        } else { false }
    }

    fn attach_right(&mut self, tree: Self::Tree) -> bool {
        if self.right.is_leaf() {
            self.right = Box::new(tree);
            true
        } else { false }
    }

    fn detach_left(&mut self) -> Self::Tree {
        std::mem::replace(&mut self.left, T::new_leaf())
    }

    fn detach_right(&mut self) -> Self::Tree {
        std::mem::replace(&mut self.right, T::new_leaf())
    }
    
    fn replace_left(&mut self, tree: Self::Tree) -> Self::Tree {
        std::mem::replace(&mut self.left, tree)
    }
    
    fn replace_right(&mut self, tree: Self::Tree) -> Self::Tree {
        std::mem::replace(&mut self.right, tree)
    }
}

/// Helpers for dynamic functions.
impl<K, V, T> RedBlackNode<K, V, T>
where 
    K: Ord,
    T: BinaryTree<Node = Self>,
{
    /// Rotates the left edge, making the left child the new root.
    /// Returns a true if the tree was changed (a rotation happened), and false otherwise.
    fn rotate_left(tree: &mut T) -> bool {
        let Some(root) = tree.root_mut() else { return false; };
        let mut new_tree = root.detach_left();
        if let Some(mut new_root) = new_tree.root_mut() {
            let rotating_subtree = new_root.detach_right();
            root.replace_left(rotating_subtree);
            std::mem::swap(root, &mut new_root);
            root.replace_right(new_tree);
            true
        } else {
            // Left subtree is a leaf.
            false
        }
    }

    /// Rotates the right edge, making the right child the new root.
    /// Returns a true if the tree was changed (a rotation happened), and false otherwise.
    fn rotate_right(tree: &mut T) -> bool {
        let Some(root) = tree.root_mut() else { return false; };
        let mut new_tree = root.detach_right();
        if let Some(mut new_root) = new_tree.root_mut() {
            let rotating_subtree = new_root.detach_left();
            root.replace_right(rotating_subtree);
            std::mem::swap(root, &mut new_root);
            root.replace_left(new_tree);
            true
        } else {
            // Right subtree is a leaf.
            false
        }
    }

    fn rotate_edge(tree: &mut T, side: Side) -> bool {
        match side {
            Side::Left => Self::rotate_left(tree),
            Side::Right => Self::rotate_right(tree),
        }
    }
    
    fn set_root_color(tree: &mut T, color: Color) {
        if let Some(root) = tree.root_mut() {
            root.color = color;
        }
    }
}

/// Insertions.
impl<K, V, T> RedBlackNode<K, V, T>
where 
    K: Ord,
    T: BinaryTree<Node = Self>,
{
    fn rotate_left_insertion(tree: &mut T) -> bool {
        if Self::rotate_left(tree) {
            Self::set_root_color(tree, Color::Black);
            Self::set_root_color(tree.root_mut().unwrap().right_subtree_mut(), Color::Red); // Can unwrap safely: right subtree exists since the rotation was successful.
            true
        } else { false }
    }

    fn rotate_right_insertion(tree: &mut T) -> bool {
        if Self::rotate_right(tree) {
            Self::set_root_color(tree, Color::Black);
            Self::set_root_color(tree.root_mut().unwrap().left_subtree_mut(), Color::Red); // Can unwrap safely: right subtree exists since the rotation was successful.
            true
        } else { false }
    }

    /// Swaps the colors of self and its children if both children (exist and) are red.
    fn color_swap(tree: &mut T) {
        let Some(root) = tree.root_mut() else { return; };
        let (left, right) = root.subtrees_mut();
        if let Some(left) = left.root_mut() && let Some(right) = right.root_mut()
            && left.color == Color::Red && right.color == Color::Red
        {
            left.color = Color::Black;
            right.color = Color::Black;
            root.color = Color::Red;
        }
    }

    fn fix_local_violation(tree: &mut T, side1: Side, side2: Side) -> bool {
        let Some(root) = tree.root_mut() else { return false; };
        if let Some(child) = root.subtree(side1).root() && let Some(grandchild) = child.subtree(side2).root()
            && child.color == Color::Red && grandchild.color == Color::Red
        {
            match (side1, side2) {
                (Side::Left, Side::Left) => Self::rotate_left_insertion(tree),
                (Side::Right, Side::Right) => Self::rotate_right_insertion(tree),
                (Side::Left, Side::Right) => {
                    // Perform a double left rotation.
                    if Self::rotate_right_insertion(root.left_subtree_mut()) {
                        Self::rotate_left_insertion(tree);
                        true
                    } else { false }
                },
                (Side::Right, Side::Left) => {
                    // Perform a double left rotation.
                    if Self::rotate_left_insertion(root.right_subtree_mut()) {
                        Self::rotate_right_insertion(tree);
                        true
                    } else { false }
                },
            }
        } else { false }
    }

    /// Inserts the key-value pair into the tree.
    /// If the key was not present in the tree yet, None is returned.
    /// Otherwise, the value stored at the given key is updated, and the old value is returned.
    /// Time complexity: O(log n).
    pub fn insert(tree: &mut T, key: K, value: V) -> Option<V> {
        // Traverse the tree, keeping track of three nodes: the current node, its parent, and its grandparent.
        // To not need multiple mutable references into self, keep track of only the grandparent, together with the sides to take to get to parent and current.
        // We first handle the cases where there is no parent or no current, i.e., the path to the leaf where we put value is too short.
        
        let mut side1 = if let Some(root) = tree.root_mut() {
            match K::cmp(&key, &root.key) {
                Ordering::Less => Side::Left,
                Ordering::Greater => Side::Right,
                Ordering::Equal => return Some(std::mem::replace(&mut root.value, value)),
            }
        } else {
            // Tree is empty.
            *tree = T::new_node(Self::new(key, value, Color::Black));
            return None;
        };

        // Check for a color swap at every node we come accross.
        Self::color_swap(tree);

        // Walk down the tree, updating the tree as we go.
        let mut current = &mut *tree;
        loop {
            let child_tree = current.root_mut().unwrap().subtree_mut(side1); // Can unwrap safely, we ensure that current is not a leaf.
            Self::color_swap(child_tree);
            
            let side2 = if let Some(child) = child_tree.root_mut() {
                match K::cmp(&key, &child.key) {
                    Ordering::Less => Side::Left,
                    Ordering::Greater => Side::Right,
                    Ordering::Equal => return Some(std::mem::replace(&mut child.value, value)),
                }
            } else {
                // Child tree is empty.
                *child_tree = T::new_node(Self::new(key, value, Color::Red));
                break;
            };

            let grandchild_tree = child_tree.root_mut().unwrap().subtree_mut(side2); // Can unwrap safely, we ensure that child is not a leaf.
            if grandchild_tree.is_leaf() {
                *grandchild_tree = T::new_node(Self::new(key, value, Color::Red));
                Self::fix_local_violation(current, side1, side2);
                break;
            }
            Self::color_swap(grandchild_tree);

            if Self::fix_local_violation(current, side1, side2) {
                // Need to do comparison again, since tree structure has been changed.
                // In particular, grandparent has been changed, and side1 and side2 might have been changed with it.
                side1 = match K::cmp(&key, &current.root().unwrap().key) {
                    Ordering::Less => Side::Left,
                    Ordering::Greater => Side::Right,
                    Ordering::Equal => return Some(std::mem::replace(&mut current.root_mut().unwrap().value, value)),
                };
            } else { 
                // Structure of the tree is unchanged, can safely continue the search in a subtree of grandparent.
                current = current.root_mut().unwrap().subtree_mut(side1);
                side1 = side2;
            }
        }

        // Maintain the invariant that the root is black.
        Self::set_root_color(tree, Color::Black);
        None
    }
}

/// Deletions.
impl<K, V, T> RedBlackNode<K, V, T>
where 
    K: Ord,
    T: BinaryTree<Node = Self>,
{
    fn left_color(tree: &T) -> Option<Color> {
        Some(tree.root()?.left_subtree().root()?.color)
    }

    fn right_color(tree: &T) -> Option<Color> {
        Some(tree.root()?.right_subtree().root()?.color)
    }

    fn subtree_color(tree: &T, side: Side) -> Option<Color> {
        match side {
            Side::Left => Self::left_color(tree),
            Side::Right => Self::right_color(tree),
        }
    }

    fn color_flip(tree: &mut T) {
        let Some(root) = tree.root_mut() else { return; };
        root.color = root.color.opposite();
        let (left, right) = root.subtrees_mut();
        if let Some(left) = left.root_mut() {
            left.color = left.color.opposite();
        }
        if let Some(right) = right.root_mut() {
            right.color = right.color.opposite();
        }
    }

    fn rotate_edge_deletion(tree: &mut T, side: Side) -> bool {
        let Some(root) = tree.root_mut() else { return false; };
        let col_root = root.color;
        if Self::rotate_edge(tree, side) {
            Self::set_root_color(tree, col_root);
            Self::set_root_color(tree.root_mut().unwrap().subtree_mut(side.opposite()), Color::Red);
            true
        } else { false }
    }

    fn swap_root_with_predecessor(tree: &mut T) {
        let Some(root) = tree.root_mut() else { return; };
        let mut left_tree = root.detach_left();

        let Some(mut current) = left_tree.root_mut() else { return; };
        while !current.right_subtree().is_leaf() {
            current = current.right_subtree_mut().root_mut().unwrap();
        }

        std::mem::swap(&mut root.key, &mut current.key);
        std::mem::swap(&mut root.value, &mut current.value);
        root.attach_left(left_tree);
    }

    /// Helper function for removing roots with at most one subtree attached.
    /// Returns None if the root has both subtrees attached.
    /// Colors the new root black.
    fn remove_root_single_child(tree: &mut T) -> Option<(K, V)> {
        let Some(root) = tree.root_mut() else { return None; };
        if root.left.is_leaf() {
            let mut right_tree = root.detach_right();
            Self::set_root_color(&mut right_tree, Color::Black);
            let old = std::mem::replace(tree, right_tree);
            old.into_root().map(Self::into_data)
        } else if root.right.is_leaf() {
            let mut left_tree = root.detach_left();
            Self::set_root_color(&mut left_tree, Color::Black);
            let old = std::mem::replace(tree, left_tree);
            old.into_root().map(Self::into_data)
        } else {
            None
        }
    }

    /// Removes the node with the given key from the tree.
    /// Returns the key and associated value.
    /// Time complexity: O(log n).
    pub fn remove_entry<Q>(tree: &mut T, key: &Q) -> Option<(K, V)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        // Top-down deletion algorithm is for deleting nodes without subtrees.
        // To handle deleting any node, we can swap the to-be-deleted node with its predecessor.
        // We have to wait with swapping until we move out of the to-be-deleted node, since after the swap, its subtree is invalid.
        // By advancing down one level, the subtree becomes valid again.

        let mut current = &mut *tree;
        while let Some(curr_root) = current.root() {
            let (side, found) = match Q::cmp(key, curr_root.key.borrow()) {
                Ordering::Less => (Side::Left, false),
                Ordering::Greater => (Side::Right, false),
                Ordering::Equal => {
                    // If the node has at most one subtree, we can easily remove it.
                    // The remove_root_single_child function handles the no-child case as well, as by this point,
                    // the tree has been restructured sufficiently to be valid after deleting the node.
                    if let Some(data) = Self::remove_root_single_child(current) {
                        Self::set_root_color(tree, Color::Black);
                        return Some(data);
                    } else {
                        (Side::Left, true)
                    }
                }
            };

            // First phase.
            if Self::subtree_color(current, side) == Some(Color::Red) {
                // We advance without changing the location of the to-be-deleted current node.
                // The subtree we advance to contains the predecessor of current, and not current itself.
                // Thus, swapping current with its predecessor keeps the subtree valid.
                if found {
                    Self::swap_root_with_predecessor(current);
                }

                current = current.root_mut().unwrap().subtree_mut(side);
                continue;
            }

            // Second phase.
            if Self::subtree_color(current, side.opposite()) == Some(Color::Red) {
                // Current stays in the same node, since the rotation is of the opposite edge.
                Self::rotate_edge_deletion(current, side.opposite());
                current = current.root_mut().unwrap().subtree_mut(side);
            }

            // Third phase.
            if let Some(child_tree) = current.root().map(|root| root.subtree(side))
                && (Self::left_color(child_tree) == Some(Color::Red) || Self::right_color(child_tree) == Some(Color::Red))
            {
                // We advance without changing the location of the to-be-deleted current node.
                // The subtree we advance to contains the predecessor of current, and not current itself.
                // Thus, swapping current with its predecessor keeps the subtree valid.
                if found {
                    Self::swap_root_with_predecessor(current);
                }

                current = current.root_mut().unwrap().subtree_mut(side);
                continue;
            }

            Self::color_flip(current);
            if let Some(sibling_tree) = current.root_mut().map(|root| root.subtree_mut(side.opposite()))
                && (Self::left_color(sibling_tree) == Some(Color::Red) || Self::right_color(sibling_tree) == Some(Color::Red))
            {
                if Self::subtree_color(sibling_tree, side.opposite()) != Some(Color::Red) {
                    Self::rotate_edge_deletion(sibling_tree, side);
                }

                // Current stays in the same node, since the rotation is of the opposite edge.
                Self::rotate_edge_deletion(current, side.opposite());
                Self::color_flip(current);
                current = current.root_mut().unwrap().subtree_mut(side);
            }
        }

        Self::set_root_color(tree, Color::Black);
        None
    }
}

// Queries.
impl<K, V, T> RedBlackNode<K, V, T>
where
    K: Ord,
    T: BinaryTree<Node = Self>,
{
    pub fn predecessor<Q>(&self, key: &Q) -> Option<&K>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut current = self;
        let mut pred_key = None;
        loop {
            match Q::cmp(key, current.key.borrow()) {
                Ordering::Equal => return Some(&current.key),
                Ordering::Less => {
                    let Some(left) = current.left_subtree().root() else { break; };
                    current = left;
                },
                Ordering::Greater => {
                    pred_key = Some(&current.key);
                    let Some(right) = current.right_subtree().root() else { break; };
                    current = right;
                },
            }
        }
        pred_key
    }

    pub fn successor<Q>(&self, key: &Q) -> Option<&K>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut current = self;
        let mut succ_key = None;
        loop {
            match Q::cmp(key, current.key.borrow()) {
                Ordering::Equal => return Some(&current.key),
                Ordering::Less => {
                    succ_key = Some(&current.key);
                    let Some(left) = current.left_subtree().root() else { break; };
                    current = left;
                },
                Ordering::Greater => {
                    let Some(right) = current.right_subtree().root() else { break; };
                    current = right;
                },
            }
        }
        succ_key
    }

    pub fn min(&self) -> &K {
        let mut current = self;
        while let Some(child) = current.left_subtree().root() {
            current = child;
        }
        current.key()
    }

    pub fn max(&self) -> &K {
        let mut current = self;
        while let Some(child) = current.right_subtree().root() {
            current = child;
        }
        current.key()
    }

    #[inline]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.get(key).is_some()
    }

    #[inline]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.get_key_value(key).map(|(_, v)| v)
    }

    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut current = self;
        loop {
            match Q::cmp(key, current.key.borrow()) {
                Ordering::Equal => return Some(current.data()),
                Ordering::Less => current = current.left_subtree().root()?,
                Ordering::Greater => current = current.right_subtree().root()?,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use rand::prelude::*;

    use crate::binary_search_tree::red_black_tree::RedBlackTree;

    use super::*;

    fn assert_binary_search_tree<K, V>(tree: &RedBlackTree<K, V>)
    where 
        K: Clone + Ord,
    {
        fn assert_binary_search_tree_recursive<K, V>(tree: &RedBlackTree<K, V>) -> Option<(K, K)>
        where
            K: Clone + Ord,
        {
            let Some(node) = tree.root() else { return None; };
            let left_result = assert_binary_search_tree_recursive(&node.left);
            let right_result = assert_binary_search_tree_recursive(&node.right);
            if let Some((_, max_left)) = left_result.as_ref() {
                assert_eq!(K::cmp(&node.key, &max_left), Ordering::Greater);
            }
            if let Some((min_right, _)) = right_result.as_ref() {
                assert_eq!(K::cmp(&node.key, &min_right), Ordering::Less);
            }
            Some((
                left_result.map_or(node.key.clone(), |(min, _)| min),
                right_result.map_or(node.key.clone(), |(_, max)| max)
            ))
        }
        assert_binary_search_tree_recursive(tree);
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
            let Some(node) = tree.root() else { return 1; };

            // Assert no consecutive red nodes.
            if node.color == Color::Red {
                assert_ne!(node.left.root().map(|left| left.color), Some(Color::Red));
                assert_ne!(node.right.root().map(|right| right.color), Some(Color::Red));
            }

            // Assert validity of subtrees.
            let num_black_left = assert_valid_tree_recursive(&node.left);
            let num_black_right = assert_valid_tree_recursive(&node.right);

            // Assert black counts match.
            assert_eq!(num_black_left, num_black_right);

            // Return number of black nodes on any root-to-leaf path.
            if node.color == Color::Red {
                num_black_left
            } else {
                1 + num_black_left
            }
        }

        if let Some(node) = tree.root() {
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
