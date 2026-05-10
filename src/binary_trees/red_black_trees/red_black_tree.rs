use std::fmt::Display;

use crate::binary_trees::binary_tree::binary_tree::BinaryTree;
use super::{Color, cursors::{Cursor, CursorMut}};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RedBlackNode<K, V> {
    key: K, 
    value: V,
    color: Color,
}

impl<K, V> RedBlackNode<K, V> {
    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    pub(super) fn color(&self) -> Color {
        self.color
    }

    pub(super) fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

#[derive(Clone, Debug)]
pub struct RedBlackTree<K, V>(BinaryTree<RedBlackNode<K, V>>);

impl<K, V> Default for RedBlackTree<K, V> {
    fn default() -> Self {
        Self(BinaryTree::default())
    }
}

impl<K, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cursor(&self) -> Cursor<'_, K, V> {
        Cursor::new(self.0.cursor())
    }

    pub fn cursor_mut(&mut self) -> CursorMut<'_, K, V> {
        CursorMut::new(self.0.cursor_mut())
    }
}

/// Insertions.
impl<K, V> RedBlackTree<K, V>
where 
    K: Ord,
{
    /// Inserts the key-value pair into the tree.
    /// If the key was not present in the tree yet, None is returned.
    /// Otherwise, the value stored at the given key is updated, and the old value is returned.
    /// Time complexity: O(log n).
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // TODO: Cormen's algorithm. Simple and iterative, using two passes of the tree. Works very well with cursors, since you need to peek only one node up/down.
        todo!()
    }
}

impl<K, V> Display for RedBlackNode<K, V>
where 
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {})", self.key, self.value)
    }
}

impl<K, V> Display for RedBlackTree<K, V>
where 
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use rand::prelude::*;

    use super::*;
    use crate::binary_trees::traits::binary_tree_cursor::BinaryTreeCursor;

    fn assert_binary_search_tree<K, V>(tree: &RedBlackTree<K, V>)
    where 
        K: Clone + Ord,
    {
        fn assert_binary_search_tree_recursive<K, V>(cursor: Cursor<'_, K, V>) -> Option<(K, K)>
        where
            K: Clone + Ord,
        {
            let Some(node) = cursor.node() else { return None; };
            let mut left_cursor = cursor;
            let mut right_cursor = cursor.clone();
            left_cursor.move_left();
            right_cursor.move_right();
            let left_result = assert_binary_search_tree_recursive(left_cursor);
            let right_result = assert_binary_search_tree_recursive(right_cursor);

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
        
        assert_binary_search_tree_recursive(tree.cursor());
    }

    /// Asserts the given tree is a valid red-black tree.
    fn assert_valid_tree<K, V>(tree: &RedBlackTree<K, V>)
    where 
        K: Clone + Ord,
    {
        // Asserts the given tree is a valid red-black tree, and returns the number of black nodes on any root-to-leaf path in the tree.
        fn assert_valid_tree_recursive<K, V>(cursor: Cursor<'_, K, V>) -> usize
        where
            K: Clone + Ord,
        {
            // Leaves are considered black.
            let Some(node) = cursor.node() else { return 1; };

            // Assert no consecutive red nodes.
            if node.color == Color::Red {
                assert_ne!(cursor.peek_left().map(|left| left.color), Some(Color::Red));
                assert_ne!(cursor.peek_right().map(|right| right.color), Some(Color::Red));
            }

            // Assert validity of subtrees.
            let mut left_cursor = cursor;
            let mut right_cursor = cursor.clone();
            left_cursor.move_left();
            right_cursor.move_right();
            let num_black_left = assert_valid_tree_recursive(left_cursor);
            let num_black_right = assert_valid_tree_recursive(right_cursor);

            // Assert black counts match.
            assert_eq!(num_black_left, num_black_right);

            // Return number of black nodes on any root-to-leaf path.
            if node.color == Color::Red {
                num_black_left
            } else {
                1 + num_black_left
            }
        }

        let cursor = tree.cursor();
        if let Some(node) = cursor.node() {
            assert_eq!(node.color, Color::Black);
        }
        assert_binary_search_tree(tree);
        assert_valid_tree_recursive(cursor);
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
}
