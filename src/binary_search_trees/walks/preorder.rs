use lending_iterator::prelude::*;

use crate::binary_search_trees::{
    node_traits::BinaryTree,
    walks::{WalkInstruction, traversal_stack::TraversalStack},
};

pub struct PreorderWalk<'node, T, F>
where 
    T: BinaryTree,
    F: Fn(&T::Node) -> WalkInstruction,
{
    stack: TraversalStack<'node, T, F>,
}

impl<'node, T, F> PreorderWalk<'node, T, F>
where 
    T: BinaryTree,
    F: Fn(&T::Node) -> WalkInstruction,
{
    pub fn new(tree: &'node mut T, instruction_fn: F) -> Self {
        Self {
            stack: TraversalStack::new(tree, instruction_fn),
        }
    }
}

#[gat]
impl<'node, T, F> LendingIterator for PreorderWalk<'node, T, F>
where 
    T: BinaryTree,
    F: Fn(&T::Node) -> WalkInstruction,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut T::Node;

    fn next(self: &'_ mut PreorderWalk<'node, T, F>) -> Option<&'_ mut T::Node> {
        if !self.stack.is_root_reported() {
            return self.stack.report_root();
        }
        
        while self.stack.pop_if_expanded().is_some() {}
        self.stack.expand_and_report()
    }
}
