use lending_iterator::prelude::*;

use crate::binary_search_tree::{
    tree_traits::{
        BinaryTree,
        BinaryTreeMut,
    },
    tree_iterators::{
        WalkInstruction,
        traversal_stack::TraversalStack,
        traversal_stack_mut::TraversalStackMut,
    },
};

macro_rules! impl_inorder_iter {
    ($struct_name: ident, $tree_trait: ident, $item: ident, $stack_name: ident) => {
        #[gat]
        impl<'tree, T, F> LendingIterator for $struct_name<'tree, T, F>
        where 
            T: $tree_trait,
            F: Fn(&T) -> WalkInstruction,
        {
            type Item<'next>
            where 
                Self: 'next,
                = T::$item<'next>;

            fn next(self: &'_ mut $struct_name<'tree, T, F>) -> Option<T::$item<'_>> {
                loop {
                    let instruction = (self.instruction_fn)(self.stack.last()?);
                    if matches!(instruction, WalkInstruction::Left | WalkInstruction::Both) && self.stack.expand_left() {
                        continue;
                    }

                    // Left subtree has previously been expanded and reported.
                    if !self.stack.is_reported() {
                        return self.stack.report();
                    } else if matches!(instruction, WalkInstruction::Right | WalkInstruction::Both) && self.stack.expand_right() {
                        continue;
                    } else {
                        // Tree and right subtree have previously been reported.
                        self.stack.pop();
                        continue;
                    }
                }
            }
        }
    };
}

pub struct InorderIter<'tree, T, F>
where 
    T: BinaryTree + 'tree,
    F: Fn(&T) -> WalkInstruction,
{
    stack: TraversalStack<'tree, T>,
    instruction_fn: F,
}

impl<'tree, T, F> InorderIter<'tree, T, F> 
where 
    T: BinaryTree + 'tree,
    F: Fn(&T) -> WalkInstruction,
{
    pub fn new(tree: &'tree T, instruction_fn: F) -> Self {
        Self {
            stack: TraversalStack::new(tree),
            instruction_fn,
        }
    }
}
impl_inorder_iter!(InorderIter, BinaryTree, NodeRef, TraversalStack);

pub(crate) struct InorderIterMut<'tree, T, F>
where 
    T: BinaryTreeMut,
    F: Fn(&T) -> WalkInstruction,
{
    stack: TraversalStackMut<'tree, T>,
    instruction_fn: F,
}

impl<'tree, T, F> InorderIterMut<'tree, T, F>
where 
    T: BinaryTreeMut,
    F: Fn(&T) -> WalkInstruction,
{
    pub(crate) fn new(tree: &'tree mut T, instruction_fn: F) -> Self {
        Self {
            stack: TraversalStackMut::new(tree),
            instruction_fn,
        }
    }
}
impl_inorder_iter!(InorderIterMut, BinaryTreeMut, NodeRefMut, TraversalStackMut);

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

    #[test]
    fn test_inorder_walk() {
        // Test the inorder iterator for random trees.
        let mut rng = rand::rng();
        for _ in 0..5 {
            let mut tree = RedBlackTree::new();
            let mut keys = (1..=30).collect::<Vec<_>>();
            keys.shuffle(&mut rng);
            for key in keys {
                tree.insert(key, ());
            }

            let inorder_sequence = {
                let mut iter = InorderIterMut::new(&mut tree, |_| WalkInstruction::Both);
                let mut inorder_sequence = Vec::new();
                while let Some(node) = iter.next() {
                    inorder_sequence.push(node.key.clone());
                }
                inorder_sequence
            };
            
            let paths = inorder_sequence.iter()
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
                        (Some(side), _) => *side == Side::Left,
                        (None, Some(side)) => *side == Side::Right,
                        (None, None) => true,
                    }
                )
            }
        }
    }
}
