use lending_iterator::prelude::*;

use crate::binary_trees::{
    Side, binary_tree::{BinaryTree, BinaryTreeNode, Cursor, CursorMut}, traits::{BinaryTree as BinaryTreeTrait, BinaryTreeMut, binary_tree_cursor::{
            BinaryTreeCursor,
            BinaryTreeCursorMut
        }},
};

fn is_cursor_in_valid_node<C, F>(cursor: &C, subtree_filter: F) -> bool
where 
    C: BinaryTreeCursor,
    F: Fn(&C::Node) -> bool,
{
    cursor.node().map_or(false, subtree_filter)
}

/// Moves the given cursor to the next (possibly null) node of the inorder iterator.
/// Assumes the cursor points to the previous element in the iterator.
fn move_cursor_to_next_node<C, F>(cursor: &mut C, subtree_filter: F)
where 
    C: BinaryTreeCursor,
    F: Fn(&C::Node) -> bool,
{
    if cursor.try_move_right() {
        while is_cursor_in_valid_node(cursor, &subtree_filter) && cursor.try_move_left() {}
    } else {
        while cursor.move_up() == Some(Side::Right) {}
    }
}

pub struct InorderIter<'t, T>(InorderIterFiltered<'t, T, fn(&BinaryTreeNode<T>) -> bool>);

pub struct InorderIterFiltered<'t, T, F> {
    tree: &'t BinaryTree<T>,
    cursor: Cursor<'t, T>,
    subtree_filter: F,
    first_iteration: bool,
}

impl<'t, T> InorderIter<'t, T> {
    pub fn new(tree: &'t BinaryTree<T>) -> Self {
        Self(InorderIterFiltered::new(tree, |_| true))
    }
}

impl<'t, T, F> InorderIterFiltered<'t, T, F> {
    pub fn new(tree: &'t BinaryTree<T>, subtree_filter: F) -> Self {
        Self {
            tree,
            cursor: tree.cursor(),
            subtree_filter,
            first_iteration: true,
        }
    }
}

impl<'t, T> Iterator for InorderIter<'t, T> {
    type Item = &'t BinaryTreeNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'t, T, F> Iterator for InorderIterFiltered<'t, T, F>
where 
    F: Fn(&BinaryTreeNode<T>) -> bool,
{
    type Item = &'t BinaryTreeNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_iteration {
            // In the first iteration, move the cursor to the leftmost node that satisfies the filter.
            self.first_iteration = false;
            while is_cursor_in_valid_node(&self.cursor, &self.subtree_filter) && self.cursor.try_move_left() {}
            if !is_cursor_in_valid_node(&self.cursor, &self.subtree_filter) {
                self.cursor.move_up();
            }
            self.tree.node(self.cursor.node_id())
        } else {
            move_cursor_to_next_node(&mut self.cursor, &self.subtree_filter);
            self.tree.node(self.cursor.node_id())
        }
    }
}

pub struct InorderIterMut<'t, T>(InorderIterFilteredMut<'t, T, fn(&BinaryTreeNode<T>) -> bool>);

pub struct InorderIterFilteredMut<'t, T, F> {
    cursor: CursorMut<'t, T>,
    subtree_filter: F,
    first_iteration: bool,
}

impl<'t, T> InorderIterMut<'t, T> {
    pub fn new(tree: &'t mut BinaryTree<T>) -> Self {
        Self(InorderIterFilteredMut::new(tree, |_| true))
    }
}

impl<'t, T, F> InorderIterFilteredMut<'t, T, F> {
    pub fn new(tree: &'t mut BinaryTree<T>, subtree_filter: F) -> Self {
        Self {
            cursor: tree.cursor_mut(),
            subtree_filter,
            first_iteration: true,
        }
    }
}

#[gat]
impl<'t, T> LendingIterator for InorderIterMut<'t, T> {
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut BinaryTreeNode<T>;

    fn next(self: &mut InorderIterMut<'t, T>) -> Option<&mut BinaryTreeNode<T>> {
        self.0.next()
    }
}

#[gat]
impl<'t, T, F> LendingIterator for InorderIterFilteredMut<'t, T, F>
where 
    F: Fn(&BinaryTreeNode<T>) -> bool,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut BinaryTreeNode<T>;

    fn next(self: &mut InorderIterFilteredMut<'t, T, F>) -> Option<&mut BinaryTreeNode<T>> {
        if self.first_iteration {
            // In the first iteration, move the cursor to the leftmost node that satisfies the filter.
            self.first_iteration = false;
            while is_cursor_in_valid_node(&self.cursor, &self.subtree_filter) && self.cursor.try_move_left() {}
            if !is_cursor_in_valid_node(&self.cursor, &self.subtree_filter) {
                self.cursor.move_up();
            }
            self.cursor.node_mut()
        } else {
            move_cursor_to_next_node(&mut self.cursor, &self.subtree_filter);
            self.cursor.node_mut()
        }
    }
}
