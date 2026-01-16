use lending_iterator::prelude::*;

use crate::binary_search_tree::{
    binary_search_tree_node::Side, node_traits::BinaryTree, walks::{WalkInstruction, traversal_stack::{TraversalStack}}
};

pub struct InorderWalk<'node, T, F>
where 
    T: BinaryTree,
    F: Fn(&T::Node) -> WalkInstruction,
{
    stack: TraversalStack<'node, T, F>,
}

impl<'node, T, F> InorderWalk<'node, T, F>
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
impl<'node, T, F> LendingIterator for InorderWalk<'node, T, F>
where 
    T: BinaryTree,
    F: Fn(&T::Node) -> WalkInstruction,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut T::Node;

    fn next(self: &'_ mut InorderWalk<'node, T, F>) -> Option<&'_ mut T::Node> {
        while self.stack.pop_if_reported().is_some() {}
        if self.stack.is_empty() {
            return self.stack.report_root();
        }

        if self.stack.side_of_parent() == Some(Side::Right) && !self.stack.is_parent_reported() {
            self.stack.report_parent()
        } else {
            while !self.stack.is_expanded() && self.stack.has_subtree(Side::Left) {
                self.stack.expand();
            }
            if self.stack.has_subtree(Side::Right) {
                self.stack.expand_and_report()
            } else {
                self.stack.pop()
            }
        }
    }
}
