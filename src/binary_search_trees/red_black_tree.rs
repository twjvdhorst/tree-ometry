use std::fmt;

use crate::binary_search_trees::node_traits::{BinarySearchTree, BinarySearchTreeNode, Insert};
use crate::binary_search_trees::{
    red_black_node::RedBlackNode,
    node_traits::BinaryTree,
};

pub struct RedBlackTree<K, V>(Option<Box<RedBlackNode<K, V, Self>>>);

impl<K, V> Default for RedBlackTree<K, V> {
    fn default() -> Self {
        Self(None)
    }
}

impl<K, V> BinaryTree for RedBlackTree<K, V> {
    type Node = RedBlackNode<K, V, Self>;

    fn new(root: Self::Node) -> Self {
        Self(Some(Box::new(root)))
    }

    fn new_leaf() -> Self {
        Self(None)
    }

    fn root(&self) -> Option<&Self::Node> {
        self.0.as_ref().map(|root| root.as_ref())
    }

    fn root_mut(&mut self) -> Option<&mut Self::Node> {
        self.0.as_mut().map(|root| root.as_mut())
    }
}

impl<K, V> BinarySearchTree for RedBlackTree<K, V>
where 
    K: Ord,
{}

impl<K, V> Insert for RedBlackTree<K, V>
where 
    K: Ord,
{
    fn insert(&mut self, key: <Self::Node as super::node_traits::BinarySearchTreeNode>::Key, value: <Self::Node as super::node_traits::BinarySearchTreeNode>::Value) -> Option<<Self::Node as super::node_traits::BinarySearchTreeNode>::Value> {
        if let Some(root) = &mut self.0 {
            root.insert(key, value)
        } else {
            self.0 = Some(Box::new(RedBlackNode::new(key, value)));
            None
        }
    }
}
    
impl<K, V> fmt::Display for RedBlackTree<K, V>
where 
    K: fmt::Display + Ord,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recursive_fmt<K, V>(tree: &RedBlackTree<K, V>, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result
        where
            K: fmt::Display + Ord,
        {
            write!(f, "{prefix}")?;
            if is_left {
                write!(f, "├──")?;
            } else {
                write!(f, "└──")?;
            };
            if let Some(root) = tree.root() {
                write!(f, "N({})\n", root.key())?;
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
    use crate::binary_search_trees::node_traits::{BinarySearchTreeNode, BinaryTreeNode};
    use crate::binary_search_trees::red_black_node::Color;

    fn assert_binary_search_tree<K, V>(root: &RedBlackTree<K, V>)
    where 
        K: Clone + Ord,
    {
        fn assert_binary_search_tree_recursive<K, V>(tree: &RedBlackTree<K, V>) -> Option<(K, K)>
        where
            K: Clone + Ord,
        {
            let Some(root) = tree.root() else { return None; };
            if let Some(max_left) = assert_binary_search_tree_recursive(root.left_subtree()).map(|(_, max)| max) {
                assert_eq!(K::cmp(root.key(), &max_left), Ordering::Greater);
            }
            if let Some(min_right) = assert_binary_search_tree_recursive(root.right_subtree()).map(|(min, _)| min) {
                assert_eq!(K::cmp(root.key(), &min_right), Ordering::Less);
            }
            Some((
                assert_binary_search_tree_recursive(root.left_subtree()).map_or(root.key().clone(), |(min, _)| min),
                assert_binary_search_tree_recursive(root.right_subtree()).map_or(root.key().clone(), |(_, max)| max)
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
            if root.color == Color::Red {
                assert_ne!(root.left_subtree().root().map(|left| left.color), Some(Color::Red));
                assert_ne!(root.right_subtree().root().map(|right| right.color), Some(Color::Red));
            }

            // Assert validity of subtrees.
            let num_black_left = assert_valid_tree_recursive(root.left_subtree());
            let num_black_right = assert_valid_tree_recursive(root.right_subtree());

            // Assert black counts match.
            assert_eq!(num_black_left, num_black_right);

            // Return number of black nodes on any root-to-leaf path.
            if root.color == Color::Red {
                num_black_left
            } else {
                1 + num_black_left
            }
        }

        if let Some(root) = tree.root() {
            assert_eq!(root.color, Color::Black);
        }
        assert_binary_search_tree(tree);
        assert_valid_tree_recursive(tree);
    }

    #[test]
    fn test_insertion() {
        // Test inserting values in order.
        let mut tree = RedBlackTree::new_leaf();
        for key in 1..=30 {
            tree.insert(key, ());
        }
        assert_valid_tree(&tree);

        // Test inserting values in random order.
        let mut rng = rand::rng();
        for _ in 0..5 {
            let mut tree = RedBlackTree::new_leaf();
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

            let mut tree = RedBlackTree::new_leaf();
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
