use lending_iterator::prelude::*;

use crate::binary_search_tree::{
    Side, 
    tree_traits::BinaryTreeMut,
    walks::{
        WalkInstruction,
        traversal_stack::TraversalStack,
    }
};

#[derive(Clone, Copy)]
enum StackLocation {
    Index(usize),
    Root,
}

struct StackFrame<T> {
    tree: T,
    parent_location: StackLocation,
    side_of_parent: Side,
}

pub(crate) struct PostorderWalkMut<'tree, T, F>
where 
    T: BinaryTreeMut,
    F: Fn(&T) -> WalkInstruction,
{
    stack: TraversalStack<'tree, T>,
    instruction_fn: F,
}

impl<'tree, T, F> PostorderWalkMut<'tree, T, F>
where 
    T: BinaryTreeMut,
    F: Fn(&T) -> WalkInstruction,
{
    pub fn new(tree: &'tree mut T, instruction_fn: F) -> Self {
        Self {
            stack: TraversalStack::new(tree),
            instruction_fn,
        }
    }
}

impl<'tree, T, F> PostorderWalkMut<'tree, T, F>
where 
    T: BinaryTreeMut,
    F: Fn(&T) -> WalkInstruction,
{
    fn expand(&mut self) -> bool {
        let Some(tree) = self.stack.last() else { return false; };
        match (self.instruction_fn)(tree) {
            WalkInstruction::Left => self.stack.expand_left(),
            WalkInstruction::Right => self.stack.expand_right(),
            WalkInstruction::Both => self.stack.expand_both(),
            WalkInstruction::None => false,
        }
    }
}

#[gat]
impl<'tree, T, F> LendingIterator for PostorderWalkMut<'tree, T, F>
where 
    T: BinaryTreeMut,
    F: Fn(&T) -> WalkInstruction,
{
    type Item<'next>
    where 
        Self: 'next,
        = T::NodeRefMut<'next>;

    fn next(self: &'_ mut PostorderWalkMut<'tree, T, F>) -> Option<T::NodeRefMut<'_>> {
        while self.expand() {}
        self.stack.pop()
    }
}
