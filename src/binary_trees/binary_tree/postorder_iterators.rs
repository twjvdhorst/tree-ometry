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
    binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursor,
        PeekingCursorMut,
    },
};

impl<T> BinaryTree<T> {
    pub fn postorder_iter(&self) -> PostorderIter<'_, T> {
        PostorderIter::new(self)
    }

    pub fn postorder_iter_filtered<P>(&self, subtree_filter: P) -> PostorderIterFiltered<'_, T, P>
    where 
        P: Fn(&T) -> bool,
    {
        PostorderIterFiltered::new(self, subtree_filter)
    }

    pub fn postorder_iter_mut(&mut self) -> PostorderIterMut<'_, T> {
        PostorderIterMut::new(self)
    }

    pub fn postorder_iter_filtered_mut<P>(&mut self, subtree_filter: P) -> PostorderIterFilteredMut<'_, T, P>
    where 
        P: Fn(&T) -> bool,
    {
        PostorderIterFilteredMut::new(self, subtree_filter)
    }

    pub fn into_postorder_iter(self) -> IntoPostorderIter<T> {
        IntoPostorderIter::new(self)
    }

    pub fn into_postorder_iter_filtered<P>(self, subtree_filter: P) -> IntoPostorderIterFiltered<T, P>
    where 
        P: Fn(&T) -> bool,
    {
        IntoPostorderIterFiltered::new(self, subtree_filter)
    }
}

pub struct PostorderIter<'t, T>(PostorderIterFiltered<'t, T, fn(&T) -> bool>);

pub struct PostorderIterFiltered<'t, T, P> {
    cursor: Cursor<'t, T>,
    subtree_filter: P,
    first_iteration: bool,
}

impl<'t, T> PostorderIter<'t, T> {
    fn new(tree: &'t BinaryTree<T>) -> Self {
        Self(PostorderIterFiltered::new(tree, |_| true))
    }
}

impl<'t, T, P> PostorderIterFiltered<'t, T, P> 
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

    fn is_cursor_in_valid_node(&self) -> bool {
        self.cursor.get().map_or(false, &self.subtree_filter)
    }

    fn move_cursor_to_successor_in_subtree(&mut self) {
        if self.cursor.try_move_right() {
            while self.is_cursor_in_valid_node() && self.cursor.try_move_left() {}
        }
    }

    /// Moves the given cursor to the next (possibly null) node of the postorder iterator.
    /// Assumes the cursor points to the previous element in the iterator.
    fn move_cursor_to_next_node(&mut self) {
        if self.cursor.move_up() == Some(Side::Left) {
            // Explore the right subtree of the node's parent (which the cursor points to now).
            self.move_cursor_to_successor_in_subtree();
        }
    }
}

impl<'t, T> Iterator for PostorderIter<'t, T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'t, T, P> Iterator for PostorderIterFiltered<'t, T, P>
where 
    P: Fn(&T) -> bool,
{
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_iteration {
            // In the first iteration, move the cursor to the leftmost node that satisfies the filter.
            self.first_iteration = false;
            while self.is_cursor_in_valid_node() && self.cursor.try_move_left() {}
            if !self.is_cursor_in_valid_node() {
                self.cursor.move_up();
            }
            self.cursor.get()
        } else {
            self.move_cursor_to_next_node();
            self.cursor.get()
        }
    }
}

pub struct PostorderIterMut<'t, T>(PostorderIterFilteredMut<'t, T, fn(&T) -> bool>);

pub struct PostorderIterFilteredMut<'t, T, P> {
    cursor: CursorMut<'t, T>,
    subtree_filter: P,
    first_iteration: bool,
}

impl<'t, T> PostorderIterMut<'t, T> {
    fn new(tree: &'t mut BinaryTree<T>) -> Self {
        Self(PostorderIterFilteredMut::new(tree, |_| true))
    }
}

impl<'t, T, P> PostorderIterFilteredMut<'t, T, P>
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

    fn is_cursor_in_valid_node(&self) -> bool {
        self.cursor.get().map_or(false, &self.subtree_filter)
    }

    fn move_cursor_to_successor_in_subtree(&mut self) {
        if self.cursor.try_move_right() {
            while self.is_cursor_in_valid_node() && self.cursor.try_move_left() {}
        }
    }

    /// Moves the given cursor to the next (possibly null) node of the postorder iterator.
    /// Assumes the cursor points to the previous element in the iterator.
    fn move_cursor_to_next_node(&mut self) {
        if self.cursor.move_up() == Some(Side::Left) {
            // Explore the right subtree of the node's parent (which the cursor points to now).
            self.move_cursor_to_successor_in_subtree();
        }
    }
}

#[gat]
impl<'t, T> LendingIterator for PostorderIterMut<'t, T> {
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut T;

    fn next(self: &mut PostorderIterMut<'t, T>) -> Option<&mut T> {
        self.0.next()
    }
}

#[gat]
impl<'t, T, P> LendingIterator for PostorderIterFilteredMut<'t, T, P>
where 
    P: Fn(&T) -> bool,
{
    type Item<'next>
    where 
        Self: 'next,
        = &'next mut T;

    fn next(self: &mut PostorderIterFilteredMut<'t, T, P>) -> Option<&mut T> {
        if self.first_iteration {
            // In the first iteration, move the cursor to the leftmost node that satisfies the filter.
            self.first_iteration = false;
            while self.is_cursor_in_valid_node() && self.cursor.try_move_left() {}
            if !self.is_cursor_in_valid_node() {
                self.cursor.move_up();
            }
            self.cursor.get_mut()
        } else {
            self.move_cursor_to_next_node();
            self.cursor.get_mut()
        }
    }
}

pub struct IntoPostorderIter<T>(IntoPostorderIterFiltered<T, fn(&T) -> bool>);

pub struct IntoPostorderIterFiltered<T, P> {
    tree: BinaryTree<T>,
    subtree_filter: P,
    stack: Vec<NodeId>,
}

impl<T> IntoPostorderIter<T> {
    fn new(tree: BinaryTree<T>) -> Self {
        Self(IntoPostorderIterFiltered::new(tree, |_| true))
    }
}

impl<T, P> IntoPostorderIterFiltered<T, P> 
where 
    P: Fn(&T) -> bool,
{
    fn new(tree: BinaryTree<T>, subtree_filter: P) -> Self {
        // Move the "cursor" to the first node in the inorder order.
        let mut id = tree.root_id();
        let mut stack = Vec::new();
        while tree.node(id).map(|node| subtree_filter(node.data())).unwrap_or(false) {
            stack.push(id);
            id = tree.left_id(id).unwrap_or(NodeId::null());
        }

        Self {
            tree,
            subtree_filter,
            stack,
        }
    }

    fn is_id_valid(&self, id: NodeId) -> bool {
        self.tree.node(id)
            .map(|node| (self.subtree_filter)(node.data()))
            .unwrap_or(false)
    }
}

impl<T> Iterator for IntoPostorderIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<T, P> Iterator for IntoPostorderIterFiltered<T, P>
where 
    P: Fn(&T) -> bool,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Expand the stack first, then report the element.
        let id = self.stack.last()?;
        if let Some(right_id) = self.tree.right_id(*id) && self.is_id_valid(right_id) {
            self.stack.push(right_id);
            while let Some(left_id) = self.tree.left_id(*self.stack.last().unwrap()) && self.is_id_valid(left_id) {
                self.stack.push(left_id);
            }
        }
        let next_id = self.stack.pop().unwrap();
        self.tree.remove_node(next_id)
    }
}
