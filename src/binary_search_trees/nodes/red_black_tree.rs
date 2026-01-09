use std::borrow::Borrow;
use std::fmt;

use crate::binary_search_trees::red_black_wrapper::RedBlackWrapper;
use crate::binary_search_trees::node_traits::{
    BinarySearchTreeNode, BinaryTree, BinaryTreeMut, BinaryTreeNode, BinaryTreeNodeMut
};

pub struct RedBlackTree<K, V>(Option<RedBlackWrapper<BSTNode<K, V, Box<Self>>>>);

pub struct BSTNode<K, V, T> {
    key: K,
    value: V,
    left: T,
    right: T,
}

impl<K, V> Default for RedBlackTree<K, V> {
    fn default() -> Self {
        Self(None)
    }
}

impl<K, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<K, V> BinaryTree for RedBlackTree<K, V> {
    type Node = RedBlackWrapper<BSTNode<K, V, Box<Self>>>;

    fn new(root: Self::Node) -> Self {
        Self(Some(root))
    }

    fn new_leaf() -> Self {
        Self(None)
    }

    fn root(&self) -> Option<&Self::Node> {
        self.0.as_ref()
    }

    fn is_leaf(&self) -> bool {
        self.0.is_none()
    }
}

impl<K, V> BinaryTreeMut for RedBlackTree<K, V> {
    fn root_mut(&mut self) -> Option<&mut Self::Node> {
        self.0.as_mut()
    }
}

impl<K, V> RedBlackTree<K, V>
where 
    K: Ord,
{
    pub fn predecessor<Q>(&self, value: &Q) -> Option<&K>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.as_ref().and_then(|root| root.predecessor(value))
    }

    pub fn successor<Q>(&self, value: &Q) -> Option<&K>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.as_ref().and_then(|root| root.successor(value))
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(root) = &mut self.0 {
            root.insert(key, value)
        } else {
            self.0 = Some(RedBlackWrapper::new(key, value));
            None
        }
    }
}

impl<K, V, T> BinaryTreeNode for BSTNode<K, V, T>
where 
    T: BinaryTree,
{
    type Tree = T;

    fn get_left(&self) -> &Self::Tree {
        &self.left
    }

    fn get_right(&self) -> &Self::Tree {
        &self.right
    }
}

impl<K, V, T> BinaryTreeNodeMut for BSTNode<K, V, T>
where 
    T: BinaryTreeMut,
{
    fn get_left_mut(&mut self) -> &mut Self::Tree {
        &mut self.left
    }

    fn get_right_mut(&mut self) -> &mut Self::Tree {
        &mut self.right
    }
    
    fn attach_left(&mut self, tree: impl Into<Self::Tree>) -> bool {
        if !self.has_left() {
            self.left = tree.into();
            true
        } else { false }
    }
    
    fn attach_right(&mut self, tree: impl Into<Self::Tree>) -> bool {
        if !self.has_right() {
            self.right = tree.into();
            true
        } else { false }
    }
    
    fn detach_left(&mut self) -> Self::Tree {
        std::mem::replace(&mut self.left, Self::Tree::new_leaf())
    }
    
    fn detach_right(&mut self) -> Self::Tree {
        std::mem::replace(&mut self.right, Self::Tree::new_leaf())
    }
    
    fn replace_left(&mut self, tree: impl Into<Self::Tree>) -> Self::Tree {
        std::mem::replace(&mut self.left, tree.into())
    }
    
    fn replace_right(&mut self, tree: impl Into<Self::Tree>) -> Self::Tree {
        std::mem::replace(&mut self.right, tree.into())
    }
}

impl<K, V, T> BinarySearchTreeNode for BSTNode<K, V, T>
where 
    K: Ord,
    T: BinaryTreeMut,
{
    type Key = K;
    type Value = V;    

    fn new(key: Self::Key, value: Self::Value) -> Self {
        Self {
            key,
            value,
            left: T::new_leaf(),
            right: T::new_leaf(),
        }
    }

    fn key(&self) -> &Self::Key {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }

    fn replace_value(&mut self, value: Self::Value) -> Self::Value {
        std::mem::replace(&mut self.value, value)
    }
}

impl<K, V> fmt::Display for RedBlackTree<K, V>
where 
    RedBlackWrapper<BSTNode<K, V, Box<Self>>>: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(root)=> root.fmt(f),
            None => write!(f, "L\n"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use rand::prelude::*;

    use super::*;
    use crate::binary_search_trees::red_black_wrapper::Color;

    fn assert_binary_search_tree<K, V>(root: &RedBlackTree<K, V>)
    where 
        K: Clone + Ord,
    {
        fn assert_binary_search_tree_recursive<K, V>(tree: &RedBlackTree<K, V>) -> Option<(K, K)>
        where
            K: Clone + Ord,
        {
            let Some(root) = tree.root() else { return None; };
            if let Some(max_left) = assert_binary_search_tree_recursive(root.get_left()).map(|(_, max)| max) {
                assert_eq!(K::cmp(root.key(), &max_left), Ordering::Greater);
            }
            if let Some(min_right) = assert_binary_search_tree_recursive(root.get_right()).map(|(min, _)| min) {
                assert_eq!(K::cmp(root.key(), &min_right), Ordering::Less);
            }
            Some((
                assert_binary_search_tree_recursive(root.get_left()).map_or(root.key().clone(), |(min, _)| min),
                assert_binary_search_tree_recursive(root.get_right()).map_or(root.key().clone(), |(_, max)| max)
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
            let Some(root) = tree.root() else { return 1; };

            // Assert no consecutive red nodes.
            if root.color() == Color::Red {
                assert_ne!(root.get_left().root().map(|left| left.color()), Some(Color::Red));
                assert_ne!(root.get_right().root().map(|right| right.color()), Some(Color::Red));
            }

            // Assert validity of subtrees.
            let num_black_left = assert_valid_tree_recursive(root.get_left());
            let num_black_right = assert_valid_tree_recursive(root.get_right());

            // Assert black counts match.
            assert_eq!(num_black_left, num_black_right);

            // Return number of black nodes on any root-to-leaf path.
            if root.color() == Color::Red {
                num_black_left
            } else {
                1 + num_black_left
            }
        }

        if let Some(root) = tree.root() {
            assert_eq!(root.color(), Color::Black);
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
            let mut keys = (1..=5).cycle();
            let mut values = (1..=30).collect::<Vec<_>>();
            values.shuffle(&mut rng);

            let mut tree = RedBlackTree::new();
            let mut key_data_map = HashMap::new();
            for value in values {
                let key = keys.next().unwrap();
                let old_value_tree = tree.insert(key.clone(), value.clone());
                let old_value_map = key_data_map.insert(key.clone(), value.clone());
                assert_eq!(old_value_tree, old_value_map);
            }
        }
    }
}
