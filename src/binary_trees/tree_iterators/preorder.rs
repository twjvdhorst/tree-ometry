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

pub struct PreorderIter<'t, T, F>
where 
    T: BinaryTree + ?Sized + 't,
    F: Fn(&T::Node) -> bool,
{
    cursor: T::Cursor<'t>,
    subtree_filter: F,
    first_iteration: bool,
}

impl<'t, T, F> PreorderIter<'t, T, F>
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

    fn is_current_node_valid(&self) -> bool {
        self.cursor.node().map_or(false, &self.subtree_filter)
    }

    fn move_cursor_left_if_valid(&mut self) -> bool {
        if !self.cursor.try_move_left() {
            return false;
        }

        if !(self.subtree_filter)(self.cursor.node().unwrap()) {
            self.cursor.move_up();
            false
        } else {
            true
        }
    }

    fn move_cursor_right_if_valid(&mut self) -> bool {
        if !self.cursor.try_move_right() {
            return false;
        }

        if !(self.subtree_filter)(self.cursor.node().unwrap()) {
            self.cursor.move_up();
            false
        } else {
            true
        }
    }
}

#[gat]
impl<'t, T, F> LendingIterator for PreorderIter<'t, T, F>
where 
    T: BinaryTree + ?Sized + 't,
    F: Fn(&T::Node) -> bool,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next T::Node;

    fn next(self: &mut PreorderIter<'t, T, F>) -> Option<&T::Node> {
        if self.first_iteration {
            // In the first iteration, simply report the root node pointed at by the cursor.
            self.first_iteration = false;
            return if self.is_current_node_valid() {
                self.cursor.node()
            } else {
                None
            };
        }
        
        // In successive iterations, the cursor starts in the node reported in the previous iteration.
        if self.move_cursor_left_if_valid() {
            self.cursor.node()
        } else if self.move_cursor_right_if_valid() {
            self.cursor.node()
        } else {
            while let Some(side) = self.cursor.move_up() {
                if side == Side::Left && self.move_cursor_right_if_valid() {
                    return self.cursor.node();
                }
            }
            None
        }
    }
}

pub struct PreorderIterMut<'t, T, F>
where 
    T: BinaryTreeMut + ?Sized + 't,
    F: Fn(&T::Node) -> bool,
{
    cursor: T::CursorMut<'t>,
    subtree_filter: F,
    first_iteration: bool,
}

impl<'t, T, F> PreorderIterMut<'t, T, F>
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

    fn is_current_node_valid(&self) -> bool {
        self.cursor.node().map_or(false, &self.subtree_filter)
    }

    fn move_cursor_left_if_valid(&mut self) -> bool {
        if !self.cursor.try_move_left() {
            return false;
        }

        if !(self.subtree_filter)(self.cursor.node().unwrap()) {
            self.cursor.move_up();
            false
        } else {
            true
        }
    }

    fn move_cursor_right_if_valid(&mut self) -> bool {
        if !self.cursor.try_move_right() {
            return false;
        }

        if !(self.subtree_filter)(self.cursor.node().unwrap()) {
            self.cursor.move_up();
            false
        } else {
            true
        }
    }
}

#[gat]
impl<'t, T, F> LendingIterator for PreorderIterMut<'t, T, F>
where 
    T: BinaryTreeMut + ?Sized + 't,
    F: Fn(&T::Node) -> bool,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut T::Node;

    fn next(self: &mut PreorderIterMut<'t, T, F>) -> Option<&mut T::Node> {
        if self.first_iteration {
            // In the first iteration, simply report the root node pointed at by the cursor.
            self.first_iteration = false;
            return if self.is_current_node_valid() {
                self.cursor.node_mut()
            } else {
                None
            };
        }
        
        // In successive iterations, the cursor starts in the node reported in the previous iteration.
        if self.move_cursor_left_if_valid() {
            self.cursor.node_mut()
        } else if self.move_cursor_right_if_valid() {
            self.cursor.node_mut()
        } else {
            while let Some(side) = self.cursor.move_up() {
                if side == Side::Left && self.move_cursor_right_if_valid() {
                    return self.cursor.node_mut();
                }
            }
            None
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
            let mut iter = PreorderIter::new(tree, |_| true);
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
            let mut iter = PreorderIter::new(tree, |_| true);
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
                        (Some(side), Some(_)) => *side == Side::Left,
                        (Some(_), None) => false,
                        (None, Some(_)) => true,
                        (None, None) => true,
                    }
                )
            }
        }
    }
}

