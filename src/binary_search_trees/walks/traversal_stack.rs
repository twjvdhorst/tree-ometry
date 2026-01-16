use crate::binary_search_trees::{
    binary_search_tree_node::Side,
    node_traits::{BinaryTree, BinaryTreeNode},
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

struct StackFrame<T> {
    tree: T,
    parent_location: StackLocation,
    side_of_parent: Side,
    state: NodeState, // Can maybe remove; if !(explore_left || explore_right) then "expanded" or "reported"
    explore_left: bool, // Wether the left subtree still needs to be reported, set to false if already reported
    explore_right: bool, // Can add "expand_left / expand_right" methods based on these bools
}

impl<T> StackFrame<T>
where 
    T: BinaryTree,
{
    fn new(tree: T, parent_location: StackLocation, side_of_parent: Side) -> Self {
        Self {
            tree,
            parent_location,
            side_of_parent,
            state: NodeState::Untouched,
        }
    }

    fn root_mut(&mut self) -> Option<&mut T::Node> {
        self.tree.root_mut()
    }

    fn into_tree(self) -> T {
        self.tree
    }

    fn attach_subtree(&mut self, side: Side, tree: T) -> Option<&mut T::Node> {
        self.tree.attach_subtree(side, tree);
        self.tree.subtree_mut(side)
            .and_then(|tree| tree.root_mut())
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

pub(super) struct TraversalStack<'tree, T, F>
where 
    T: BinaryTree,
    F: Fn(&T::Node) -> WalkInstruction,
{
    root_state: (&'tree mut T, NodeState),
    stack: Vec<StackFrame<T>>,
    instruction_fn: F,
}

/// Custom drop implementation that unwinds the stack to restore the tree.
impl<'tree, T, F> Drop for TraversalStack<'tree, T, F>
where 
    T: BinaryTree,
    F: Fn(&T::Node) -> WalkInstruction,
{
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<'tree, T, F> TraversalStack<'tree, T, F>
where 
    T: BinaryTree,
    F: Fn(&T::Node) -> WalkInstruction,
{
    pub(super) fn new(tree: &'tree mut T, instruction_fn: F) -> Self {
        // Expand root before iteration.
        let mut stack = Vec::new();
        if let Some(root) = tree.root_mut() {
            match (instruction_fn)(root) {
                WalkInstruction::Left => {
                    let left = root.detach_left();
                    if !left.is_leaf() {
                        stack.push(StackFrame::new(left, StackLocation::Root, Side::Left));
                    }
                },
                WalkInstruction::Right => {
                    let right = root.detach_left();
                    if !right.is_leaf() {
                        stack.push(StackFrame::new(right, StackLocation::Root, Side::Right));
                    }
                },
                WalkInstruction::Both => {
                    let left = root.detach_left();
                    let right = root.detach_right();
                    if !right.is_leaf() {
                        stack.push(StackFrame::new(right, StackLocation::Root, Side::Right));
                    }
                    if !left.is_leaf() {
                        stack.push(StackFrame::new(left, StackLocation::Root, Side::Left));
                    }
                },
                WalkInstruction::None => (),
            };
        }
        
        Self {
            root_state: (tree, NodeState::Expanded),
            stack,
            instruction_fn,
        }
    }

    fn last_index(&self) -> StackLocation {
        if !self.stack.is_empty() {
            StackLocation::Index(self.stack.len() - 1)
        } else { StackLocation::Root }
    }

    fn get_node_mut(&mut self, idx: StackLocation) -> Option<&mut T::Node> {
        match idx {
            StackLocation::Index(idx) => self.stack.get_mut(idx).and_then(|state| state.root_mut()),
            StackLocation::Root => self.root_state.0.root_mut()
        }
    }

    pub(super) fn report_root(&mut self) -> Option<&mut T::Node> {
        if self.root_state.1 != NodeState::Reported {
            self.root_state.1 = NodeState::Reported;
            self.root_state.0.root_mut()
        } else { None }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub(super) fn is_root_reported(&self) -> bool {
        self.root_state.1 == NodeState::Reported
    }

    pub(super) fn is_expanded(&self) -> bool {
        self.stack.last().map(|state| state.is_expanded()).unwrap_or(self.root_state.1 == NodeState::Expanded)
    }

    pub(super) fn has_subtree(&self, side: Side) -> bool {
        self.stack.last().map(|state| state.tree.has_child(side)).unwrap_or(false)
    }

    fn reattach(&mut self, state: StackFrame<T>) -> Option<&mut T::Node> {
        if let StackLocation::Index(idx) = state.parent_location {
            let parent = self.stack.get_mut(idx)?;
            parent.attach_subtree(state.side_of_parent, state.into_tree())
        } else {
            let side = state.side_of_parent;
            self.root_state.0.attach_subtree(side, state.into_tree());
            self.root_state.0.subtree_mut(side)?.root_mut()
        }
    }

    pub(super) fn pop(&mut self) -> Option<&mut T::Node> {
        let state = self.stack.pop()?;
        self.reattach(state)
    }

    pub(super) fn pop_if_expanded(&mut self) -> Option<&mut T::Node> {
        if let Some(state) = self.stack.last() && state.is_expanded() {
            let state = self.stack.pop()?;
            self.reattach(state)
        } else { None }
    }

    pub(super) fn pop_if_reported(&mut self) -> Option<&mut T::Node> {
        if let Some(state) = self.stack.last() && state.is_reported() {
            let state = self.stack.pop()?;
            self.reattach(state)
        } else { None }
    }

    /// Expands the last node in the frontier, returning a reference to it if done successfully.
    pub(super) fn expand(&mut self) -> Option<&mut T::Node> {
        // Move state to the expanded stack.
        let location = self.last_index();
        let state = self.stack.last_mut()?;
        if state.is_expanded() { return None; }
        state.mark_expanded();

        // Expand the node.
        let node = state.root_mut()?;
        match (self.instruction_fn)(node)  {
            WalkInstruction::Left => {
                let left = node.detach_left();
                if !left.is_leaf() {
                    self.stack.push(StackFrame::new(left, location, Side::Left))
                }
            },
            WalkInstruction::Right => {
                let right = node.detach_right();
                if !right.is_leaf() {
                    self.stack.push(StackFrame::new(right, location, Side::Right))
                }
            },
            WalkInstruction::Both => {
                let left = node.detach_left();
                let right = node.detach_right();
                if !right.is_leaf() {
                    self.stack.push(StackFrame::new(right, location, Side::Right))
                }
                if !left.is_leaf() {
                    self.stack.push(StackFrame::new(left, location, Side::Left))
                }
            },
            WalkInstruction::None => (),
        };
        self.get_node_mut(location)
    }

    pub(super) fn expand_and_report(&mut self) -> Option<&mut T::Node> {
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

    pub(super) fn report_parent(&mut self) -> Option<&mut T::Node> {
        let state = self.stack.last()?;
        match state.parent_location {
            StackLocation::Index(idx) => self.stack.get_mut(idx).and_then(|state| {
                state.mark_reported();
                state.root_mut()
            }),
            StackLocation::Root => self.report_root(),
        }
    }
}
