use lending_iterator::prelude::*;
use slotmap::Key;

use crate::binary_trees::{
    Side, 
    binary_tree::{
        BinaryTree,
        Cursor,
        CursorMut,
        NodeId,
    },
    traits::{
        BinaryTree as BinaryTreeTrait,
        BinaryTreeMut, 
        binary_tree_cursor::{
            BinaryTreeCursor,
            PeekingCursor,
            PeekingCursorMut,
        },
    }
};

impl<T> BinaryTree<T> {
    pub fn preorder_iter(&self) -> PreorderIter<'_, T> {
        PreorderIter::new(self)
    }

    pub fn preorder_iter_filtered<P>(&self, subtree_filter: P) -> PreorderIterFiltered<'_, T, P>
    where 
        P: Fn(&T) -> bool,
    {
        PreorderIterFiltered::new(self, subtree_filter)
    }

    pub fn preorder_iter_mut(&mut self) -> PreorderIterMut<'_, T> {
        PreorderIterMut::new(self)
    }

    pub fn preorder_iter_filtered_mut<P>(&mut self, subtree_filter: P) -> PreorderIterFilteredMut<'_, T, P>
    where 
        P: Fn(&T) -> bool,
    {
        PreorderIterFilteredMut::new(self, subtree_filter)
    }

    pub fn into_preorder_iter(self) -> IntoPreorderIter<T> {
        IntoPreorderIter::new(self)
    }

    /*
    pub fn into_preorder_iter_filtered<P>(&self, subtree_filter: P) -> IntoPreorderIterFiltered<T, P>
    where 
        P: Fn(&T) -> bool,
    {
        IntoPreorderIterFiltered::new(self, subtree_filter)
    }
    */
}

pub struct PreorderIter<'t, T>(PreorderIterFiltered<'t, T, fn(&T) -> bool>);

pub struct PreorderIterFiltered<'t, T, P> {
    cursor: Cursor<'t, T>,
    subtree_filter: P,
    first_iteration: bool,
}

impl<'t, T> PreorderIter<'t, T> {
    fn new(tree: &'t BinaryTree<T>) -> Self {
        Self(PreorderIterFiltered::new(tree, |_| true))
    }
}

impl<'t, T, P> PreorderIterFiltered<'t, T, P> 
where 
    P: Fn(&T) -> bool,
{
    fn new(tree: &'t BinaryTree<T>, subtree_filter: P) -> Self {
        Self {
            cursor: tree.cursor(),
            subtree_filter,
            first_iteration: true,
        }
    }

    fn move_cursor_left_if_valid(&mut self) -> bool {
        if !self.cursor.try_move_left() {
            return false;
        }

        if !(self.subtree_filter)(self.cursor.get().unwrap()) {
            self.cursor.move_up();
            false
        } else {
            true
        }
    }

    fn move_cursor_right_if_valid(&mut self) -> bool {
        if !self.cursor.try_move_right() {
            return false;
        }

        if !(self.subtree_filter)(self.cursor.get().unwrap()) {
            self.cursor.move_up();
            false
        } else {
            true
        }
    }

    /// Moves the given cursor to the next (possibly null) node of the preorder iterator.
    /// Assumes the cursor points to the previous element in the iterator.
    fn move_cursor_to_next_node(&mut self) {
        if self.move_cursor_left_if_valid() {
            return;
        }
        
        if self.move_cursor_right_if_valid() {
            return;
        }
        
        while let Some(side) = self.cursor.move_up() {
            if side == Side::Left && self.move_cursor_right_if_valid() {
                return;
            }
        }
    }
}

impl<'t, T> Iterator for PreorderIter<'t, T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'t, T, P> Iterator for PreorderIterFiltered<'t, T, P>
where 
    P: Fn(&T) -> bool,
{
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_iteration {
            // In the first iteration, simply report the root node pointed at by the cursor.
            self.first_iteration = false;
            let node = self.cursor.get()?;
            if (self.subtree_filter)(node) {
                self.cursor.get()
            } else {
                None
            }
        } else {
            self.move_cursor_to_next_node();
            self.cursor.get()
        }
    }
}

pub struct PreorderIterMut<'t, T>(PreorderIterFilteredMut<'t, T, fn(&T) -> bool>);

pub struct PreorderIterFilteredMut<'t, T, P> {
    cursor: CursorMut<'t, T>,
    subtree_filter: P,
    first_iteration: bool,
}

impl<'t, T> PreorderIterMut<'t, T> {
    fn new(tree: &'t mut BinaryTree<T>) -> Self {
        Self(PreorderIterFilteredMut::new(tree, |_| true))
    }
}

impl<'t, T, P> PreorderIterFilteredMut<'t, T, P>
where 
    P: Fn(&T) -> bool,
{
    fn new(tree: &'t mut BinaryTree<T>, subtree_filter: P) -> Self {
        Self {
            cursor: tree.cursor_mut(),
            subtree_filter,
            first_iteration: true,
        }
    }

    fn move_cursor_left_if_valid(&mut self) -> bool {
        if !self.cursor.try_move_left() {
            return false;
        }

        if !(self.subtree_filter)(self.cursor.get().unwrap()) {
            self.cursor.move_up();
            false
        } else {
            true
        }
    }

    fn move_cursor_right_if_valid(&mut self) -> bool {
        if !self.cursor.try_move_right() {
            return false;
        }

        if !(self.subtree_filter)(self.cursor.get().unwrap()) {
            self.cursor.move_up();
            false
        } else {
            true
        }
    }

    /// Moves the given cursor to the next (possibly null) node of the preorder iterator.
    /// Assumes the cursor points to the previous element in the iterator.
    fn move_cursor_to_next_node(&mut self) {
        if self.move_cursor_left_if_valid() {
            return;
        }
        
        if self.move_cursor_right_if_valid() {
            return;
        }
        
        while let Some(side) = self.cursor.move_up() {
            if side == Side::Left && self.move_cursor_right_if_valid() {
                return;
            }
        }
    }
}

#[gat]
impl<'t, T> LendingIterator for PreorderIterMut<'t, T> {
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut T;

    fn next(self: &mut PreorderIterMut<'t, T>) -> Option<&mut T> {
        self.0.next()
    }
}

#[gat]
impl<'t, T, P> LendingIterator for PreorderIterFilteredMut<'t, T, P>
where 
    P: Fn(&T) -> bool,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut T;

    fn next(self: &mut PreorderIterFilteredMut<'t, T, P>) -> Option<&mut T> {
        if self.first_iteration {
            // In the first iteration, simply report the root node pointed at by the cursor.
            self.first_iteration = false;
            let node = self.cursor.get_mut()?;
            if (self.subtree_filter)(node) {
                Some(node)
            } else {
                None
            }
        } else {
            self.move_cursor_to_next_node();
            self.cursor.get_mut()
        }
    }
}

pub struct IntoPreorderIter<T> {
    tree: BinaryTree<T>,
    stack: Vec<NodeId>,
}

impl<T> IntoPreorderIter<T> {
    fn new(tree: BinaryTree<T>) -> Self {
        let id = tree.root_id();
        let mut stack = Vec::new();
        if !id.is_null() {
            stack.push(id);
        }

        Self {
            tree,
            stack,
        }
    }
}

impl<T> Iterator for IntoPreorderIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Get id of the to-be-reported element, and expand stack.
        let next_id = self.stack.pop()?;
        if let Some(right_id) = self.tree.right_id(next_id) {
            self.stack.push(right_id);
        }
        if let Some(left_id) = self.tree.left_id(next_id) {
            self.stack.push(left_id);
        }
        self.tree.remove_node(next_id)
    }
}
