use crate::binary_trees::{
    Side, 
    binary_tree_traits::{
        BinaryTree, 
        BinaryTreeNodeMut
    }
};

enum Tree<'tree, T> {
    MutRef(&'tree mut T),
    Detached(T, usize, Side), // Detached subtrees have a stackindex of their parent, as well as the side of the parent they were detached from
}

struct TreeState<'tree, T> {
    tree: Tree<'tree, T>,
    is_left_expanded: bool,
    is_right_expanded: bool,
    is_reported: bool,
}

impl<'tree, T> TreeState<'tree, T> {
    fn new_mut(tree: &'tree mut T) -> Self {
        Self {
            tree: Tree::MutRef(tree),
            is_left_expanded: false,
            is_right_expanded: false,
            is_reported: false,
        }
    }

    fn new_detached(tree: T, parent_idx: usize, side_of_parent: Side) -> Self {
        Self {
            tree: Tree::Detached(tree, parent_idx, side_of_parent),
            is_left_expanded: false,
            is_right_expanded: false,
            is_reported: false,
        }
    }

    fn tree(&self) -> &T {
        match &self.tree {
            Tree::MutRef(tree) => tree,
            Tree::Detached(tree, _, _) => tree,
        }
    }
    
    fn tree_mut(&mut self) -> &mut T {
        match &mut self.tree {
            Tree::MutRef(tree) => tree,
            Tree::Detached(tree, _, _) => tree,
        }
    }
}

pub(super) struct TraversalStackMut<'tree, T>
where 
    T: BinaryTree<Node: BinaryTreeNodeMut<Tree = T>>,
{
    stack: Vec<TreeState<'tree, T>>,
}

/// Custom Drop implementation that rebuilds the tree by popping the stack.
impl<'tree, T> Drop for TraversalStackMut<'tree, T>
where 
    T: BinaryTree<Node: BinaryTreeNodeMut<Tree = T>>,
{
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<'tree, T> TraversalStackMut<'tree, T>
where 
    T: BinaryTree<Node: BinaryTreeNodeMut<Tree = T>>,
{
    pub(super) fn new(tree: &'tree mut T) -> Self {
        Self {
            stack: vec![TreeState::new_mut(tree)],
        }
    }

    pub(super) fn last_tree(&self) -> Option<&T> {
        self.stack.last()
            .map(|state| state.tree())
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

    pub(super) fn report(&'_ mut self) -> Option<&'_ mut T> {
        let state = self.stack.last_mut()?;
        if !state.is_reported {
            state.is_reported = true;
            Some(state.tree_mut())
        } else {
            None
        }
    }

    pub(super) fn pop(&'_ mut self) -> Option<&'_ mut T> {
        let state = self.stack.pop()?;
        match state.tree {
            Tree::MutRef(subtree) => Some(subtree), // Subtree is still attached to parent (or is root), no need to reattach
            Tree::Detached(subtree, parent_idx, side_of_parent) => { // Subtree is detached from parent. Reattach before reporting
                let parent = self.stack.get_mut(parent_idx)?
                    .tree_mut()
                    .root_mut()?;
                parent.attach_subtree(side_of_parent, subtree);
                Some(parent.subtree_mut(side_of_parent))
            },
        }
    }
    
    pub(super) fn expand_left(&mut self) -> bool {
        if self.is_left_expanded() { return false; }

        let parent_idx = self.stack.len() - 1;
        let mut is_expanded = false;
        if let Some(state) = self.stack.last_mut() {
            state.is_left_expanded = true;
            if let Some(left) = state.tree_mut().root_mut().map(|root| root.detach_left()) 
                && !left.is_leaf()
            {
                self.stack.push(TreeState::new_detached(left, parent_idx, Side::Left));
                is_expanded = true;
            }
        }
        is_expanded
    }

    pub(super) fn expand_right(&mut self) -> bool {
        if self.is_right_expanded() { return false; }

        let parent_idx = self.stack.len() - 1;
        let mut is_expanded = false;
        if let Some(state) = self.stack.last_mut() {
            state.is_right_expanded = true;
            if let Some(right) = state.tree_mut().root_mut().map(|root| root.detach_right()) 
                && !right.is_leaf()
            {
                self.stack.push(TreeState::new_detached(right, parent_idx, Side::Right));
                is_expanded = true;
            }
        }
        is_expanded
    }

    pub(super) fn expand_both(&mut self) -> bool {
        if self.is_left_expanded() || self.is_right_expanded() { return false; }

        let parent_idx = self.stack.len() - 1;
        let mut is_expanded = false;
        if let Some(state) = self.stack.last_mut() {
            state.is_left_expanded = true;
            state.is_right_expanded = true;
            let Some((left, right)) = state.tree_mut()
                .root_mut()
                .map(|root| root.detach_both()) else { return false; };
            if !right.is_leaf() {
                self.stack.push(TreeState::new_detached(right, parent_idx, Side::Right));
                is_expanded = true;
            }
            if !left.is_leaf() {
                self.stack.push(TreeState::new_detached(left, parent_idx, Side::Left));
                is_expanded = true;
            }
        }
        is_expanded
    }
}
