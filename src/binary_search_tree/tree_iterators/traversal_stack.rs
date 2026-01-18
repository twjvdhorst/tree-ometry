use crate::binary_search_tree::tree_traits::BinaryTree;

struct StackFrame<T> {
    tree: T,
    is_left_expanded: bool,
    is_right_expanded: bool,
    is_reported: bool,
}

impl<T> StackFrame<T> {
    fn new(tree: T) -> Self {
        Self {
            tree,
            is_left_expanded: false,
            is_right_expanded: false,
            is_reported: false,
        }
    }
}

pub(super) struct TraversalStack<'tree, T>
where 
    T: BinaryTree,
{
    stack: Vec<StackFrame<&'tree T>>,
}

impl<'tree, T> TraversalStack<'tree, T>
where 
    T: BinaryTree,
{
    pub(super) fn new(tree: &'tree T) -> Self {
        Self {
            stack: vec![StackFrame {
                tree,
                is_left_expanded: false,
                is_right_expanded: false,
                is_reported: false
            }],
        }
    }

    pub(super) fn last(&self) -> Option<&T> {
        self.stack.last().map(|state| state.tree)
    }

    pub(super) fn is_left_expanded(&self) -> bool {
        self.stack.last()
            .map(|state| state.is_left_expanded)
            .unwrap_or(true)
    }

    pub(super) fn is_right_expanded(&self) -> bool {
        self.stack.last()
            .map(|state| state.is_right_expanded)
            .unwrap_or(true)
    }

    pub(super) fn is_reported(&self) -> bool {
        self.stack.last()
            .map(|state| state.is_reported)
            .unwrap_or(true)
    }

    pub(super) fn report(&'_ mut self) -> Option<T::NodeRef<'_>> {
        let state = self.stack.last_mut()?;
        if !state.is_reported {
            state.is_reported = true;
            state.tree.node_ref()
        } else {
            None
        }
    }

    pub(super) fn pop(&'_ mut self) -> Option<T::NodeRef<'_>> {
        self.stack.pop()?.tree.node_ref()
    }
    
    pub(super) fn expand_left(&mut self) -> bool {
        if self.is_left_expanded() { return false; }

        let mut is_expanded = false;
        if let Some(state) = self.stack.last_mut() {
            state.is_left_expanded = true;
            if let Some(left) = state.tree.left_subtree() && !left.is_leaf() {
                self.stack.push(StackFrame::new(left));
                is_expanded = true;
            }
        }
        is_expanded
    }

    pub(super) fn expand_right(&mut self) -> bool {
        if self.is_right_expanded() { return false; }

        let mut is_expanded = false;
        if let Some(state) = self.stack.last_mut() {
            state.is_right_expanded = true;
            if let Some(right) = state.tree.right_subtree() && !right.is_leaf() {
                self.stack.push(StackFrame::new(right));
                is_expanded = true;
            }
        }
        is_expanded
    }

    pub(super) fn expand_both(&mut self) -> bool {
        if self.is_left_expanded() || self.is_right_expanded() { return false; }

        let mut is_expanded = false;
        if let Some(state) = self.stack.last_mut() {
            state.is_left_expanded = true;
            state.is_right_expanded = true;
            match (state.tree.left_subtree(), state.tree.right_subtree()) {
                (Some(left), Some(right)) => {
                    if !right.is_leaf() {
                        self.stack.push(StackFrame::new(right));
                        is_expanded = true;
                    }
                    if !left.is_leaf() {
                        self.stack.push(StackFrame::new(left));
                        is_expanded = true;
                    }
                },
                (Some(left), _) => {
                    if !left.is_leaf() {
                        self.stack.push(StackFrame::new(left));
                        is_expanded = true;
                    }
                },
                (_, Some(right)) => {
                    if !right.is_leaf() {
                        self.stack.push(StackFrame::new(right));
                        is_expanded = true;
                    }
                },
                _ => (),
            }
        }
        is_expanded
    }
}
