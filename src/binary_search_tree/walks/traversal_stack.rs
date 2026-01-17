use super::WalkInstruction;
use crate::binary_search_tree::{Side, tree_traits::BinaryTreeMut};

#[derive(Clone, Copy)]
enum StackLocation {
    Index(usize),
    Root,
}

struct StackFrame<T> {
    tree: T,
    parent_location: StackLocation,
    side_of_parent: Side,
    is_left_expanded: bool,
    is_right_expanded: bool,
    is_reported: bool,
}

impl<T> StackFrame<T>
where 
    T: BinaryTreeMut,
{
    fn new(tree: T, parent_location: StackLocation, side_of_parent: Side) -> Self {
        Self {
            tree,
            parent_location,
            side_of_parent,
            is_left_expanded: false,
            is_right_expanded: false,
            is_reported: false,
        }
    }
}

struct TreeState<'tree, T> {
    tree: Option<&'tree mut T>,
    is_left_expanded: bool,
    is_right_expanded: bool,
    is_reported: bool,
}

pub(super) struct TraversalStack<'tree, T>
where 
    T: BinaryTreeMut,
{
    tree_state: TreeState<'tree, T>,
    stack: Vec<StackFrame<T>>,
}

/// Custom drop implementation that unwinds the stack to restore the tree.
impl<'tree, T> Drop for TraversalStack<'tree, T>
where 
    T: BinaryTreeMut,
{
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<'tree, T> TraversalStack<'tree, T>
where 
    T: BinaryTreeMut,
{
    pub(super) fn new(tree: &'tree mut T) -> Self {
        Self {
            tree_state: TreeState {
                tree: Some(tree),
                is_left_expanded: false,
                is_right_expanded: false,
                is_reported: false
            },
            stack: Vec::new(),
        }
    }

    pub(super) fn last(&self) -> Option<&T> {
        if !self.stack.is_empty() {
            self.stack.last().map(|state| &state.tree)
        } else {
            self.tree_state.tree.as_deref()
        }
    }

    fn last_index(&self) -> StackLocation {
        if !self.stack.is_empty() {
            StackLocation::Index(self.stack.len() - 1)
        } else { StackLocation::Root }
    }

    pub(super) fn is_left_expanded(&self) -> bool {
        self.stack.last()
            .map(|state| state.is_left_expanded)
            .unwrap_or(self.tree_state.is_left_expanded)
    }

    pub(super) fn is_right_expanded(&self) -> bool {
        self.stack.last()
            .map(|state| state.is_right_expanded)
            .unwrap_or(self.tree_state.is_right_expanded)
    }

    pub(super) fn is_fully_expanded(&self) -> bool {
        self.is_left_expanded() && self.is_right_expanded()
    }

    pub(super) fn is_reported(&self) -> bool {
        self.stack.last()
            .map(|state| state.is_reported)
            .unwrap_or(self.tree_state.is_reported)
    }

    pub(super) fn report(&'_ mut self) -> Option<T::NodeRefMut<'_>> {
        if !self.stack.is_empty() && !self.stack.last()?.is_reported {
            let state = self.stack.last_mut()?;
            state.is_reported = true;
            state.tree.node_ref_mut()
        } else if !self.tree_state.is_reported {
            self.tree_state.is_reported = true;
            self.tree_state.tree.as_mut()
                .and_then(|tree| tree.node_ref_mut())
        } else {
            None
        }
    }

    fn pop_stack(&'_ mut self) -> Option<T::NodeRefMut<'_>> {
        let state = self.stack.pop()?;
        if let StackLocation::Index(idx) = state.parent_location {
            let parent = &mut self.stack.get_mut(idx)?.tree;
            parent.attach_subtree(state.side_of_parent, state.tree);
            parent.subtree_mut(state.side_of_parent)?
                .node_ref_mut()
        } else {
            self.tree_state.tree.as_mut()?
                .attach_subtree(state.side_of_parent, state.tree);
            self.tree_state.tree.as_mut()?
                .subtree_mut(state.side_of_parent)?
                .node_ref_mut()
        }
    }

    fn pop_main(&'_ mut self) -> Option<T::NodeRefMut<'_>> {
        self.tree_state.tree.take()?
            .node_ref_mut()
    }

    pub(super) fn pop(&'_ mut self) -> Option<T::NodeRefMut<'_>> {
        if !self.stack.is_empty() {
            self.pop_stack()
        } else {
            self.pop_main()
        }
    }
    
    pub(super) fn expand_left(&mut self) -> bool {
        if self.is_left_expanded() { return false; }

        let parent_location = self.last_index();
        let mut is_expanded = false;
        if let Some(state) = self.stack.last_mut() {
            state.is_left_expanded = true;
            if let Some(left) = state.tree.detach_left() && !left.is_leaf() {
                self.stack.push(StackFrame::new(left, parent_location, Side::Left));
                is_expanded = true;
            }
        } else {
            self.tree_state.is_left_expanded = true;
            if let Some(left) = self.tree_state.tree.as_mut()
                .and_then(|tree| tree.detach_left())
                && !left.is_leaf()
            {
                self.stack.push(StackFrame::new(left, parent_location, Side::Left));
                is_expanded = true;
            }
        }
        is_expanded
    }

    pub(super) fn expand_right(&mut self) -> bool {
        if self.is_right_expanded() { return false; }

        let parent_location = self.last_index();
        let mut is_expanded = false;
        if let Some(state) = self.stack.last_mut() {
            state.is_right_expanded = true;
            if let Some(right) = state.tree.detach_right() && !right.is_leaf() {
                self.stack.push(StackFrame::new(right, parent_location, Side::Right));
                is_expanded = true;
            }
        } else {
            self.tree_state.is_right_expanded = true;
            if let Some(right) = self.tree_state.tree.as_mut()
                .and_then(|tree| tree.detach_right())
                && !right.is_leaf()
            {
                self.stack.push(StackFrame::new(right, parent_location, Side::Right));
                is_expanded = true;
            }
        }
        is_expanded
    }

    pub(super) fn expand_both(&mut self) -> bool {
        if self.is_left_expanded() || self.is_right_expanded() { return false; }

        let parent_location = self.last_index();
        let mut is_expanded = false;
        if let Some(state) = self.stack.last_mut() {
            state.is_left_expanded = true;
            state.is_right_expanded = true;
            match (state.tree.detach_left(), state.tree.detach_right()) {
                (Some(left), Some(right)) => {
                    if !right.is_leaf() {
                        self.stack.push(StackFrame::new(right, parent_location, Side::Right));
                        is_expanded = true;
                    }
                    if !left.is_leaf() {
                        self.stack.push(StackFrame::new(left, parent_location, Side::Left));
                        is_expanded = true;
                    }
                },
                (Some(left), _) => {
                    if !left.is_leaf() {
                        self.stack.push(StackFrame::new(left, parent_location, Side::Left));
                        is_expanded = true;
                    }
                },
                (_, Some(right)) => {
                    if !right.is_leaf() {
                        self.stack.push(StackFrame::new(right, parent_location, Side::Right));
                        is_expanded = true;
                    }
                },
                _ => (),
            }
        } else {
            self.tree_state.is_left_expanded = true;
            self.tree_state.is_right_expanded = true;
            match (
                self.tree_state.tree.as_mut().and_then(|tree| tree.detach_left()),
                self.tree_state.tree.as_mut().and_then(|tree| tree.detach_right())
             ) {
                (Some(left), Some(right)) => {
                    if !right.is_leaf() {
                        self.stack.push(StackFrame::new(right, parent_location, Side::Right));
                        is_expanded = true;
                    }
                    if !left.is_leaf() {
                        self.stack.push(StackFrame::new(left, parent_location, Side::Left));
                        is_expanded = true;
                    }
                },
                (Some(left), _) => {
                    if !left.is_leaf() {
                        self.stack.push(StackFrame::new(left, parent_location, Side::Left));
                        is_expanded = true;
                    }
                },
                (_, Some(right)) => {
                    if !right.is_leaf() {
                        self.stack.push(StackFrame::new(right, parent_location, Side::Right));
                        is_expanded = true;
                    }
                },
                _ => (),
            }
        }
        is_expanded
    }
}
