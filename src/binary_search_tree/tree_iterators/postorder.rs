use lending_iterator::prelude::*;

use crate::binary_search_tree::{
    tree_iterators::{
        WalkInstruction,
        traversal_stack::TraversalStack,
        traversal_stack_mut::TraversalStackMut,
    }, tree_traits::{BinaryTree, BinaryTreeMut}
};

macro_rules! impl_postorder_iter {
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
                let mut expand = true;
                while expand {
                    if let Some(tree) = self.stack.last() {
                        expand = match (self.instruction_fn)(tree) {
                            WalkInstruction::Left => self.stack.expand_left(),
                            WalkInstruction::Right => self.stack.expand_right(),
                            WalkInstruction::Both => self.stack.expand_both(),
                            WalkInstruction::None => false,
                        }
                    } else {
                        expand = false;
                    }
                }
                self.stack.pop()
            }
        }
    };
}

pub struct PostorderIter<'tree, T, F>
where 
    T: BinaryTree,
    F: Fn(&T) -> WalkInstruction,
{
    stack: TraversalStack<'tree, T>,
    instruction_fn: F,
}

impl<'tree, T, F> PostorderIter<'tree, T, F> 
where 
    T: BinaryTree,
    F: Fn(&T) -> WalkInstruction,
{
    pub fn new(tree: &'tree T, instruction_fn: F) -> Self {
        Self {
            stack: TraversalStack::new(tree),
            instruction_fn,
        }
    }
}
impl_postorder_iter!(PostorderIter, BinaryTree, NodeRef, TraversalStack);

pub(crate) struct PostorderIterMut<'tree, T, F>
where 
    T: BinaryTreeMut,
    F: Fn(&T) -> WalkInstruction,
{
    stack: TraversalStackMut<'tree, T>,
    instruction_fn: F,
}

impl<'tree, T, F> PostorderIterMut<'tree, T, F>
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
impl_postorder_iter!(PostorderIterMut, BinaryTreeMut, NodeRefMut, TraversalStackMut);

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
    fn test_postorder_walk() {
        // Test the postorder iterator for random trees.
        let mut rng = rand::rng();
        for _ in 0..5 {
            let mut tree = RedBlackTree::new();
            let mut keys = (1..=30).collect::<Vec<_>>();
            keys.shuffle(&mut rng);
            for key in keys {
                tree.insert(key, ());
            }

            let postorder_sequence = {
                let mut iter = PostorderIterMut::new(&mut tree, |_| WalkInstruction::Both);
                let mut postorder_sequence = Vec::new();
                while let Some(node) = iter.next() {
                    postorder_sequence.push(node.key.clone());
                }
                postorder_sequence
            };
            
            let paths = postorder_sequence.iter()
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
                        (Some(_), None) => true,
                        (None, Some(_)) => false,
                        (None, None) => true,
                    }
                )
            }
        }
    }
}
