use std::ops::DerefMut;
use lending_iterator::prelude::*;

use crate::binary_search_trees::{
    node_traits::BinaryTreeNodeMut,
    walks::{WalkInstruction, traversal_stack::TraversalStack},
};

pub struct PreorderWalk<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Tree = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    stack: TraversalStack<'node, N, P, F>,
}

impl<'node, N, P, F> PreorderWalk<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Tree = N, NodePointer = P>,
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
impl<'node, N, P, F> LendingIterator for PreorderWalk<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Tree = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut N;

    fn next(self: &'_ mut PreorderWalk<'node, N, P, F>) -> Option<&'_ mut N> {
        if !self.stack.is_root_reported() {
            return self.stack.report_root();
        }
        
        while self.stack.pop_if_expanded().is_some() {}
        self.stack.expand_and_report()
    }
}
