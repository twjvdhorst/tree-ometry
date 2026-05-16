use lending_iterator::prelude::*;

use crate::binary_trees::{
    Side, 
    traits::{
        BinaryTree, 
        BinaryTreeMut, 
        binary_tree_cursor::{
            BinaryTreeCursor,
            PeekingCursor,
            PeekingCursorMut,
        },
    },
};

pub struct PreorderIter<'t, T>(PreorderIterFiltered<'t, T, fn(&T::Node) -> bool>)
where T: BinaryTree + ?Sized + 't;

pub struct PreorderIterFiltered<'t, T, P>
where 
    T: BinaryTree + ?Sized + 't,
    P: Fn(&T::Node) -> bool,
{
    cursor: T::Cursor<'t>,
    subtree_filter: P,
    first_iteration: bool,
}

impl<'t, T> PreorderIter<'t, T>
where 
    T: BinaryTree + ?Sized + 't,
{
    pub fn new(tree: &'t T) -> Self {
        Self(PreorderIterFiltered::new(tree, |_| true))
    }
}

impl<'t, T, P> PreorderIterFiltered<'t, T, P> 
where 
    T: BinaryTree + ?Sized + 't,
    P: Fn(&T::Node) -> bool,
{
    pub fn new(tree: &'t T, subtree_filter: P) -> Self {
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

        if !(self.subtree_filter)(self.cursor.node().unwrap()) {
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

        if !(self.subtree_filter)(self.cursor.node().unwrap()) {
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

impl<'t, T> Iterator for PreorderIter<'t, T> 
where 
    T: BinaryTree + ?Sized + 't,
{
    type Item = &'t T::Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'t, T, P> Iterator for PreorderIterFiltered<'t, T, P>
where 
    T: BinaryTree + ?Sized + 't,
    P: Fn(&T::Node) -> bool,
{
    type Item = &'t T::Node;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_iteration {
            // In the first iteration, simply report the root node pointed at by the cursor.
            self.first_iteration = false;
            let node = self.cursor.node()?;
            if (self.subtree_filter)(node) {
                self.cursor.node()
            } else {
                None
            }
        } else {
            self.move_cursor_to_next_node();
            self.cursor.node()
        }
    }
}

pub struct PreorderIterMut<'t, T>(PreorderIterFilteredMut<'t, T, fn(&T::Node) -> bool>)
where T: BinaryTreeMut + ?Sized + 't;

pub struct PreorderIterFilteredMut<'t, T, P>
where 
    T: BinaryTreeMut + ?Sized + 't,
    P: Fn(&T::Node) -> bool,
{
    cursor: T::CursorMut<'t>,
    subtree_filter: P,
    first_iteration: bool,
}

impl<'t, T> PreorderIterMut<'t, T>
where 
    T: BinaryTreeMut + ?Sized + 't,
{
    pub fn new(tree: &'t mut T) -> Self {
        Self(PreorderIterFilteredMut::new(tree, |_| true))
    }
}

impl<'t, T, P> PreorderIterFilteredMut<'t, T, P>
where 
    T: BinaryTreeMut + ?Sized + 't,
    P: Fn(&T::Node) -> bool,
{
    pub fn new(tree: &'t mut T, subtree_filter: P) -> Self {
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

        if !(self.subtree_filter)(self.cursor.node().unwrap()) {
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

        if !(self.subtree_filter)(self.cursor.node().unwrap()) {
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
impl<'t, T> LendingIterator for PreorderIterMut<'t, T>
where 
    T: BinaryTreeMut + ?Sized + 't,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut T::Node;

    fn next(self: &mut PreorderIterMut<'t, T>) -> Option<&mut T::Node> {
        self.0.next()
    }
}

#[gat]
impl<'t, T, P> LendingIterator for PreorderIterFilteredMut<'t, T, P>
where 
    T: BinaryTreeMut + ?Sized + 't,
    P: Fn(&T::Node) -> bool,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut T::Node;

    fn next(self: &mut PreorderIterFilteredMut<'t, T, P>) -> Option<&mut T::Node> {
        if self.first_iteration {
            // In the first iteration, simply report the root node pointed at by the cursor.
            self.first_iteration = false;
            let node = self.cursor.node_mut()?;
            if (self.subtree_filter)(node) {
                Some(node)
            } else {
                None
            }
        } else {
            self.move_cursor_to_next_node();
            self.cursor.node_mut()
        }
    }
}
