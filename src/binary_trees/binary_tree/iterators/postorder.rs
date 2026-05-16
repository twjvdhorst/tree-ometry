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

fn is_cursor_in_valid_node<C, P>(cursor: &C, subtree_filter: P) -> bool
where 
    C: BinaryTreeCursor,
    P: Fn(&C::Node) -> bool,
{
    cursor.node().map_or(false, subtree_filter)
}

fn move_cursor_to_successor_in_subtree<C, P>(cursor: &mut C, subtree_filter: P)
where 
    C: BinaryTreeCursor,
    P: Fn(&C::Node) -> bool,
{
    if cursor.try_move_right() {
        while is_cursor_in_valid_node(cursor, &subtree_filter) && cursor.try_move_left() {}
    }
}

/// Moves the given cursor to the next (possibly null) node of the postorder iterator.
/// Assumes the cursor points to the previous element in the iterator.
fn move_cursor_to_next_node<C, P>(cursor: &mut C, subtree_filter: P)
where 
    C: BinaryTreeCursor,
    P: Fn(&C::Node) -> bool,
{
    if cursor.move_up() == Some(Side::Left) {
        // Explore the right subtree of the node's parent (which the cursor points to now).
        move_cursor_to_successor_in_subtree(cursor, subtree_filter);
    }
}

pub struct PostorderIter<'t, T>(PostorderIterFiltered<'t, T, fn(&BinaryTreeNode<T>) -> bool>);

pub struct PostorderIterFiltered<'t, T, P> {
    tree: &'t BinaryTree<T>,
    cursor: Cursor<'t, T>,
    subtree_filter: P,
    first_iteration: bool,
}

impl<'t, T> PostorderIter<'t, T> {
    pub fn new(tree: &'t BinaryTree<T>) -> Self {
        Self(PostorderIterFiltered::new(tree, |_| true))
    }
}

impl<'t, T, P> PostorderIterFiltered<'t, T, P> {
    pub fn new(tree: &'t BinaryTree<T>, subtree_filter: P) -> Self {
        Self {
            tree,
            cursor: tree.cursor(),
            subtree_filter,
            first_iteration: true,
        }
    }
}

impl<'t, T> Iterator for PostorderIter<'t, T> {
    type Item = &'t BinaryTreeNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'t, T, P> Iterator for PostorderIterFiltered<'t, T, P>
where 
    P: Fn(&BinaryTreeNode<T>) -> bool,
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

pub struct PostorderIterMut<'t, T>(PostorderIterFilteredMut<'t, T, fn(&BinaryTreeNode<T>) -> bool>);

pub struct PostorderIterFilteredMut<'t, T, P> {
    cursor: CursorMut<'t, T>,
    subtree_filter: P,
    first_iteration: bool,
}

impl<'t, T> PostorderIterMut<'t, T> {
    pub fn new(tree: &'t mut BinaryTree<T>) -> Self {
        Self(PostorderIterFilteredMut::new(tree, |_| true))
    }
}

impl<'t, T, P> PostorderIterFilteredMut<'t, T, P> {
    pub fn new(tree: &'t mut BinaryTree<T>, subtree_filter: P) -> Self {
        Self {
            cursor: tree.cursor_mut(),
            subtree_filter,
            first_iteration: true,
        }
    }
}

#[gat]
impl<'t, T> LendingIterator for PostorderIterMut<'t, T> {
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut BinaryTreeNode<T>;

    fn next(self: &mut PostorderIterMut<'t, T>) -> Option<&mut BinaryTreeNode<T>> {
        self.0.next()
    }
}

#[gat]
impl<'t, T, P> LendingIterator for PostorderIterFilteredMut<'t, T, P>
where 
    P: Fn(&BinaryTreeNode<T>) -> bool,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut BinaryTreeNode<T>;

    fn next(self: &mut PostorderIterFilteredMut<'t, T, P>) -> Option<&mut BinaryTreeNode<T>> {
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
