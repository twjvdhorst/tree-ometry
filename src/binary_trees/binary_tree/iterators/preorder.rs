use lending_iterator::prelude::*;

use crate::binary_trees::{
    Side,
    binary_tree::{BinaryTree, BinaryTreeNode, Cursor, CursorMut},
    traits::{
        BinaryTree as BinaryTreeTrait, 
        BinaryTreeMut,
        binary_tree_cursor::{
            BinaryTreeCursor,
            BinaryTreeCursorMut
        }
    },
};

fn move_cursor_left_if_valid<C, P>(cursor: &mut C, subtree_filter: P) -> bool
where 
    C: BinaryTreeCursor,
    P: Fn(&C::Node) -> bool,
{
    if !cursor.try_move_left() {
        return false;
    }

    if !(subtree_filter)(cursor.node().unwrap()) {
        cursor.move_up();
        false
    } else {
        true
    }
}

fn move_cursor_right_if_valid<C, P>(cursor: &mut C, subtree_filter: P) -> bool
where 
    C: BinaryTreeCursor,
    P: Fn(&C::Node) -> bool,
{
    if !cursor.try_move_right() {
        return false;
    }

    if !(subtree_filter)(cursor.node().unwrap()) {
        cursor.move_up();
        false
    } else {
        true
    }
}

/// Moves the given cursor to the next (possibly null) node of the preorder iterator.
/// Assumes the cursor points to the previous element in the iterator.
fn move_cursor_to_next_node<C, P>(cursor: &mut C, subtree_filter: P)
where 
    C: BinaryTreeCursor,
    P: Fn(&C::Node) -> bool,
{
    if move_cursor_left_if_valid(cursor, &subtree_filter) {
        return;
    }
    
    if move_cursor_right_if_valid(cursor, &subtree_filter) {
        return;
    }
    
    while let Some(side) = cursor.move_up() {
        if side == Side::Left && move_cursor_right_if_valid(cursor, &subtree_filter) {
            return;
        }
    }
}

pub struct PreorderIter<'t, T>(PreorderIterFiltered<'t, T, fn(&BinaryTreeNode<T>) -> bool>);

pub struct PreorderIterFiltered<'t, T, P> {
    tree: &'t BinaryTree<T>,
    cursor: Cursor<'t, T>,
    subtree_filter: P,
    first_iteration: bool,
}

impl<'t, T> PreorderIter<'t, T> {
    pub fn new(tree: &'t BinaryTree<T>) -> Self {
        Self(PreorderIterFiltered::new(tree, |_| true))
    }
}

impl<'t, T, P> PreorderIterFiltered<'t, T, P> {
    pub fn new(tree: &'t BinaryTree<T>, subtree_filter: P) -> Self {
        Self {
            tree,
            cursor: tree.cursor(),
            subtree_filter,
            first_iteration: true,
        }
    }
}

impl<'t, T> Iterator for PreorderIter<'t, T> {
    type Item = &'t BinaryTreeNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'t, T, P> Iterator for PreorderIterFiltered<'t, T, P>
where 
    P: Fn(&BinaryTreeNode<T>) -> bool,
{
    type Item = &'t BinaryTreeNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_iteration {
            // In the first iteration, simply report the root node pointed at by the cursor.
            self.first_iteration = false;
            let node = self.cursor.node()?;
            if (self.subtree_filter)(node) {
                self.tree.node(self.cursor.node_id())
            } else {
                None
            }
        } else {
            move_cursor_to_next_node(&mut self.cursor, &self.subtree_filter);
            self.tree.node(self.cursor.node_id())
        }
    }
}

pub struct PreorderIterMut<'t, T>(PreorderIterFilteredMut<'t, T, fn(&BinaryTreeNode<T>) -> bool>);

pub struct PreorderIterFilteredMut<'t, T, P> {
    cursor: CursorMut<'t, T>,
    subtree_filter: P,
    first_iteration: bool,
}

impl<'t, T> PreorderIterMut<'t, T> {
    pub fn new(tree: &'t mut BinaryTree<T>) -> Self {
        Self(PreorderIterFilteredMut::new(tree, |_| true))
    }
}

impl<'t, T, P> PreorderIterFilteredMut<'t, T, P> {
    pub fn new(tree: &'t mut BinaryTree<T>, subtree_filter: P) -> Self {
        Self {
            cursor: tree.cursor_mut(),
            subtree_filter,
            first_iteration: true,
        }
    }
}

#[gat]
impl<'t, T> LendingIterator for PreorderIterMut<'t, T> {
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut BinaryTreeNode<T>;

    fn next(self: &mut PreorderIterMut<'t, T>) -> Option<&mut BinaryTreeNode<T>> {
        self.0.next()
    }
}

#[gat]
impl<'t, T, P> LendingIterator for PreorderIterFilteredMut<'t, T, P>
where 
    P: Fn(&BinaryTreeNode<T>) -> bool,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut BinaryTreeNode<T>;

    fn next(self: &mut PreorderIterFilteredMut<'t, T, P>) -> Option<&mut BinaryTreeNode<T>> {
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
            move_cursor_to_next_node(&mut self.cursor, &self.subtree_filter);
            self.cursor.node_mut()
        }
    }
}
