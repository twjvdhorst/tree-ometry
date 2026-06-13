use std::{cmp::Ordering, marker::PhantomData};

use super::{Cursor, CursorMut};
use crate::binary_trees::{Side, binary_search_trees::red_black_trees::Color, binary_tree::{BinaryTree, BinaryTreeNode}, binary_tree_cursor::{BinaryTreeCursor, PeekingCursorMut}};

pub(super) struct RbNode<T> {
    data: T,
    color: Color,
}

impl<T> RbNode<T> {
    fn data(&self) -> &T {
        &self.data
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
    //F: for<'c> FnMut(&mut CursorMut<'c, T>),//for<'c> TreeCallback<Cursor<'c> = CursorMut<'c, T, C>>,
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

    fn move_up_after_subtree_change<F>(cursor: &mut CursorMut<'_, T>, mut on_subtree_change: F) -> Option<Side>
    where 
        F: for<'c> FnMut(&mut CursorMut<'c, T>),
    {
        let side = cursor.move_up()?;
        on_subtree_change(cursor);
        Some(side)
    }

    fn insert_fixup<F>(cursor: &mut CursorMut<'_, T>, mut on_subtree_change: F)
    where 
        F: for<'c> FnMut(&mut CursorMut<'c, T>),
    {
        // Cormen et al.'s algorithm.
        // We maintain the invariant that all nodes below the cursor have the correct semigroup value.
        while cursor.parent_color() == Some(Color::Red) {
            // At the start of the loop, cursor points to z.
            let side_current = Self::move_up_after_subtree_change(cursor, &mut on_subtree_change).unwrap();//cursor.move_up_after_subtree_change().unwrap(); // Move the cursor to z.p
            let side_parent = cursor.side_of_parent().unwrap(); // Can unwrap safely, as z.p.p exists by the proof of correctness by Cormen et al.

            if cursor.uncle_color() == Some(Color::Red) {
                // Case 1
                cursor.set_color(Color::Black);
                Self::move_up_after_subtree_change(cursor, &mut on_subtree_change);//cursor.move_up_after_subtree_change(); // Move the cursor to z.p.p, where it stays for the next iteration.
                cursor.set_color(Color::Red);
                cursor.set_child_color(side_parent.opposite(), Color::Black);
            } else {
                if side_current == side_parent.opposite() {
                    // Case 2
                    cursor.rotate(side_parent).unwrap();
                    Self::move_up_after_subtree_change(cursor, &mut on_subtree_change);//cursor.move_up_after_subtree_change();
                }

                // Case 3
                cursor.set_color(Color::Black);
                Self::move_up_after_subtree_change(cursor, &mut on_subtree_change);//cursor.move_up_after_subtree_change();
                cursor.set_color(Color::Red);
                cursor.rotate(side_parent.opposite()).unwrap();

                // After rotating around z.p.p, z is the sibling of the node pointed at by the cursor.
                let side = Self::move_up_after_subtree_change(cursor, &mut on_subtree_change).unwrap();//cursor.move_up_after_subtree_change().unwrap();
                cursor.move_side(side.opposite());
            }
        }

        while Self::move_up_after_subtree_change(cursor, &mut on_subtree_change).is_some() {}//while cursor.move_up_after_subtree_change().is_some() {}
    }

    /// Inserts the key-value pair into the tree.
    /// If the key was not present in the tree yet, None is returned.
    /// Otherwise, the value stored at the given key is updated, and the old value is returned.
    /// Time complexity: O(log n).
    pub fn insert<F>(&mut self, data: T, on_subtree_change: F) -> Option<T>
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
            side
        ).unwrap();

        // Fix the red-black tree structure.
        cursor.move_side(side);
        Self::insert_fixup(&mut cursor, on_subtree_change);

        // Maintain the invariant that the root is black.
        self.0.root_mut()
            .map(BinaryTreeNode::data_mut)
            .unwrap() // Can unwrap safely: we already handled the case where the tree was empty.
            .set_color(Color::Black);
        None
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
}

