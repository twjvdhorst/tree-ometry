use std::ops::DerefMut;
use lending_iterator::prelude::*;

use crate::binary_search_trees::{
    node_traits::BinaryTreeNodeMut,
    walks::{WalkInstruction, node_stack::NodeStack},
};

pub struct PostorderWalk<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    stack: NodeStack<'node, N, P, F>,
    done_iterating: bool,
}

impl<'node, N, P, F> PostorderWalk<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    pub fn new(root: &'node mut N, instruction_fn: F) -> Self {
        Self {
            stack: NodeStack::new(root, instruction_fn),
            done_iterating: false,
        }
    }
}

/// Custom drop implementation that unwinds the stack to restore the tree (minus parts that have been altered already).
impl<'node, N, P, F> Drop for PostorderWalk<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    fn drop(&mut self) {
        while let Some(_) = self.stack.pop_and_reattach() {}
    }
}

#[gat]
impl<'node, N, P, F> LendingIterator for PostorderWalk<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut N;

    fn next(self: &'_ mut PostorderWalk<'node, N, P, F>) -> Option<&'_ mut N> {
        if !self.stack.is_empty() {
            while self.stack.expand_last().is_some() {}
            self.stack.pop_and_reattach()
        } else if !self.done_iterating {
            self.done_iterating = true;
            Some(self.stack.root_mut())
        } else { None }
    }
}
