use lending_iterator::prelude::*;

use crate::binary_search_trees::{
    node_traits::BinaryTree,
    walks::{WalkInstruction, traversal_stack::TraversalStack},
};

pub struct PostorderWalk<'tree, T, F>
where 
    T: BinaryTree,
    F: Fn(&T::Node) -> WalkInstruction,
{
    stack: TraversalStack<'tree, T, F>,
}

impl<'tree, T, F> PostorderWalk<'tree, T, F>
where 
    T: BinaryTree,
    F: Fn(&T::Node) -> WalkInstruction,
{
    pub fn new(tree: &'tree mut T, instruction_fn: F) -> Self {
        Self {
            stack: TraversalStack::new(tree, instruction_fn),
        }
    }
}

#[gat]
impl<'tree, T, F> LendingIterator for PostorderWalk<'tree, T, F>
where 
    T: BinaryTree,
    F: Fn(&T::Node) -> WalkInstruction,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut T::Node;

    fn next(self: &'_ mut PostorderWalk<'tree, T, F>) -> Option<&'_ mut T::Node> {
        if !self.stack.is_empty() {
            while self.stack.expand().is_some() {}
            self.stack.pop()
        } else {
            self.stack.report_root()
        }
    }
}
