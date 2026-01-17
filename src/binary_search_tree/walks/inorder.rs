use lending_iterator::prelude::*;

use crate::binary_search_tree::{
    tree_traits::BinaryTreeMut,
    walks::{
        WalkInstruction,
        traversal_stack::TraversalStack,
    },
};

pub(crate) struct InorderWalkMut<'node, T, F>
where 
    T: BinaryTreeMut,
    F: Fn(&T) -> WalkInstruction,
{
    stack: TraversalStack<'node, T>,
    instruction_fn: F,
}

impl<'node, T, F> InorderWalkMut<'node, T, F>
where 
    T: BinaryTreeMut,
    F: Fn(&T) -> WalkInstruction,
{
    pub fn new(tree: &'node mut T, instruction_fn: F) -> Self {
        Self {
            stack: TraversalStack::new(tree),
            instruction_fn,
        }
    }
}

#[gat]
impl<'node, T, F> LendingIterator for InorderWalkMut<'node, T, F>
where 
    T: BinaryTreeMut,
    F: Fn(&T) -> WalkInstruction,
{
    type Item<'next>
    where 
        Self: 'next,
        = T::NodeRefMut<'next>;

    fn next(self: &'_ mut InorderWalkMut<'node, T, F>) -> Option<T::NodeRefMut<'_>> {
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
