use std::{borrow::Borrow, cmp::Ordering};
use std::fmt;
use paste::paste;

/*use crate::binary_search_tree::tree_iterators::{
    inorder::{InorderIter, InorderIterMut},
    postorder::{PostorderIter, PostorderIterMut},
    preorder::{PreorderIter, PreorderIterMut},
};*/
use crate::binary_search_tree::tree_traits::{
    BinarySearchTreeNode, BinaryTree, BinaryTreeNode, BinaryTreeNodeMut
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

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub(crate) fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    pub fn left(&self) -> &T {
        &self.left
    }

    pub fn right(&self) -> &T {
        &self.right
    }
}
/*
impl<K, V> Extend<(K, V)> for RedBlackNode<K, V>
where 
    K: Ord,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            self.insert(key, value);
        }
    }
}

impl<K, V> FromIterator<(K, V)> for RedBlackTree<K, V>
where 
    K: Ord,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut tree = Self::default();
        tree.extend(iter);
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

    fn new_internal(key: K, value: V, color: Color) -> Self {
        Self::Internal(RedBlackNode::new(key, value, color))
    }

    fn into_root(self) -> Option<RedBlackNode<K, V>> {
        if let Self::Internal(node) = self {
            Some(node)
        } else { None }
    }

    fn data_mut(&mut self) -> Option<(&mut K, &mut V)> {
        self.root_mut().map(|root| (&mut root.key, &mut root.value))
    }

    fn into_data(self) -> Option<(K, V)> {
        self.into_root().map(|root| (root.key, root.value))
    }

    fn replace_value(&mut self, new_value: V) -> Option<V> {
        if let Some(root) = self.root_mut() {
            Some(std::mem::replace(&mut root.value, new_value))
        } else { None }
    }

    fn get_color(&self) -> Option<Color> {
        self.root().map(|root| root.color)
    }

    fn set_color(&mut self, new_color: Color) {
        if let Some(root) = self.root_mut() {
            root.color = new_color;
        }
    }
}
    */

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

impl<K, V, T> BinarySearchTreeNode for RedBlackNode<K, V, T>
where
    K: Ord,
    T: BinaryTree<Node = Self>
{
    type Key = K;
    type Value = V;

    fn data(&self) -> (&Self::Key, &Self::Value) {
        (&self.key, &self.value)
    }
}

/// Insertions.
impl<K, V, T> RedBlackNode<K, V, T>
where 
    K: Ord,
    T: BinaryTree<Node = Self>,
{
    fn rotate_left_insertion(&mut self) -> bool {
        if !<Self as BinaryTreeNodeMut>::rotate_left(self) { return false; }
        self.color = Color::Black;
        self.right_subtree_mut().root_mut().unwrap().color = Color::Red; // Can unwrap safely: left subtree exists since the rotation was successful.
        true
    }

    fn rotate_right_insertion(&mut self) -> bool {
        if !<Self as BinaryTreeNodeMut>::rotate_right(self) { return false; }
        self.color = Color::Black;
        self.left_subtree_mut().root_mut().unwrap().color = Color::Red; // Can unwrap safely: left subtree exists since the rotation was successful.
        true
    }

    /// Swaps the colors of self and its children if both children (exist and) are red.
    fn color_swap(&mut self) {
        let (left, right) = self.subtrees_mut();
        if let Some(left) = left.root_mut() && let Some(right) = right.root_mut()
            && left.color == Color::Red && right.color == Color::Red
        {
            left.color = Color::Black;
            right.color = Color::Black;
            self.color = Color::Red;
        }
    }

    fn fix_local_violation(&mut self, side1: Side, side2: Side) -> bool {
        if let Some(child) = self.subtree(side1).root() && let Some(grandchild) = child.subtree(side2).root()
            && child.color == Color::Red && grandchild.color == Color::Red
        {
            match (side1, side2) {
                (Side::Left, Side::Left) => self.rotate_left_insertion(),
                (Side::Right, Side::Right) => self.rotate_right_insertion(),
                (Side::Left, Side::Right) => {
                    // Perform a double left rotation.
                    let Some(left) = self.left_subtree_mut().root_mut() else { return false; };
                    if !left.rotate_right_insertion() { return false; }
                    self.rotate_left_insertion();
                    true
                },
                (Side::Right, Side::Left) => {
                    // Perform a double right rotation.
                    let Some(right) = self.right_subtree_mut().root_mut() else { return false; };
                    if !right.rotate_left_insertion() { return false; }
                    self.rotate_right_insertion();
                    true
                },
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
        
        let mut side1 = match K::cmp(&key, &self.key) {
            Ordering::Less => Side::Left,
            Ordering::Greater => Side::Right,
            Ordering::Equal => return Some(std::mem::replace(&mut self.value, value)),
        };

        // Check for a color swap at every node we come accross.
        self.color_swap();

        // Walk down the tree, updating the tree as we go.
        let mut current = &mut *self;
        loop {
            let Some(child) = current.subtree_mut(side1).root_mut() else {
                // Child tree is empty.
                current.attach_subtree(side1, <<RedBlackNode<K, V, T> as BinaryTreeNode>::Tree>::new_node(Self::new(key, value, Color::Red)));
                break;
            };
            child.color_swap();

            let side2 = match K::cmp(&key, &child.key) {
                Ordering::Less => Side::Left,
                Ordering::Greater => Side::Right,
                Ordering::Equal => return Some(std::mem::replace(&mut child.value, value)),
            };

            let Some(grandchild) = child.subtree_mut(side2).root_mut() else {
                // Grandchild tree is empty.
                child.attach_subtree(side2, <<RedBlackNode<K, V, T> as BinaryTreeNode>::Tree>::new_node(Self::new(key, value, Color::Red)));
                current.fix_local_violation(side1, side2);
                break;
            };
            grandchild.color_swap();

            if current.fix_local_violation(side1, side2) {
                // Need to do comparison again, since tree structure has been changed.
                // In particular, grandparent has been changed, and side1 and side2 might have been changed with it.
                side1 = match K::cmp(&key, &current.key) {
                    Ordering::Less => Side::Left,
                    Ordering::Greater => Side::Right,
                    Ordering::Equal => return Some(std::mem::replace(&mut current.value, value)),
                };
            } else { 
                // Structure of the tree is unchanged, can safely continue the search in a subtree of grandparent.
                current = current.subtree_mut(side1).root_mut().unwrap();
                side1 = side2;
            }
        }

        // Maintain the invariant that the root is black.
        self.color = Color::Black;
        None
    }
}

/// Deletions.
impl<K, V, T> RedBlackNode<K, V, T>
where 
    K: Ord,
    T: BinaryTree<Node = Self>,
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

    fn color_flip(&mut self) {
        self.color = self.color.opposite();
        let (left, right) = self.subtrees_mut();
        if let Some(left) = left.root_mut() {
            left.color = left.color.opposite();
        }
        if let Some(right) = right.root_mut() {
            right.color = right.color.opposite();
        }
    }

    fn rotate_edge_deletion(&mut self, side: Side) -> bool {
        let col_root = self.color;
        if <Self as BinaryTreeNodeMut>::rotate_edge(self, side) {
            self.color = col_root;
            self.subtree_mut(side.opposite()).root_mut().unwrap().color = Color::Red;
            true
        } else { false }
    }

    fn swap_with_predecessor(&mut self) {
        let mut left_tree = self.detach_left();

        let Some(mut current) = left_tree.root_mut() else { return; };
        while !current.right_subtree().is_leaf() {
            current = current.right_subtree_mut().root_mut().unwrap();
        }

        std::mem::swap(&mut self.key, &mut current.key);
        std::mem::swap(&mut self.value, &mut current.value);
        self.attach_left(left_tree);
    }

    /// Helper function for removing roots with at most one subtree attached.
    /// Returns None if the root has both subtrees attached.
    /// Colors the new root black.
    fn remove_root_single_child(tree: &mut T) -> Option<(K, V)> {
        let Some(root) = tree.root_mut() else { return None; };
        if root.left.is_leaf() {
            let mut right_tree = root.detach_right();
            if let Some(right) = right_tree.root_mut() {
                right.color = Color::Black;
            }
            let old = std::mem::replace(tree, right_tree);
            old.into_root().map(|root| (root.key, root.value))
        } else if root.right.is_leaf() {
            let mut left_tree = root.detach_left();
            left_tree.root_mut().unwrap().color = Color::Black; // Can unwrap safely; the left tree is not a leaf.
            let old = std::mem::replace(tree, left_tree);
            old.into_root().map(|root| (root.key, root.value))
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
                        if let Some(root) = tree.root_mut() {
                            root.color = Color::Black;
                        }
                        return Some(data);
                    } else {
                        (Side::Left, true)
                    }
                }
            };

            // First phase.
            if current.root().and_then(|root| root.subtree_color(side)) == Some(Color::Red) {
                // We advance without changing the location of the to-be-deleted current node.
                // The subtree we advance to contains the predecessor of current, and not current itself.
                // Thus, swapping current with its predecessor keeps the subtree valid.
                let curr_root = current.root_mut().unwrap();
                if found {
                    curr_root.swap_with_predecessor();
                }

                current = curr_root.subtree_mut(side);
                continue;
            }

            // Second phase.
            if current.root().and_then(|root| root.subtree_color(side.opposite())) == Some(Color::Red) {
                // Current stays in the same node, since the rotation is of the opposite edge.
                let curr_root = current.root_mut().unwrap();
                curr_root.rotate_edge_deletion(side.opposite());
                current = curr_root.subtree_mut(side);
            }

            // Third phase.
            if let Some(child) = current.root().and_then(|root| root.subtree(side).root())
                && (child.left_color() == Some(Color::Red) || child.right_color() == Some(Color::Red))
            {
                // We advance without changing the location of the to-be-deleted current node.
                // The subtree we advance to contains the predecessor of current, and not current itself.
                // Thus, swapping current with its predecessor keeps the subtree valid.
                let curr_root = current.root_mut().unwrap();
                if found {
                    curr_root.swap_with_predecessor();
                }

                current = curr_root.subtree_mut(side);
                continue;
            }

            if !current.is_leaf() {
                let curr_root = current.root_mut().unwrap();
                curr_root.color_flip();
            }

            if let Some(sibling_child) = current.root().and_then(|root| root.subtree(side.opposite()).root())
                && (sibling_child.left_color() == Some(Color::Red) || sibling_child.right_color() == Some(Color::Red))
            {
                if sibling_child.subtree_color(side.opposite()) != Some(Color::Red) {
                    let sibling_child = current.root_mut().unwrap().subtree_mut(side.opposite()).root_mut().unwrap();
                    sibling_child.rotate_edge_deletion(side);
                }

                // Current stays in the same node, since the rotation is of the opposite edge.
                let curr_root = current.root_mut().unwrap();
                curr_root.rotate_edge_deletion(side.opposite());
                curr_root.color_flip();
                current = curr_root.subtree_mut(side);
            }
        }

        if let Some(root) = tree.root_mut() {
            root.color = Color::Black;
        }
        None
    }
}

/*
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
            if let RedBlackTree::Internal(node) = tree {
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
            if let RedBlackTree::Internal(node) = tree {
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
    */

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
