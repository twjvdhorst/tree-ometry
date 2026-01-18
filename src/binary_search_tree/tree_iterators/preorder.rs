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

macro_rules! impl_preorder_iter {
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
                    if !self.stack.is_reported() {
                        return self.stack.report();
                    }

                    let is_expanded = match (self.instruction_fn)(self.stack.last()?) {
                        WalkInstruction::Left => self.stack.expand_left(),
                        WalkInstruction::Right => self.stack.expand_right(),
                        WalkInstruction::Both => self.stack.expand_both(),
                        WalkInstruction::None => false,
                    };
                    if !is_expanded {
                        // Last tree on the stack is either a leaf or an already expanded tree.
                        self.stack.pop();
                    }
                }
            }
        }
    };
}

pub struct PreorderIter<'tree, T, F>
where 
    T: BinaryTree + 'tree,
    F: Fn(&T) -> WalkInstruction,
{
    stack: TraversalStack<'tree, T>,
    instruction_fn: F,
}

impl<'tree, T, F> PreorderIter<'tree, T, F> 
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
impl_preorder_iter!(PreorderIter, BinaryTree, NodeRef, TraversalStack);

pub(crate) struct PreorderIterMut<'tree, T, F>
where 
    T: BinaryTreeMut,
    F: Fn(&T) -> WalkInstruction,
{
    stack: TraversalStackMut<'tree, T>,
    instruction_fn: F,
}

impl<'tree, T, F> PreorderIterMut<'tree, T, F>
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
impl_preorder_iter!(PreorderIterMut, BinaryTreeMut, NodeRefMut, TraversalStackMut);

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

            let preorder_sequence = {
                let mut iter = PreorderIterMut::new(&mut tree, |_| WalkInstruction::Both);
                let mut preorder_sequence = Vec::new();
                while let Some(node) = iter.next() {
                    preorder_sequence.push(node.key.clone());
                }
                preorder_sequence
            };
            
            let paths = preorder_sequence.iter()
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
