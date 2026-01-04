use std::ops::DerefMut;
use lending_iterator::prelude::*;

use crate::binary_search_trees::{
    node_traits::BinaryTreeNodeMut,
    walks::{WalkInstruction, node_stack::NodeStack},
};

pub struct PreorderWalk<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    stack: NodeStack<'node, N, P, F>,
    first_iteration: bool,
}

impl<'node, N, P, F> PreorderWalk<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    pub fn new(root: &'node mut N, instruction_fn: F) -> Self {
        Self {
            stack: NodeStack::new(root, instruction_fn),
            first_iteration: true,
        }
    }
}

/// Custom drop implementation that unwinds the stack to restore the tree (minus parts that have been altered already).
impl<'node, N, P, F> Drop for PreorderWalk<'node, N, P, F>
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
impl<'node, N, P, F> LendingIterator for PreorderWalk<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut N;

    fn next(self: &'_ mut PreorderWalk<'node, N, P, F>) -> Option<&'_ mut N> {
        if self.first_iteration {
            self.first_iteration = false;
            return Some(self.stack.root_mut());
        }
        
        while let Some(state) = self.stack.last() && state.is_expanded() {
            self.stack.pop_and_reattach();
        }
        self.stack.expand_last()
    }
}
