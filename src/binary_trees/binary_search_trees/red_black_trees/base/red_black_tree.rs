use std::{borrow::Borrow, cmp::Ordering};

use super::{Cursor, CursorMut};
use crate::binary_trees::{
    Neighborhood,
    Side,
    binary_search_trees::red_black_trees::Color,
    binary_tree::{
        BinaryTree,
        BinaryTreeNode,
    },
    binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursorMut,
    },
};

pub(super) struct RbNode<T> {
    data: T,
    color: Color,
}

impl<T> RbNode<T> {
    fn data(&self) -> &T {
        &self.data
    }

    pub(super) fn into_data(self) -> T {
        self.data
    }

    pub(super) fn color(&self) -> Color {
        self.color
    }

    pub(super) fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

pub(super) struct RedBlackTree<T>(BinaryTree<RbNode<T>>);

impl<T> Default for RedBlackTree<T> {
    fn default() -> Self {
        Self(BinaryTree::default())
    }
}

impl<T> Extend<T> for RedBlackTree<T>
where 
    T: Ord,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for data in iter {
            self.insert(data, |_| {});
        }
    }
}

impl<T> FromIterator<T> for RedBlackTree<T>
where 
    T: Ord,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = Self::default();
        tree.extend(iter);
        tree
    }
}

impl<T> RedBlackTree<T> {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn cursor(&self) -> Cursor<'_, T> {
        Cursor::new(self.0.cursor())
    }

    pub fn cursor_mut(&mut self) -> CursorMut<'_, T> {
        CursorMut::new(self.0.cursor_mut())
    }
}

/// Insertions.
impl<T> RedBlackTree<T>
where 
    T: Ord,
{
    /// Moves the cursor to the direct predecessor or successor of the value being inserted.
    /// Reports the side of the node that the key should be inserted at, or None if the node contains the key already.
    fn find_node_to_insert_at(cursor: &mut CursorMut<'_, T>, data: &T) -> Option<Side> {
        while let Some(curr_data) = cursor.get().map(RbNode::data) {
            match T::cmp(&data, curr_data) {
                Ordering::Less => {
                    if !cursor.try_move_left() {
                        return Some(Side::Left);
                    }
                },
                Ordering::Greater => {
                    if !cursor.try_move_right() {
                        return Some(Side::Right);
                    }
                },
                Ordering::Equal => {
                    return None;
                },
            };
        }
        None
    }

    fn insert_fixup<F>(cursor: &mut CursorMut<'_, T>, mut on_subtree_change: F)
    where 
        F: for<'c> FnMut(&mut CursorMut<'c, T>),
    {
        // Cormen et al.'s algorithm.
        // We maintain the invariant that all nodes below the cursor have the correct semigroup value.
        while cursor.parent_color() == Some(Color::Red) {
            // At the start of the loop, cursor points to z.
            let side_current = cursor.move_up_after_subtree_change(&mut on_subtree_change).unwrap(); // Move the cursor to z.p
            let side_parent = cursor.side_of_parent().unwrap(); // Can unwrap safely, as z.p.p exists by the proof of correctness by Cormen et al.

            if cursor.uncle_color() == Some(Color::Red) {
                // Case 1
                cursor.set_color(Color::Black);
                cursor.move_up_after_subtree_change(&mut on_subtree_change); // Move the cursor to z.p.p, where it stays for the next iteration.
                cursor.set_color(Color::Red);
                cursor.set_child_color(side_parent.opposite(), Color::Black);
            } else {
                if side_current == side_parent.opposite() {
                    // Case 2
                    cursor.rotate(side_parent, &mut on_subtree_change).unwrap();
                    cursor.move_up_after_subtree_change(&mut on_subtree_change);
                }

                // Case 3
                cursor.set_color(Color::Black);
                cursor.move_up_after_subtree_change(&mut on_subtree_change);
                cursor.set_color(Color::Red);
                cursor.rotate(side_parent.opposite(), &mut on_subtree_change).unwrap();

                // After rotating around z.p.p, z is the sibling of the node pointed at by the cursor.
                let side = cursor.move_up_after_subtree_change(&mut on_subtree_change).unwrap();
                cursor.move_side(side.opposite());
            }
        }

        while cursor.move_up_after_subtree_change(&mut on_subtree_change).is_some() {}
    }

    /// Inserts the key-value pair into the tree.
    /// If the key was not present in the tree yet, None is returned.
    /// Otherwise, the value stored at the given key is updated, and the old value is returned.
    /// Time complexity: O(log n).
    pub fn insert<F>(&mut self, data: T, mut on_subtree_change: F) -> Option<T>
    where 
        F: for<'c> FnMut(&mut CursorMut<'c, T>),
    {
        // Cormen et al.'s algorithm.
        if self.len() == 0 {
            self.0 = BinaryTree::new_singleton(RbNode {
                data,
                color: Color::Black,
            });
            return None;
        }

        let mut cursor = self.cursor_mut();

        // Move the cursor to the direct predecessor or successor of the to-be-inserted key.
        let Some(side) = Self::find_node_to_insert_at(&mut cursor, &data) else {
            // Cursor was moved to the node containing the key.
            return Some(std::mem::replace(&mut cursor.get_mut()?.data, data));
        };

        // The cursor now points to the parent of the node we will create.
        cursor.attach_child(
            RbNode {
                data,
                color: Color::Red,
            },
            side,
            &mut on_subtree_change,
        ).unwrap();

        // Fix the red-black tree structure.
        cursor.move_side(side);
        Self::insert_fixup(&mut cursor, &mut on_subtree_change);

        // Maintain the invariant that the root is black.
        self.0.root_mut()
            .map(BinaryTreeNode::data_mut)
            .unwrap() // Can unwrap safely: we already handled the case where the tree was empty.
            .set_color(Color::Black);
        None
    }
}

/// Deletions.
impl<T> RedBlackTree<T>
where 
    T: Ord,
{
    /// Creates a cursor at the node storing the given data.
    /// Returns None if the data is not in the tree.
    fn get_cursor_mut_at_data<Q>(&mut self, data: &Q) -> Option<CursorMut<'_, T>>
    where 
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut cursor = self.cursor_mut();
        while let Some(curr_data) = cursor.get().map(RbNode::data) {
            match Q::cmp(data, curr_data.borrow()) {
                Ordering::Less => cursor.move_left(),
                Ordering::Greater => cursor.move_right(),
                Ordering::Equal => return Some(cursor),
            };
        }
        None
    }

    fn remove_fixup_leaf<F>(cursor: &mut CursorMut<'_, T>, mut side: Side, mut on_subtree_change: F)
    where 
        F: for<'c> FnMut(&mut CursorMut<'c, T>),
    {
        // We maintain the invariant that all nodes below the cursor have the correct semigroup value.
        while cursor.get().is_some() && cursor.child_color(side) != Some(Color::Red) {
            if cursor.child_color(side.opposite()) == Some(Color::Red) {
                // Case 1.
                cursor.set_child_color(side.opposite(), Color::Black);
                cursor.set_color(Color::Red);
                cursor.rotate(side, &mut on_subtree_change).unwrap();
            }
            
            cursor.move_side(side.opposite()); // Move the cursor to w
            if cursor.left_color() != Some(Color::Red) && cursor.right_color() != Some(Color::Red) {
                // Case 2.
                cursor.set_color(Color::Red);
                cursor.move_up_after_subtree_change(&mut on_subtree_change); // Move the cursor to x.p
            } else {
                if cursor.child_color(side.opposite()) != Some(Color::Red) {
                    // Case 3.
                    cursor.set_child_color(side, Color::Black);
                    cursor.set_color(Color::Red);
                    cursor.rotate(side.opposite(), &mut on_subtree_change).unwrap();
                    cursor.move_up_after_subtree_change(&mut on_subtree_change);
                }

                // Case 4.
                cursor.set_color(cursor.parent_color().unwrap()); // w is the sibling of x, so x.p is also w.p
                cursor.set_child_color(side.opposite(), Color::Black);
                cursor.move_up_after_subtree_change(&mut on_subtree_change);
                cursor.set_color(Color::Black);
                cursor.rotate(side, &mut on_subtree_change).unwrap();

                // Move cursor to root and maintain the invariant that the root is black.
                while cursor.move_up_after_subtree_change(&mut on_subtree_change).is_some() {}
                return;
            }

            if let Some(side_parent) = cursor.move_up_after_subtree_change(&mut on_subtree_change) {
                side = side_parent;
            } else {
                break;
            }
        }

        // Cursor points to the parent of x.
        cursor.move_side(side);
        cursor.set_color(Color::Black);
    }

    /// Removes the node with the given data from the tree.
    /// Time complexity: O(log n).
    pub fn remove<Q, F>(&mut self, data: &Q, mut on_subtree_change: F) -> Option<T>
    where 
        T: Borrow<Q>,
        Q: Ord + ?Sized,
        F: for<'c> FnMut(&mut CursorMut<'c, T>),
    {
        // Cormen et al.'s algorithm, with some simplifications.
        let mut cursor = self.get_cursor_mut_at_data(data)?;
        if let Neighborhood { left: Some(_), right: Some(_), .. } = cursor.peek_neighborhood() {
            // Swap the data in the to-be-deleted node with its successor, which has at most 1 child.
            let [data_node, successor_node] = cursor.spawn_and_peek_nodes_mut(|[_, successor_cursor]| {
                if successor_cursor.try_move_right() {
                    while successor_cursor.try_move_left() {}
                }
            }).unwrap();
            std::mem::swap(&mut data_node.data, &mut successor_node.data);

            // Move the cursor to the successor node, which now holds the to-be-removed data.
            if cursor.try_move_right() {
                while cursor.try_move_left() {}
            }
        }

        // The to-be-removed node has at most one child.
        let key_color = cursor.color().unwrap(); // Can unwrap safely: the cursor exists, so it points to the node with the key.
        let data = match cursor.peek_neighborhood() {
            Neighborhood { left: None, right: None, .. } => {
                let Some(side) = cursor.side_of_parent() else {
                    // The to-be-deleted node is the only node left in the tree.
                    // No need to fix semigroup values after removal.
                    return cursor.detach_node(&mut on_subtree_change);
                };
                let data = cursor.detach_node(&mut on_subtree_change).unwrap();
                if key_color == Color::Black {
                    Self::remove_fixup_leaf(&mut cursor, side, &mut on_subtree_change);
                }
                while cursor.move_up_after_subtree_change(&mut on_subtree_change).is_some() {}
                data
            },
            _ => {
                // The to-be-deleted node has exactly one child.
                // This means it is black and its child is red, so we can simply transplant and recolor.
                let data = cursor.transplant_child().unwrap();
                cursor.set_color(Color::Black);
                while cursor.move_up_after_subtree_change(&mut on_subtree_change).is_some() {}
                data
            }
        };
        
        if let Some(root) = self.0.root_mut().map(BinaryTreeNode::data_mut) {
            root.color = Color::Black;
        }
        Some(data)
    }
}

#[cfg(test)]
mod tests {
    use std::{borrow::Borrow, cmp::Ordering};
    use std::collections::{HashMap, HashSet};
    use rand::prelude::*;

    use super::*;
    use crate::binary_trees::binary_tree_cursor::{BinaryTreeCursor, PeekingCursor};

    fn assert_binary_search_tree<T>(tree: &RedBlackTree<T>)
    where 
        T: Ord + Clone,
    {
        fn assert_binary_search_tree_recursive<T>(cursor: Cursor<'_, T>) -> Option<(T, T)>
        where
            T: Ord + Clone,
        {
            let data = &cursor.get()?.data;
            let mut left_cursor = cursor;
            let mut right_cursor = cursor.clone();
            let left_result = if left_cursor.try_move_left() {
                assert_binary_search_tree_recursive(left_cursor)
            } else { None };
            let right_result = if right_cursor.try_move_right() {
                assert_binary_search_tree_recursive(right_cursor)
            } else { None };

            if let Some((_, max_left)) = left_result.as_ref() {
                assert_eq!(T::cmp(&data, &max_left), Ordering::Greater);
            }
            if let Some((min_right, _)) = right_result.as_ref() {
                assert_eq!(T::cmp(&data, &min_right), Ordering::Less);
            }
            Some((
                left_result.map_or(data.clone(), |(min, _)| min),
                right_result.map_or(data.clone(), |(_, max)| max)
            ))
        }
        
        assert_binary_search_tree_recursive(tree.cursor());
    }

    /// Asserts the given tree is a valid red-black tree.
    fn assert_valid_rb_tree<T>(tree: &RedBlackTree<T>)
    where 
        T: Ord + Clone,
    {
        // Asserts the given tree is a valid red-black tree, and returns the number of black nodes on any root-to-leaf path in the tree.
        fn assert_valid_rb_tree_recursive<T>(cursor: Cursor<'_, T>) -> usize
        where
            T: Ord + Clone,
        {
            // Tree is non-empty.
            let node = cursor.get().unwrap();

            // Assert no consecutive red nodes.
            if node.color == Color::Red {
                assert_ne!(cursor.peek_left().map(|left| left.color), Some(Color::Red));
                assert_ne!(cursor.peek_right().map(|right| right.color), Some(Color::Red));
            }

            // Assert validity of subtrees.
            let mut left_cursor = cursor;
            let mut right_cursor = cursor.clone();
            let num_black_left = if left_cursor.try_move_left() {
                assert_valid_rb_tree_recursive(left_cursor)
            } else { 1 }; // Leaves are considered black.
            let num_black_right = if right_cursor.try_move_right() {
                assert_valid_rb_tree_recursive(right_cursor)
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
        if let Some(node) = cursor.get() {
            assert_eq!(node.color, Color::Black);
            assert_binary_search_tree(tree);
            assert_valid_rb_tree_recursive(cursor);
        }
    }

    #[test]
    fn test_insertion() {
        // Test inserting values in order.
        let mut tree = RedBlackTree::new();
        for i in 1..=30 {
            tree.insert(i, |_| {});
        }
        assert_valid_rb_tree(&tree);

        // Test inserting values in random order.
        let mut rng = rand::rng();
        for _ in 0..50 {
            let mut tree = RedBlackTree::new();
            let mut values = (1..=30).collect::<Vec<_>>();
            values.shuffle(&mut rng);
            for i in values {
                tree.insert(i, |_| {});
            }
            assert_valid_rb_tree(&tree);
        }

        // Test inserting and updating data.
        for _ in 0..50 {
            let mut values = (1..=5).cycle().take(30).collect::<Vec<_>>();
            values.shuffle(&mut rng);

            let mut tree = RedBlackTree::new();
            let mut set = HashSet::new();
            for i in values {
                let old_value_tree = tree.insert(i, |_| {});
                let old_value_set = if !set.insert(i) { Some(i) } else { None };
                assert_eq!(old_value_tree, old_value_set);
            }
        }
    }

    #[test]
    fn test_deletion() {
        // Test deleting values in random order.
        let mut rng = rand::rng();
        for _ in 0..50 {
            let mut values = (1..=30).collect::<Vec<_>>();
            values.shuffle(&mut rng);
            let mut tree = values.clone().into_iter().collect::<RedBlackTree<_>>();

            values.shuffle(&mut rng);
            for i in values {
                tree.remove(&i, |_| {});
                assert_valid_rb_tree(&tree);
            }
        }
    }
}

