use std::ops::DerefMut;

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

#[derive(Clone, Copy, PartialEq, Eq)]
enum NodeState {
    Untouched,
    Expanded,
    Reported,
}

struct StackFrame<P> {
    node_pointer: P,
    parent_location: StackLocation,
    side_of_parent: Side,
    state: NodeState,
}

impl<N, P> StackFrame<P>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
{
    fn new(node_pointer: P, parent_location: StackLocation, side_of_parent: Side) -> Self {
        Self {
            node_pointer,
            parent_location,
            side_of_parent,
            state: NodeState::Untouched,
        }
    }

    fn node_mut(&mut self) -> &mut N {
        &mut self.node_pointer
    }

    fn into_pointer(self) -> P {
        self.node_pointer
    }

    fn attach_child(&mut self, side: Side, node_pointer: P) -> Option<&mut N> {
        self.node_pointer.attach_child(side, node_pointer);
        self.node_pointer.get_child_mut(side)
    }

    fn is_expanded(&self) -> bool {
        self.state == NodeState::Expanded
    }

    fn mark_expanded(&mut self) {
        self.state = NodeState::Expanded;
    }

    fn is_reported(&self) -> bool {
        self.state == NodeState::Reported
    }

    fn mark_reported(&mut self) {
        self.state = NodeState::Reported;
    }
}

pub(super) struct TraversalStack<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    root_state: (&'node mut N, NodeState),
    stack: Vec<StackFrame<P>>,
    instruction_fn: F,
}

/// Custom drop implementation that unwinds the stack to restore the tree.
impl<'node, N, P, F> Drop for TraversalStack<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<'node, N, P, F> TraversalStack<'node, N, P, F>
where 
    N: BinaryTreeNodeMut<Wrapper = N, NodePointer = P>,
    P: DerefMut<Target = N>,
    F: Fn(&N) -> WalkInstruction,
{
    pub(super) fn new(root: &'node mut N, instruction_fn: F) -> Self {
        // Expand root before iteration.
        let mut stack = Vec::new();
        match (instruction_fn)(&root) {
            WalkInstruction::Left => {
                if let Some(left) = root.detach_left() {
                    stack.push(StackFrame::new(left, StackLocation::Root, Side::Left));
                }
            },
            WalkInstruction::Right => {
                if let Some(right) = root.detach_right() {
                    stack.push(StackFrame::new(right, StackLocation::Root, Side::Right));
                }
            },
            WalkInstruction::Both => {
                match (root.detach_left(), root.detach_right()) {
                    (Some(left), Some(right)) => {
                        stack.push(StackFrame::new(right, StackLocation::Root, Side::Right));
                        stack.push(StackFrame::new(left, StackLocation::Root, Side::Left));
                    },
                    (Some(left), _) => {
                        stack.push(StackFrame::new(left, StackLocation::Root, Side::Left));
                    },
                    (_, Some(right)) => {
                        stack.push(StackFrame::new(right, StackLocation::Root, Side::Right));
                    },
                    _ => (),
                }
            },
            WalkInstruction::None => (),
        };

        Self {
            root_state: (root, NodeState::Expanded),
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
            StackLocation::Index(idx) => self.stack.get_mut(idx).map(|state| state.node_mut()),
            StackLocation::Root => Some(self.root_state.0)
        }
    }

    pub(super) fn report_root(&mut self) -> Option<&mut N> {
        if self.root_state.1 != NodeState::Reported {
            self.root_state.1 = NodeState::Reported;
            Some(self.root_state.0)
        } else { None }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub(super) fn is_root_reported(&self) -> bool {
        self.root_state.1 == NodeState::Reported
    }

    fn reattach(&mut self, state: StackFrame<P>) -> Option<&mut N> {
        if let StackLocation::Index(idx) = state.parent_location {
            let parent = self.stack.get_mut(idx)?;
            parent.attach_child(state.side_of_parent, state.into_pointer())
        } else {
            let side = state.side_of_parent;
            self.root_state.0.attach_child(side, state.into_pointer());
            self.root_state.0.get_child_mut(side)
        }
    }

    pub(super) fn pop(&mut self) -> Option<&mut N> {
        let state = self.stack.pop()?;
        self.reattach(state)
    }

    pub(super) fn pop_if_expanded(&mut self) -> Option<&mut N> {
        if let Some(state) = self.stack.last() && state.is_expanded() {
            let state = self.stack.pop()?;
            self.reattach(state)
        } else { None }
    }

    pub(super) fn pop_if_reported(&mut self) -> Option<&mut N> {
        if let Some(state) = self.stack.last() && state.is_reported() {
            let state = self.stack.pop()?;
            self.reattach(state)
        } else { None }
    }

    /// Expands the last node in the frontier, returning a reference to it if done successfully.
    pub(super) fn expand(&mut self) -> Option<&mut N> {
        // Move state to the expanded stack.
        let location = self.last_index();
        let state = self.stack.last_mut()?;
        if state.is_expanded() { return None; }
        state.mark_expanded();

        // Expand the node.
        let node = state.node_mut();
        let new_states = match (self.instruction_fn)(node)  {
            WalkInstruction::Left => {
                if let Some(left) = node.detach_left() {
                    vec![StackFrame::new(left, location, Side::Left)]
                } else { Vec::new() }
            },
            WalkInstruction::Right => {
                if let Some(right) = node.detach_right() {
                    vec![StackFrame::new(right, location, Side::Right)]
                } else { Vec::new() }
            },
            WalkInstruction::Both => {
                match (node.detach_left(), node.detach_right()) {
                    (Some(left), Some(right)) => {
                        vec![
                            StackFrame::new(right, location, Side::Right),
                            StackFrame::new(left, location, Side::Left),
                        ]
                    },
                    (Some(left), _) => {
                        vec![StackFrame::new(left, location, Side::Left)]
                    },
                    (_, Some(right)) => {
                        vec![StackFrame::new(right, location, Side::Right)]
                    }
                    _ => Vec::new(),
                }
            },
            WalkInstruction::None => Vec::new(),
        };
        self.stack.extend(new_states);
        self.get_node_mut(location)
    }

    pub(super) fn expand_and_report(&mut self) -> Option<&mut N> {
        self.stack.last_mut()?.mark_reported();
        self.expand()
    }

    pub(super) fn side_of_parent(&self) -> Option<Side> {
        self.stack.last().map(|state| state.side_of_parent)
    }

    pub(super) fn is_parent_reported(&self) -> bool {
        let Some(state) = self.stack.last() else { return true; };
        match state.parent_location {
            StackLocation::Index(idx) => self.stack.get(idx).map(|state| state.is_reported()).unwrap_or(true),
            StackLocation::Root => self.is_root_reported()
        }
    }

    pub(super) fn report_parent(&mut self) -> Option<&mut N> {
        let state = self.stack.last()?;
        match state.parent_location {
            StackLocation::Index(idx) => self.stack.get_mut(idx).map(|state| {
                state.mark_reported();
                state.node_mut()
            }),
            StackLocation::Root => self.report_root(),
        }
    }
}
