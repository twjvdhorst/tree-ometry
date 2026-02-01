use lending_iterator::prelude::*;

use crate::binary_search_tree::{
    tree_traits::{
        BinaryTree,
        BinaryTreeMut,
    },
    tree_iterators::{
        traversal_stack::TraversalStack,
        traversal_stack_mut::TraversalStackMut,
    },
};

macro_rules! impl_preorder_next {
    ($self: ident) => {{
        while let Some(tree) = $self.stack.last_tree() {
            if !($self.subtree_filter)(tree) {
                $self.stack.pop();
                continue;
            }

            if !$self.stack.is_reported() {
                return $self.stack.report();
            }

            if !$self.stack.expand_both() {
                // Last tree on the stack is either a leaf or an already expanded tree.
                $self.stack.pop();
            }
        }
        None
    }}
}

pub struct PreorderIter<'tree, T, F>
where 
    T: BinaryTree + 'tree,
    F: Fn(&T) -> bool,
{
    stack: TraversalStack<'tree, T>,
    subtree_filter: F,
}

impl<'tree, T, F> PreorderIter<'tree, T, F> 
where 
    T: BinaryTree + 'tree,
    F: Fn(&T) -> bool,
{
    pub fn new(tree: &'tree T, subtree_filter: F) -> Self {
        Self {
            stack: TraversalStack::new(tree),
            subtree_filter,
        }
    }
}

impl<'tree, T, F> Iterator for PreorderIter<'tree, T, F>
where 
    T: BinaryTree,
    F: Fn(&T) -> bool,
{
    type Item = T::NodeRef<'tree>;
    
    fn next(&mut self) -> Option<Self::Item> {
        impl_preorder_next!(self)
    }
}

pub(crate) struct PreorderIterMut<'tree, T, F>
where 
    T: BinaryTreeMut,
    F: Fn(&T) -> bool,
{
    stack: TraversalStackMut<'tree, T>,
    subtree_filter: F,
}

impl<'tree, T, F> PreorderIterMut<'tree, T, F>
where 
    T: BinaryTreeMut,
    F: Fn(&T) -> bool,
{
    pub(crate) fn new(tree: &'tree mut T, subtree_filter: F) -> Self {
        Self {
            stack: TraversalStackMut::new(tree),
            subtree_filter,
        }
    }
}

#[gat]
impl<'tree, T, F> LendingIterator for PreorderIterMut<'tree, T, F>
where 
    T: BinaryTreeMut,
    F: Fn(&T) -> bool,
{
    type Item<'next>
    where 
        Self: 'next,
        = T::NodeRefMut<'next>;

    fn next(self: &'_ mut PreorderIterMut<'tree, T, F>) -> Option<T::NodeRefMut<'_>> {
        impl_preorder_next!(self)
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
    use crate::binary_search_tree::{
        Side,
        red_black_tree::RedBlackTree,
        tree_traits::{
            BinarySearchTree,
            BinaryTree,
        },
    };

    fn path_to_key<K, V>(mut tree: &RedBlackTree<K, V>, key: &K) -> Vec<Side>
    where 
        K: Ord,
    {
        let mut path = Vec::new();
        loop {
            let Some(root_key) = tree.key() else { break; };
            match K::cmp(&key, root_key) {
                Ordering::Less => {
                    path.push(Side::Left);
                    tree = tree.left_subtree().unwrap()
                },
                Ordering::Greater => {
                    path.push(Side::Right);
                    tree = tree.right_subtree().unwrap()
                },
                Ordering::Equal => break,
            }
        }
        path
    }

    fn get_sequence<K, V>(tree: &RedBlackTree<K, V>) -> Vec<K>
    where 
        K: Ord + Clone,
    {
            let mut iter = PreorderIter::new(tree, |_| true);
            let mut sequence = Vec::new();
            while let Some(node) = iter.next() {
                sequence.push(node.key.clone());
            }
            sequence
    }

    fn get_sequence_mut<K, V>(tree: &mut RedBlackTree<K, V>) -> Vec<K>
    where 
        K: Ord + Clone,
    {
            let mut iter = PreorderIterMut::new(tree, |_| true);
            let mut sequence = Vec::new();
            while let Some(node) = iter.next() {
                sequence.push(node.key.clone());
            }
            sequence
    }

    #[test]
    fn test_preorder_walk() {
        // Test the preorder iterator for random trees.
        let mut rng = rand::rng();
        for _ in 0..5 {
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
            
            // Verify that the sequence is preorder.
            let paths = get_sequence(&tree).iter()
                .map(|key| path_to_key(&tree, key))
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
