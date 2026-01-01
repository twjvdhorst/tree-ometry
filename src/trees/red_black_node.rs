use super::binary_tree_node::BinaryTreeNode;
use super::red_black_tree::RedBlackTree;

pub struct RedBlackNode<K, V> {
    key: K,
    value: V,
    left: Option<Box<RedBlackTree<Self>>>,
    right: Option<Box<RedBlackTree<Self>>>,
}

impl<K, V> BinaryTreeNode for RedBlackNode<K, V>
where 
    K: Ord,
{
    type Key = K;
    type Value = V;
    type Tree = RedBlackTree<Self>;
    type Edge = Box<Self::Tree>;

    fn new(key: Self::Key, value: Self::Value) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
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
    
    fn get_left(&self) -> Option<&Self::Tree> {
        self.left.as_ref().map(|left| left.as_ref())
    }
    
    fn get_right(&self) -> Option<&Self::Tree> {
        self.right.as_ref().map(|right| right.as_ref())
    }
    
    fn get_left_mut(&mut self) -> Option<&mut Self::Tree> {
        self.left.as_mut().map(|left| left.as_mut())
    }
    
    fn get_right_mut(&mut self) -> Option<&mut Self::Tree> {
        self.right.as_mut().map(|right| right.as_mut())
    }
    
    fn attach_left(&mut self, tree: impl Into<Self::Edge>) -> bool {
        if !self.has_left() {
            self.left = Some(tree.into());
            true
        } else { false }
    }
    
    fn attach_right(&mut self, tree: impl Into<Self::Edge>) -> bool {
        if !self.has_right() {
            self.right = Some(tree.into());
            true
        } else { false }
    }
    
    fn detach_left(&mut self) -> Option<Self::Edge> {
        self.left.take()
    }
    
    fn detach_right(&mut self) -> Option<Self::Edge> {
        self.right.take()
    }
    
    fn replace_left(&mut self, tree: impl Into<Self::Edge>) -> Option<Self::Edge> {
        self.left.replace(tree.into())
    }
    
    fn replace_right(&mut self, tree: impl Into<Self::Edge>) -> Option<Self::Edge> {
        self.right.replace(tree.into())
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use rand::prelude::*;

    use super::*;
    use crate::trees::red_black_tree::Color;

    fn assert_binary_search_tree<K, V>(root: &RedBlackTree<RedBlackNode<K, V>>)
    where 
        K: Clone + Ord,
    {
        fn assert_binary_search_tree_recursive<K, V>(root: Option<&RedBlackTree<RedBlackNode<K, V>>>) -> Option<(K, K)>
        where
            K: Clone + Ord,
        {
            let Some(root) = root else { return None; };
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
        assert_binary_search_tree_recursive(Some(root));
    }

    /// Asserts the given tree is a valid red-black tree
    fn assert_valid_tree<K, V>(root: &RedBlackTree<RedBlackNode<K, V>>)
    where 
        K: Clone + Ord,
    {
        // Asserts the given tree is a valid red-black tree, and returns the number of black nodes on any root-to-leaf path in the tree
        fn assert_valid_tree_recursive<K, V>(root: Option<&RedBlackTree<RedBlackNode<K, V>>>) -> usize
        where
            K: Clone + Ord,
        {
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
        let mut tree = RedBlackTree::new(0, ());
        for key in 1..=30 {
            tree.insert(key, ());
        }
        assert_valid_tree(&tree);

        // Test inserting values in random order
        let mut rng = rand::rng();
        for _ in 0..5 {
            let mut tree = RedBlackTree::new(0, ());
            let mut keys = (1..=30).collect::<Vec<_>>();
            keys.shuffle(&mut rng);
            for key in keys {
                tree.insert(key, ());
            }
            assert_valid_tree(&tree);
        }

        // Test inserting and updating data
        for _ in 0..5 {
            let mut keys = (1..=5).cycle();
            let mut values = (1..=30).collect::<Vec<_>>();
            values.shuffle(&mut rng);

            let mut tree = RedBlackTree::<RedBlackNode<i32, i32>>::new(0, 0);
            let mut key_data_map = HashMap::new();
            key_data_map.insert(0, 0);
            for value in values {
                let key = keys.next().unwrap();
                let old_value_tree = tree.insert(key.clone(), value.clone());
                let old_value_map = key_data_map.insert(key.clone(), value.clone());
                assert_eq!(old_value_tree, old_value_map);
            }
        }
    }
}
