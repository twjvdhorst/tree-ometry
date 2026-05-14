use std::{borrow::Borrow, cmp::Ordering, fmt::{Debug, Display}, mem::MaybeUninit};

use crate::binary_trees::{Side, binary_tree::{BinaryTree, BinaryTreeNode}, traits::binary_tree_cursor::{BinaryTreeCursor, BinaryTreeCursorMut}};
use super::{Color, cursors::{Cursor, CursorMut}};

#[derive(Clone, Copy, PartialEq, Eq)]
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

    pub(super) fn data_mut(&mut self) -> (&mut K, &mut V) {
        (&mut self.key, &mut self.value)
    }

    pub fn into_data(self) -> (K, V) {
        (self.key, self.value)
    }

    fn is_red(&self) -> bool {
        self.color == Color::Red
    }

    fn is_black(&self) -> bool {
        self.color == Color::Black
    }

    pub(super) fn color(&self) -> Color {
        self.color
    }

    pub(super) fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

#[derive(Clone)]
pub struct RedBlackTree<K, V>(BinaryTree<RedBlackNode<K, V>>);

impl<K, V> Default for RedBlackTree<K, V> {
    fn default() -> Self {
        Self(BinaryTree::default())
    }
}

impl<K, V> Extend<(K, V)> for RedBlackTree<K, V>
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

impl<K, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn root(&self) -> Option<&RedBlackNode<K, V>> {
        self.0.root().map(BinaryTreeNode::data)
    }

    fn root_mut(&mut self) -> Option<&mut RedBlackNode<K, V>> {
        self.0.root_mut().map(BinaryTreeNode::data_mut)
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
    fn insert_fixup(cursor: &mut CursorMut<'_, K, V>) {
        // Cormen et al.'s algorithm.
        while cursor.peek_up().map_or(false, RedBlackNode::is_red) {
            // Throughout the loop, cursor points to z, and peeking_cursor moves around to check states of various nodes.
            let mut peeking_cursor = cursor.spawn_cursor();
            let side_current = peeking_cursor.move_up().unwrap(); // Move the cursor to z.p
            let side_parent = peeking_cursor.move_up() // Move the cursor to z.p.p
                .unwrap(); // Can unwrap safely, as z.p.p exists by the proof of correctness by Cormen et al.

            if let Some(uncle) = peeking_cursor.peek_side(side_parent.opposite()) && uncle.is_red() {
                // Case 1
                cursor.move_up(); // Move the cursor to z.p
                cursor.set_color(Color::Black);
                cursor.move_up(); // Move the cursor to z.p.p, where it stays for the next iteration.
                cursor.set_color(Color::Red);
                cursor.peek_side_mut(side_parent.opposite()).unwrap().set_color(Color::Black);
            } else {
                if side_current == side_parent.opposite() {
                    // Case 2
                    cursor.move_up();
                    cursor.rotate(side_parent).unwrap();
                }

                // Case 3
                cursor.move_up();
                cursor.set_color(Color::Black);
                cursor.move_up();
                cursor.set_color(Color::Red);
                cursor.rotate(side_parent.opposite()).unwrap();

                // After rotating around z.p.p, z is the sibling of the node pointed at by the cursor.
                let side = cursor.move_up().unwrap();
                cursor.move_side(side.opposite());
            }
        }
    }

    /// Inserts the key-value pair into the tree.
    /// If the key was not present in the tree yet, None is returned.
    /// Otherwise, the value stored at the given key is updated, and the old value is returned.
    /// Time complexity: O(log n).
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // Cormen et al.'s algorithm.
        if self.root().is_none() {
            self.0 = BinaryTree::new_singleton(RedBlackNode {
                key,
                value,
                color: Color::Black,
            });
            return None;
        }

        let mut cursor = self.cursor_mut();

        // Move the cursor to the direct predecessor or successor of the to-be-inserted key.
        let mut side = MaybeUninit::uninit();
        while let Some(node) = cursor.node_mut() {
            match K::cmp(&key, &node.key) {
                Ordering::Less => {
                    if !cursor.try_move_left() {
                        side.write(Side::Left);
                        break;
                    }
                },
                Ordering::Greater => {
                    if !cursor.try_move_right() {
                        side.write(Side::Right);
                        break;
                    }
                },
                Ordering::Equal => {
                    let old_value = std::mem::replace(node.value_mut(), value);
                    return Some(old_value);
                },
            };
        }

        // The cursor now points to the parent of the node we will create.
        let new_node = RedBlackNode {
            key,
            value,
            color: Color::Red
        };
        let side = unsafe { side.assume_init() };
        cursor.attach_child(new_node, side).unwrap();

        // Fix the red-black tree structure.
        cursor.move_side(side);
        Self::insert_fixup(&mut cursor);

        // Maintain the invariant that the root is black.
        self.root_mut().unwrap().set_color(Color::Black); // Can unwrap safely: we already handled the case where the tree was empty.
        None
    }
}

/// Deletions.
impl<K, V> RedBlackTree<K, V>
where 
    K: Ord,
{
    /// Creates a cursor at the node storing the given key.
    /// Returns None if the key is not in the tree.
    fn get_cursor_mut_at_key<Q>(&mut self, key: &Q) -> Option<CursorMut<'_, K, V>>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut cursor = self.cursor_mut();
        while let Some(node) = cursor.node() {
            match Q::cmp(key, node.key.borrow()) {
                Ordering::Less => cursor.move_left(),
                Ordering::Greater => cursor.move_right(),
                Ordering::Equal => return Some(cursor),
            };
        }
        None
    }

    fn move_cursor_to_successor(cursor: &mut impl BinaryTreeCursor) {
        if cursor.try_move_right() {
            while cursor.try_move_left() {}
        }
    }

    fn remove_fixup_leaf(cursor: &mut CursorMut<'_, K, V>, mut side: Side) {
        while cursor.node().is_some() && cursor.peek_side(side).map_or(true, RedBlackNode::is_black) {
            let sibling = cursor.peek_side_mut(side.opposite()).unwrap(); // w
            if sibling.is_red() {
                // Case 1.
                sibling.set_color(Color::Black);
                cursor.set_color(Color::Red);
                cursor.rotate(side).unwrap();
            }
            
            cursor.move_side(side.opposite()); // Move the cursor to w
            if let (left, right) = cursor.peek_both()
                && left.map_or(true, RedBlackNode::is_black) && right.map_or(true, RedBlackNode::is_black)
            {
                // Case 2.
                cursor.set_color(Color::Red);
                cursor.move_up(); // Move the cursor to x.p
            } else {
                if cursor.peek_side(side.opposite()).map_or(true, RedBlackNode::is_black) {
                    // Case 3.
                    cursor.peek_side_mut(side).unwrap().set_color(Color::Black);
                    cursor.set_color(Color::Red);
                    cursor.rotate(side.opposite()).unwrap();
                    cursor.move_up();
                }

                // Case 4.
                cursor.set_color(cursor.peek_up().unwrap().color); // w is the sibling of x, so x.p is also w.p
                cursor.peek_side_mut(side.opposite()).unwrap().set_color(Color::Black);
                cursor.move_up();
                cursor.set_color(Color::Black);
                cursor.rotate(side).unwrap();

                // Move cursor to root and maintain the invariant that the root is black.
                while cursor.try_move_up().is_some() {}
                cursor.set_color(Color::Black);
                return;
            }

            if let Some(side_parent) = cursor.move_up() {
                side = side_parent;
            } else {
                break;
            }
        }

        // Cursor points to the parent of x.
        cursor.move_side(side);
        cursor.set_color(Color::Black);
    }

    /// Removes the node with the given key from the tree.
    /// Returns the key and associated value.
    /// Time complexity: O(log n).
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized + Debug,
    {
        // Cormen et al.'s algorithm, with some simplifications.
        let mut cursor = self.get_cursor_mut_at_key(key)?;
        if let (Some(_), Some(_)) = cursor.peek_both() {
            // Swap the data in the to-be-deleted node with its successor, which has at most 1 child.
            let [key_node, successor_node] = cursor.spawn_and_peek_mut(|[_, successor_cursor]| {
                Self::move_cursor_to_successor(successor_cursor);
            }).unwrap();
            std::mem::swap(&mut key_node.key, &mut successor_node.key);
            std::mem::swap(&mut key_node.value, &mut successor_node.value);

            // Move the cursor to the successor node, which now holds the to-be-removed data.
            Self::move_cursor_to_successor(&mut cursor);
        }

        // The to-be-removed node has at most one child.
        let key_color = cursor.node().unwrap().color; // Can unwrap safely: the cursor exists, so it points to the node with the key.
        let data = match cursor.peek_both() {
            (None, None) => {
                let Some(side) = cursor.side_of_parent() else {
                    // The to-be-deleted node is the only node left in the tree.
                    return cursor.detach_node();
                };
                let data = cursor.detach_node().unwrap();
                if key_color == Color::Black {
                    Self::remove_fixup_leaf(&mut cursor, side);
                }
                data
            },
            _ => {
                // The to-be-deleted node has exactly one child.
                // This means it is black and its child is red, so we can simply transplant and recolor.
                let data = cursor.transplant_child().unwrap();
                cursor.set_color(Color::Black);
                data
            }
        };
        Some(data)
    }
}

impl<K, V> Debug for RedBlackNode<K, V>
where 
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.color {
            Color::Red => "r",
            Color::Black => "b",
        };
        write!(f, "({:?}: {:?}) ({c})", self.key, self.value)
    }
}

impl<K, V> Debug for RedBlackTree<K, V>
where 
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
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
            let left_result = if left_cursor.try_move_left() {
                assert_binary_search_tree_recursive(left_cursor)
            } else { None };
            let right_result = if right_cursor.try_move_right() {
                assert_binary_search_tree_recursive(right_cursor)
            } else { None };

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
            // Tree is non-empty.
            let node = cursor.node().unwrap();

            // Assert no consecutive red nodes.
            if node.color == Color::Red {
                assert_ne!(cursor.peek_left().map(|left| left.color), Some(Color::Red));
                assert_ne!(cursor.peek_right().map(|right| right.color), Some(Color::Red));
            }

            // Assert validity of subtrees.
            let mut left_cursor = cursor;
            let mut right_cursor = cursor.clone();
            let num_black_left = if left_cursor.try_move_left() {
                assert_valid_tree_recursive(left_cursor)
            } else { 1 }; // Leaves are considered black.
            let num_black_right = if right_cursor.try_move_right() {
                assert_valid_tree_recursive(right_cursor)
            } else { 1 }; // Leaves are considered black.

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
            assert_binary_search_tree(tree);
            assert_valid_tree_recursive(cursor);
        }
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
