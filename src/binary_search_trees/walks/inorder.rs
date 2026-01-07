use std::ops::DerefMut;
use lending_iterator::prelude::*;

use crate::binary_search_trees::{
    node_traits::{BinaryTreeNodeMut, Side},
    walks::{WalkInstruction, traversal_stack::TraversalStack},
};

pub struct InorderWalk<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    stack: TraversalStack<'node, N, P, F>,
}

impl<'node, N, P, F> InorderWalk<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    pub fn new(root: &'node mut N, instruction_fn: F) -> Self {
        Self {
            stack: TraversalStack::new(root, instruction_fn),
        }
    }
}

#[gat]
impl<'node, N, P, F> LendingIterator for InorderWalk<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut N;

    fn next(self: &'_ mut InorderWalk<'node, N, P, F>) -> Option<&'_ mut N> {
        while self.stack.pop_if_reported().is_some() {}
        if self.stack.is_empty() {
            return self.stack.report_root();
        }

        if self.stack.side_of_parent() == Some(Side::Right) && !self.stack.is_parent_reported() {
            self.stack.report_parent()
        } else {
            while self.stack.expand().is_some() {}
            self.stack.pop()
        }
    }
}
