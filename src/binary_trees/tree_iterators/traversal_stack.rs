use crate::binary_trees::traits::{BinaryTree, BinaryTreeNode};

struct TreeState<T> {
    tree: T,
    is_left_expanded: bool,
    is_right_expanded: bool,
    is_reported: bool,
}

impl<T> TreeState<T> {
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
    stack: Vec<TreeState<&'tree T>>,
}

impl<'tree, T> TraversalStack<'tree, T>
where 
    T: BinaryTree<Node: BinaryTreeNode<Tree = T>>,
{
    pub(super) fn new(tree: &'tree T) -> Self {
        Self {
            stack: vec![TreeState {
                tree,
                is_left_expanded: false,
                is_right_expanded: false,
                is_reported: false
            }],
        }
    }

    pub(super) fn last_tree(&self) -> Option<&T> {
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

    pub(super) fn report(&mut self) -> Option<&'tree T> {
        let state = self.stack.last_mut()?;
        if !state.is_reported {
            state.is_reported = true;
            Some(state.tree)
        } else {
            None
        }
    }

    pub(super) fn pop(&mut self) -> Option<&'tree T> {
        Some(self.stack.pop()?.tree)
    }
    
    pub(super) fn expand_left(&mut self) -> bool {
        if self.is_left_expanded() { return false; }

        let mut is_expanded = false;
        if let Some(state) = self.stack.last_mut() {
            state.is_left_expanded = true;
            if let Some(left) = state.tree.root().map(|root| root.left_subtree()) 
                && !left.is_leaf()
            {
                self.stack.push(TreeState::new(left));
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
            if let Some(right) = state.tree.root().map(|root| root.right_subtree())
                && !right.is_leaf()
            {
                self.stack.push(TreeState::new(right));
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
            let Some((left, right)) = state.tree
                .root()
                .map(|root| root.subtrees()) else { return false; };
            if !right.is_leaf() {
                self.stack.push(TreeState::new(right));
                is_expanded = true;
            }
            if !left.is_leaf() {
                self.stack.push(TreeState::new(left));
                is_expanded = true;
            }
        }
        is_expanded
    }
}
