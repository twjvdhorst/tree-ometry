use std::{
    borrow::Borrow,
    cmp::Ordering
};
use std::fmt;

use crate::binary_trees::{
    Side,
    traits::{
        BinaryTree,
        BinaryTreeMut,
        BinaryTreeNode,
        BinaryTreeNodeMut,
        BinarySearchTreeNode,
    },
};

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
    fn new_with_color(key: K, value: V, color: Color) -> Self {
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

    pub fn value_mut(&mut self) -> &mut V {
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

impl<K, V, T> BinaryTreeNode for RedBlackNode<K, V, T>
where 
    T: BinaryTree,
{
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
    T: BinaryTree,
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

impl<K, V, T> BinarySearchTreeNode for RedBlackNode<K, V, T>
where
    K: Ord,
    T: BinaryTree,
{
    type Key = K;
    type Value = V;

    fn key(&self) -> &Self::Key {
        &self.key
    }
    
    fn value(&self) -> &Self::Value {
        &self.value
    }
}

/// Helpers for dynamic functions.
impl<K, V, T> RedBlackNode<K, V, T>
where 
    K: Ord,
    T: BinaryTreeMut<Node = Self>,
{
    fn rotate_left(&mut self) -> bool {
        let mut new_tree = self.detach_left();
        if let Some(mut new_root) = new_tree.root_mut() {
            let rotating_subtree = new_root.detach_right();
            self.replace_left(rotating_subtree);
            std::mem::swap(self, &mut new_root);
            self.replace_right(new_tree);
            true
        } else {
            // Left subtree is a leaf.
            false
        }
    }

    /// Rotates the right edge, making the right child the new root.
    /// Returns a true if the tree was changed (a rotation happened), and false otherwise.
    fn rotate_right(&mut self) -> bool {
        let mut new_tree = self.detach_right();
        if let Some(mut new_root) = new_tree.root_mut() {
            let rotating_subtree = new_root.detach_left();
            self.replace_right(rotating_subtree);
            std::mem::swap(self, &mut new_root);
            self.replace_left(new_tree);
            true
        } else {
            // Right subtree is a leaf.
            false
        }
    }

    fn rotate_edge(&mut self, side: Side) -> bool {
        match side {
            Side::Left => self.rotate_left(),
            Side::Right => self.rotate_right(),
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
    T: BinaryTreeMut<Node = Self> + From<Self>,
{
    fn rotate_edge_insertion(&mut self, side: Side) -> bool {
        if self.rotate_edge(side) {
            self.color = Color::Black;
            Self::set_root_color(self.subtree_mut(side.opposite()), Color::Red);
            true
        } else { false }
    }

    /// Swaps the colors of self and its children if both children (exist and) are red.
    fn color_swap(&mut self) {
        if let Some(left) = self.left.root_mut() && let Some(right) = self.right.root_mut()
            && left.color == Color::Red && right.color == Color::Red
        {
            self.color = Color::Red;
            left.color = Color::Black;
            right.color = Color::Black;
        }
    }

    /// Fixes a red-red edge violation in the child and grandchild of the given node.
    /// Returns whether the tree's structure has changed.
    fn fix_local_violation(&mut self, side1: Side, side2: Side) -> bool {
        if let Some(child) = self.subtree(side1).root() && let Some(grandchild) = child.subtree(side2).root()
            && child.color == Color::Red && grandchild.color == Color::Red
        {
            if side1 == side2 {
                self.rotate_edge_insertion(side1)
            } else {
                if self.subtree_mut(side1)
                    .root_mut()
                    .unwrap()
                    .rotate_edge_insertion(side2)
                {
                    self.rotate_edge_insertion(side1);
                    true
                } else { false }
            }
        } else { false }
    }

    /// Inserts the key-value pair into the tree.
    /// If the key was not present in the tree yet, None is returned.
    /// Otherwise, the value stored at the given key is updated, and the old value is returned.
    /// Time complexity: O(log n).
    pub fn insert(tree: &mut T, key: K, value: V) -> Option<V> {
        // Eternally Confuzzled's insertion algorithm.
        // Inserting the key-value pair as a red leaf is easiest. Traverse the tree and push a red node down, maintaining Red-Black Tree properties.
        // We keep track of three nodes: the current node, its parent, and its grandparent.
        // To not need multiple mutable references into self, keep track of only the grandparent, together with the sides to take to get to parent and current.
        
        let Some(root) = tree.root_mut() else {
            // Tree is empty.
            *tree = T::from(Self::new_with_color(key, value, Color::Black));
            return None;
        };
        let mut side1 = match K::cmp(&key, &root.key) {
            Ordering::Less => Side::Left,
            Ordering::Greater => Side::Right,
            Ordering::Equal => return Some(std::mem::replace(&mut root.value, value)),
        };
        root.color_swap();

        // Walk down the tree, updating the tree as we go.
        let mut current = &mut *tree;
        while let Some(root) = current.root_mut() {
            let child = {
                let child_tree = root.subtree_mut(side1);
                let Some(child) = child_tree.root_mut() else {
                    *child_tree = T::from(Self::new_with_color(key, value, Color::Red));
                    break;
                };
                child
            };
            child.color_swap();

            let side2 = match K::cmp(&key, &child.key) {
                Ordering::Less => Side::Left,
                Ordering::Greater => Side::Right,
                Ordering::Equal => return Some(std::mem::replace(&mut child.value, value)),
            };

            let grandchild = {
                let grandchild_tree = child.subtree_mut(side2);
                let Some(grandchild) = grandchild_tree.root_mut() else {
                    *grandchild_tree = T::from(Self::new_with_color(key, value, Color::Red));
                    root.fix_local_violation(side1, side2);
                    break;
                };
                grandchild
            };
            grandchild.color_swap();

            if root.fix_local_violation(side1, side2) {
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
    T: BinaryTreeMut<Node = Self>,
{
    fn left_color(&self) -> Option<Color> {
        Some(self.left_subtree().root()?.color)
    }

    fn right_color(&self) -> Option<Color> {
        Some(self.right_subtree().root()?.color)
    }

    fn subtree_color(&self, side: Side) -> Option<Color> {
        match side {
            Side::Left => self.left_color(),
            Side::Right => self.right_color(),
        }
    }

    fn has_child_with_color(&self, color: Color) -> bool {
        self.left_color() == Some(color) || self.right_color() == Some(color)
    }

    fn color_flip(&mut self) {
        self.color = self.color.opposite();
        if let Some(left) = self.left.root_mut() {
            left.color = left.color.opposite();
        }
        if let Some(right) = self.right.root_mut() {
            right.color = right.color.opposite();
        }
    }

    fn rotate_edge_deletion(&mut self, side: Side) -> bool {
        let col_root = self.color;
        if self.rotate_edge(side) {
            self.color = col_root;
            Self::set_root_color(self.subtree_mut(side.opposite()), Color::Red);
            true
        } else { false }
    }

    fn swap_root_with_predecessor(tree: &mut T) {
        let Some(root) = tree.root_mut() else { return; };
        if root.left_subtree().is_leaf() { return; };
        let (mut left_tree, right_tree) = root.detach_both();
        
        let pred = {
            let mut current = &mut left_tree;
            while let Some(root) = current.root()
                && !root.right_subtree().is_leaf()
            {
                current = current.root_mut().unwrap().right_subtree_mut();
            }
            current
        };

        // Don't need the right subtree of pred, as it is a leaf.
        let pred_left = pred.root_mut().unwrap().detach_left();

        std::mem::swap(tree, pred);

        // Swap the colors, so they are as they were originally.
        let root_tree = tree.root_mut().unwrap();
        let root_pred = pred.root_mut().unwrap();
        std::mem::swap(&mut root_tree.color, &mut root_pred.color);

        // Reconstruct the tree.
        root_pred.attach_left(pred_left);
        root_tree.attach_left(left_tree);
        root_tree.attach_right(right_tree);

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
        // Algovision's top-down deletion algorithm. This algorithm is specified for deleting nodes without subtrees.
        // To handle deleting any node, we can swap the to-be-deleted node with its predecessor, to make it a leaf.
        // We have to wait with swapping until we move out of the to-be-deleted node, as otherwise swapping makes its subtree invalid.

        let mut current = &mut *tree;
        while let Some(root) = current.root() {
            let (side, found) = match Q::cmp(key, root.key.borrow()) {
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
            let root = current.root_mut().unwrap();

            // First phase.
            if root.subtree_color(side) == Some(Color::Red) {
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
            if root.subtree_color(side.opposite()) == Some(Color::Red) {
                // Current stays in the same node, since the rotation is of the opposite edge.
                root.rotate_edge_deletion(side.opposite());
                current = current.root_mut().unwrap().subtree_mut(side);
            }

            // Third phase.
            if let Some(child) = current.root().and_then(|root| root.subtree(side).root())
                && child.has_child_with_color(Color::Red)
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

            current.root_mut().unwrap().color_flip();
            if let Some(sibling) = current.root_mut().and_then(|root| root.subtree_mut(side.opposite()).root_mut())
                && sibling.has_child_with_color(Color::Red)
            {
                if sibling.subtree_color(side.opposite()) != Some(Color::Red) {
                    sibling.rotate_edge_deletion(side);
                }

                // Current stays in the same node, since the rotation is of the opposite edge.
                let root = current.root_mut().unwrap();
                root.rotate_edge_deletion(side.opposite());
                root.color_flip();
                current = root.subtree_mut(side);
            }
        }

        Self::set_root_color(tree, Color::Black);
        None
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use rand::prelude::*;

    use super::*;
    use crate::binary_trees::red_black_trees::red_black_tree::RedBlackTree;
    use crate::binary_trees::traits::Dynamic;

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
