use std::ops::DerefMut;
use lending_iterator::prelude::*;

use crate::binary_search_trees::binary_search_tree_node::{
    BinaryTreeNodeMut,
    Side,
};
use super::WalkInstruction;

#[derive(Clone, Copy)]
enum StackLocation {
    Index(usize),
    Root,
}

struct NodeState<P> {
    node: P,
    parent_location: StackLocation,
    side: Side,
    is_expanded: bool,
}

impl<P> NodeState<P> {
    fn new(node: P, parent_location: StackLocation, side: Side) -> Self {
        Self {
            node,
            parent_location,
            side,
            is_expanded: false,
        }
    }

    fn node(&self) -> &P {
        &self.node
    }

    fn node_mut(&mut self) -> &mut P {
        &mut self.node
    }

    fn into_node(self) -> P {
        self.node
    }

    fn parent_location(&self) -> StackLocation {
        self.parent_location
    }

    fn side(&self) -> Side {
        self.side
    }

    fn is_expanded(&self) -> bool {
        self.is_expanded
    }

    fn mark_expanded(&mut self) {
        self.is_expanded = true;
    }
}

struct PostorderWalk<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    root: Option<&'node mut N>,
    stack: Vec<NodeState<P>>,
    instruction_fn: F,
}

impl<'node, N, P, F> PostorderWalk<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    pub fn new(root: &'node mut N, instruction_fn: F) -> Self {
        // Create iterators for left and right subtree, which can be owned.
        let mut stack = Vec::new();
        match (instruction_fn)(&root) {
            WalkInstruction::Left => {
                if let Some(left) = root.detach_left() {
                    stack.push(NodeState::new(left, StackLocation::Root, Side::Left));
                }
            },
            WalkInstruction::Right => {
                if let Some(right) = root.detach_right() {
                    stack.push(NodeState::new(right, StackLocation::Root, Side::Right));
                }
            },
            WalkInstruction::Both => {
                match (root.detach_left(), root.detach_right()) {
                    (Some(left), Some(right)) => {
                        stack.push(NodeState::new(right, StackLocation::Root, Side::Right));
                        stack.push(NodeState::new(left, StackLocation::Root, Side::Left));
                    },
                    (Some(left), _) => {
                        stack.push(NodeState::new(left, StackLocation::Root, Side::Left));
                    },
                    (_, Some(right)) => {
                        stack.push(NodeState::new(right, StackLocation::Root, Side::Right));
                    },
                    _ => (),
                }
            },
            WalkInstruction::None => (),
        };

        Self {
            root: Some(root),
            stack,
            instruction_fn,
        }
    }

    fn last_index(&self) -> StackLocation {
        if !self.stack.is_empty() {
            StackLocation::Index(self.stack.len() - 1)
        } else { StackLocation::Root }
    }
    
    fn get_node_mut(&mut self, idx: StackLocation) -> Option<&mut N> {
        match idx {
            StackLocation::Index(idx) => self.stack.get_mut(idx).map(|state| state.node_mut().deref_mut()),
            StackLocation::Root => self.root.as_deref_mut()
        }
    }

    fn pop_and_reattach(&mut self) -> Option<&mut N> {
        let state = self.stack.pop()?;
        let parent = self.get_node_mut(state.parent_location())?;
        let side = state.side();
        if parent.attach_child(side, state.into_node()) {
            parent.get_child_mut(side)
        } else { None }
    }

    fn expand_last(&mut self) -> bool {
        // Check if last has not been expanded yet.
        let parent_location = self.last_index();
        let Some(state) = self.stack.last_mut() else { return false; };
        if state.is_expanded() { return false; }

        // Expand the node.
        let instruction = (self.instruction_fn)(state.node());
        let node = state.node_mut();
        let new_states = match instruction  {
            WalkInstruction::Left => {
                if let Some(left) = node.detach_left() {
                    vec![NodeState::new(left, parent_location, Side::Left)]
                } else { Vec::new() }
            },
            WalkInstruction::Right => {
                if let Some(right) = node.detach_right() {
                    vec![NodeState::new(right, parent_location, Side::Right)]
                } else { Vec::new() }
            },
            WalkInstruction::Both => {
                match (node.detach_left(), node.detach_right()) {
                    (Some(left), Some(right)) => {
                        vec![
                            NodeState::new(right, parent_location, Side::Right),
                            NodeState::new(left, parent_location, Side::Left),
                        ]
                    },
                    (Some(left), _) => {
                        vec![NodeState::new(left, parent_location, Side::Left)]
                    },
                    (_, Some(right)) => {
                        vec![NodeState::new(right, parent_location, Side::Right)]
                    }
                    _ => Vec::new(),
                }
            },
            WalkInstruction::None => Vec::new(),
        };

        state.mark_expanded();
        if !new_states.is_empty() {
            self.stack.extend(new_states);
            true
        } else { false }
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
        while let Some(_) = self.pop_and_reattach() {}
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
            // Keep expanding nodes as long as possible.
            while self.expand_last() {}
            self.pop_and_reattach()
        } else {
            // Last iteration (or later, if self.root == None).
            self.root.take()
        }
    }
}
