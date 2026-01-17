use lending_iterator::prelude::*;

use crate::binary_search_tree::{
    tree_traits::BinaryTreeMut,
    walks::{
        WalkInstruction,
        traversal_stack::TraversalStack,
    },
};

pub(crate) struct PreorderWalkMut<'node, T, F>
where 
    T: BinaryTreeMut,
    F: Fn(&T) -> WalkInstruction,
{
    stack: TraversalStack<'node, T>,
    instruction_fn: F,
}

impl<'node, T, F> PreorderWalkMut<'node, T, F>
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
impl<'node, T, F> LendingIterator for PreorderWalkMut<'node, T, F>
where 
    T: BinaryTreeMut,
    F: Fn(&T) -> WalkInstruction,
{
    type Item<'next>
    where 
        Self: 'next,
        = T::NodeRefMut<'next>;

    fn next(self: &'_ mut PreorderWalkMut<'node, T, F>) -> Option<T::NodeRefMut<'_>> {
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
