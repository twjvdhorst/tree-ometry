use lending_iterator::prelude::*;

use crate::binary_trees::{
    Side, 
    traits::{
        BinaryTree, 
        BinaryTreeMut, 
        binary_tree_cursor::{
            BinaryTreeCursor,
            PeekingCursor,
            PeekingCursorMut,
        },
    },
};

pub struct InorderIter<'t, T>(InorderIterFiltered<'t, T, fn(&T::Node) -> bool>)
where T: BinaryTree + ?Sized + 't;

pub struct InorderIterFiltered<'t, T, P> 
where 
    T: BinaryTree + ?Sized + 't,
    P: Fn(&T::Node) -> bool,
{
    cursor: T::Cursor<'t>,
    subtree_filter: P,
    first_iteration: bool,
}

impl<'t, T> InorderIter<'t, T> 
where 
    T: BinaryTree + ?Sized + 't,
{
    pub fn new(tree: &'t T) -> Self {
        Self(InorderIterFiltered::new(tree, |_| true))
    }
}

impl<'t, T, P> InorderIterFiltered<'t, T, P>
where 
    T: BinaryTree + ?Sized + 't,
    P: Fn(&T::Node) -> bool,
{
    pub fn new(tree: &'t T, subtree_filter: P) -> Self {
        Self {
            cursor: tree.cursor(),
            subtree_filter,
            first_iteration: true,
        }
    }

    fn is_cursor_in_valid_node(&self) -> bool {
        self.cursor.node().map_or(false, &self.subtree_filter)
    }

    /// Moves the given cursor to the next (possibly null) node of the inorder iterator.
    /// Assumes the cursor points to the previous element in the iterator.
    fn move_cursor_to_next_node(&mut self) {
        if self.cursor.try_move_right() {
            while self.is_cursor_in_valid_node() && self.cursor.try_move_left() {}
        } else {
            while self.cursor.move_up() == Some(Side::Right) {}
        }
    }
}

impl<'t, T> Iterator for InorderIter<'t, T>
where 
    T: BinaryTree + ?Sized + 't,
{
    type Item = &'t T::Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'t, T, P> Iterator for InorderIterFiltered<'t, T, P>
where 
    T: BinaryTree + ?Sized + 't,
    P: Fn(&T::Node) -> bool,
{
    type Item = &'t T::Node;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_iteration {
            // In the first iteration, move the cursor to the leftmost node that satisfies the filter.
            self.first_iteration = false;
            while self.is_cursor_in_valid_node() && self.cursor.try_move_left() {}
            if !self.is_cursor_in_valid_node() {
                self.cursor.move_up();
            }
            self.cursor.node()
        } else {
            self.move_cursor_to_next_node();
            self.cursor.node()
        }
    }
}

pub struct InorderIterMut<'t, T>(InorderIterFilteredMut<'t, T, fn(&T::Node) -> bool>)
where T: BinaryTreeMut + ?Sized + 't;

pub struct InorderIterFilteredMut<'t, T, P> 
where 
    T: BinaryTreeMut + ?Sized + 't,
    P: Fn(&T::Node) -> bool,
{
    cursor: T::CursorMut<'t>,
    subtree_filter: P,
    first_iteration: bool,
}

impl<'t, T> InorderIterMut<'t, T>
where 
    T: BinaryTreeMut + ?Sized + 't,
{
    pub fn new(tree: &'t mut T) -> Self {
        Self(InorderIterFilteredMut::new(tree, |_| true))
    }
}

impl<'t, T, P> InorderIterFilteredMut<'t, T, P>
where 
    T: BinaryTreeMut + ?Sized + 't,
    P: Fn(&T::Node) -> bool,
{
    pub fn new(tree: &'t mut T, subtree_filter: P) -> Self {
        Self {
            cursor: tree.cursor_mut(),
            subtree_filter,
            first_iteration: true,
        }
    }

    fn is_cursor_in_valid_node(&self) -> bool {
        self.cursor.node().map_or(false, &self.subtree_filter)
    }

    /// Moves the given cursor to the next (possibly null) node of the inorder iterator.
    /// Assumes the cursor points to the previous element in the iterator.
    fn move_cursor_to_next_node(&mut self) {
        if self.cursor.try_move_right() {
            while self.is_cursor_in_valid_node() && self.cursor.try_move_left() {}
        } else {
            while self.cursor.move_up() == Some(Side::Right) {}
        }
    }
}

#[gat]
impl<'t, T> LendingIterator for InorderIterMut<'t, T>
where 
    T: BinaryTreeMut + ?Sized + 't,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut T::Node;

    fn next(self: &mut InorderIterMut<'t, T>) -> Option<&mut T::Node> {
        self.0.next()
    }
}

#[gat]
impl<'t, T, P> LendingIterator for InorderIterFilteredMut<'t, T, P>
where 
    T: BinaryTreeMut + ?Sized + 't,
    P: Fn(&T::Node) -> bool,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut T::Node;

    fn next(self: &mut InorderIterFilteredMut<'t, T, P>) -> Option<&mut T::Node> {
        if self.first_iteration {
            // In the first iteration, move the cursor to the leftmost node that satisfies the filter.
            self.first_iteration = false;
            while self.is_cursor_in_valid_node() && self.cursor.try_move_left() {}
            if !self.is_cursor_in_valid_node() {
                self.cursor.move_up();
            }
            self.cursor.node_mut()
        } else {
            self.move_cursor_to_next_node();
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
            let mut iter = InorderIter::new(tree);
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
            let mut iter = InorderIterMut::new(tree);
            let mut sequence = Vec::new();
            while let Some(node) = iter.next() {
                sequence.push(node.key().clone());
            }
            sequence
    }

    #[test]
    fn test_inorder_iters() {
        // Test the inorder iterators for random trees.
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
