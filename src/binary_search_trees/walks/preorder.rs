use std::ops::DerefMut;
use lending_iterator::prelude::*;

use crate::binary_search_trees::{
    node_traits::{
        BinaryTreeNodeMut,
        Side,
    },
    walks::WalkInstruction,
};

#[derive(Clone, Copy)]
enum StackLocation {
    Index(usize),
    Root,
}

struct NodeState<P> {
    node_pointer: P,
    parent_location: StackLocation,
    side: Side,
    is_expanded: bool,
}

impl<P> NodeState<P> {
    fn new(node: P, parent_location: StackLocation, side: Side) -> Self {
        Self {
            node_pointer: node,
            parent_location,
            side,
            is_expanded: false,
        }
    }

    fn node_pointer(&self) -> &P {
        &self.node_pointer
    }

    fn node_pointer_mut(&mut self) -> &mut P {
        &mut self.node_pointer
    }

    fn into_node_pointer(self) -> P {
        self.node_pointer
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

pub struct PreorderWalk<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    root: &'node mut N,
    stack: Vec<NodeState<P>>,
    instruction_fn: F,
    first_iteration: bool,
}

impl<'node, N, P, F> PreorderWalk<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    pub fn new(root: &'node mut N, instruction_fn: F) -> Self {
        // Expand root before iteration.
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
            root,
            stack,
            instruction_fn,
            first_iteration: true,
        }
    }

    fn last_index(&self) -> StackLocation {
        if !self.stack.is_empty() {
            StackLocation::Index(self.stack.len() - 1)
        } else { StackLocation::Root }
    }
    
    fn get_node_mut(&mut self, idx: StackLocation) -> Option<&mut N> {
        match idx {
            StackLocation::Index(idx) => self.stack.get_mut(idx).map(|state| state.node_pointer_mut().deref_mut()),
            StackLocation::Root => Some(self.root)
        }
    }

    fn pop_and_reattach(&mut self) -> Option<&mut N> {
        let state = self.stack.pop()?;
        let parent = self.get_node_mut(state.parent_location())?;
        let side = state.side();
        if parent.attach_child(side, state.into_node_pointer()) {
            parent.get_child_mut(side)
        } else { None }
    }

    fn expand(&mut self) -> Option<&mut N> {
        let location = self.last_index();
        let Some(state) = self.stack.last_mut() else { return None; };
        if state.is_expanded() { return None; }

        // Expand the node.
        let instruction = (self.instruction_fn)(state.node_pointer());
        let node = state.node_pointer_mut();
        let new_states = match instruction  {
            WalkInstruction::Left => {
                if let Some(left) = node.detach_left() {
                    vec![NodeState::new(left, location, Side::Left)]
                } else { Vec::new() }
            },
            WalkInstruction::Right => {
                if let Some(right) = node.detach_right() {
                    vec![NodeState::new(right, location, Side::Right)]
                } else { Vec::new() }
            },
            WalkInstruction::Both => {
                match (node.detach_left(), node.detach_right()) {
                    (Some(left), Some(right)) => {
                        vec![
                            NodeState::new(right, location, Side::Right),
                            NodeState::new(left, location, Side::Left),
                        ]
                    },
                    (Some(left), _) => {
                        vec![NodeState::new(left, location, Side::Left)]
                    },
                    (_, Some(right)) => {
                        vec![NodeState::new(right, location, Side::Right)]
                    }
                    _ => Vec::new(),
                }
            },
            WalkInstruction::None => Vec::new(),
        };

        
        state.mark_expanded();
        self.stack.extend(new_states);
        self.get_node_mut(location)
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
            return Some(self.root);
        }
        
        while let Some(state) = self.stack.last() && state.is_expanded {
            self.pop_and_reattach();
        }
        self.expand()
    }
}
