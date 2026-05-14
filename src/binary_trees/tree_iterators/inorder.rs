use lending_iterator::prelude::*;

use crate::binary_trees::{
    Side, 
    traits::{
        BinaryTree, 
        BinaryTreeMut,
        binary_tree_cursor::{
            BinaryTreeCursor,
            BinaryTreeCursorMut
        }
    },
};

fn is_cursor_in_valid_node<C, F>(cursor: &C, subtree_filter: F) -> bool
where 
    C: BinaryTreeCursor,
    F: Fn(&C::Node) -> bool,
{
    cursor.node().map_or(false, subtree_filter)
}

/// Moves the given cursor to the next (possibly null) node of the inorder iterator.
/// Assumes the cursor points to the previous element in the iterator.
fn move_cursor_to_next_node<C, F>(cursor: &mut C, subtree_filter: F)
where 
    C: BinaryTreeCursor,
    F: Fn(&C::Node) -> bool,
{
    if cursor.try_move_right() {
        while is_cursor_in_valid_node(cursor, &subtree_filter) && cursor.try_move_left() {}
    } else {
        while cursor.move_up() == Some(Side::Right) {}
    }
}

pub struct InorderIter<'t, T, F>
where 
    T: BinaryTree + ?Sized + 't,
    F: Fn(&T::Node) -> bool,
{
    cursor: T::Cursor<'t>,
    subtree_filter: F,
    first_iteration: bool,
}

impl<'t, T, F> InorderIter<'t, T, F>
where 
    T: BinaryTree + ?Sized + 't,
    F: Fn(&T::Node) -> bool,
{
    pub fn new(tree: &'t T, subtree_filter: F) -> Self {
        Self {
            cursor: tree.cursor(),
            subtree_filter,
            first_iteration: true,
        }
    }
}

#[gat]
impl<'t, T, F> LendingIterator for InorderIter<'t, T, F>
where 
    T: BinaryTree + ?Sized + 't,
    F: Fn(&T::Node) -> bool,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next T::Node;

    fn next(self: &mut InorderIter<'t, T, F>) -> Option<&T::Node> {
        if self.first_iteration {
            // In the first iteration, move the cursor to the leftmost node that satisfies the filter.
            self.first_iteration = false;
            while is_cursor_in_valid_node(&self.cursor, &self.subtree_filter) && self.cursor.try_move_left() {}
            if !is_cursor_in_valid_node(&self.cursor, &self.subtree_filter) {
                self.cursor.move_up();
            }
            self.cursor.node()
        } else {
            move_cursor_to_next_node(&mut self.cursor, &self.subtree_filter);
            self.cursor.node()
        }
    }
}

pub struct InorderIterMut<'t, T, F>
where 
    T: BinaryTreeMut + ?Sized + 't,
    F: Fn(&T::Node) -> bool,
{
    cursor: T::CursorMut<'t>,
    subtree_filter: F,
    first_iteration: bool,
}

impl<'t, T, F> InorderIterMut<'t, T, F>
where 
    T: BinaryTreeMut + ?Sized + 't,
    F: Fn(&T::Node) -> bool,
{
    pub fn new(tree: &'t mut T, subtree_filter: F) -> Self {
        Self {
            cursor: tree.cursor_mut(),
            subtree_filter,
            first_iteration: true,
        }
    }
}

#[gat]
impl<'t, T, F> LendingIterator for InorderIterMut<'t, T, F>
where 
    T: BinaryTreeMut + ?Sized + 't,
    F: Fn(&T::Node) -> bool,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut T::Node;

    fn next(self: &mut InorderIterMut<'t, T, F>) -> Option<&mut T::Node> {
        if self.first_iteration {
            // In the first iteration, move the cursor to the leftmost node that satisfies the filter.
            self.first_iteration = false;
            while is_cursor_in_valid_node(&self.cursor, &self.subtree_filter) && self.cursor.try_move_left() {}
            if !is_cursor_in_valid_node(&self.cursor, &self.subtree_filter) {
                self.cursor.move_up();
            }
            self.cursor.node_mut()
        } else {
            move_cursor_to_next_node(&mut self.cursor, &self.subtree_filter);
            self.cursor.node_mut()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::{
        Ordering, 
        min,
    };
    use rand::seq::SliceRandom;

    use super::*;
    use crate::binary_trees::{
        Side,
        red_black_tree::RedBlackTree,
    };

    fn path_to_key<K, V>(tree: &RedBlackTree<K, V>, key: &K) -> Option<Vec<Side>>
    where 
        K: Ord,
    {
        let mut cursor = tree.cursor();
        let mut path = Vec::new();
        while let Some(node) = cursor.node() {
            match K::cmp(key, node.key()) {
                Ordering::Less => {
                    path.push(Side::Left);
                    cursor.move_left();
                },
                Ordering::Greater => {
                    path.push(Side::Right);
                    cursor.move_right();
                },
                Ordering::Equal => return Some(path),
            }
        }

        // Key not in tree.
        None
    }

    fn get_sequence<K, V>(tree: &RedBlackTree<K, V>) -> Vec<K>
    where 
        K: Ord + Clone,
    {
            let mut iter = InorderIter::new(tree, |_| true);
            let mut sequence = Vec::new();
            while let Some(node) = iter.next() {
                sequence.push(node.key().clone());
            }
            sequence
    }

    fn get_sequence_mut<K, V>(tree: &mut RedBlackTree<K, V>) -> Vec<K>
    where 
        K: Ord + Clone,
    {
            let mut iter = InorderIterMut::new(tree, |_| true);
            let mut sequence = Vec::new();
            while let Some(node) = iter.next() {
                sequence.push(node.key().clone());
            }
            sequence
    }

    #[test]
    fn test_inorder_walk() {
        // Test the inorder iterator for random trees.
        let mut rng = rand::rng();
        for _ in 0..50 {
            let mut tree = RedBlackTree::new();
            let mut keys = (1..=30).collect::<Vec<_>>();
            keys.shuffle(&mut rng);
            for key in keys {
                tree.insert(key, ());
            }

            // Ensure immutable and mutable iterators yield the same values.
            for (k1, k2) in Iterator::zip(
                get_sequence(&tree).iter(), 
                get_sequence_mut(&mut tree).iter()
            ) {
                assert!(k1 == k2);
            }
            
            // Verify that the sequence is inorder.
            let paths = get_sequence(&tree).iter()
                .map(|key| path_to_key(&tree, key).unwrap_or(Vec::new()))
                .collect::<Vec<_>>();
            for window in paths.windows(2) {
                let path1 = &window[0];
                let path2 = &window[1];
                let first_divergence_idx = Iterator::zip(path1.iter(), path2.iter())
                    .position(|(side1, side2)| side1 != side2)
                    .unwrap_or(min(path1.len(), path2.len()));

                assert!(
                    match (path1.get(first_divergence_idx), path2.get(first_divergence_idx)) {
                        (Some(side), _) => *side == Side::Left,
                        (None, Some(side)) => *side == Side::Right,
                        (None, None) => true,
                    }
                )
            }
        }
    }
}

