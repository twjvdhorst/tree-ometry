use std::{iter::FusedIterator, mem};

use slotmap::Key;

use crate::binary_trees::{
    Side,
    binary_tree::{
        BinaryTree,
        Cursor,
        CursorMut,
        NodeId,
    }, binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursor, PeekingCursorMut,
    }
};

impl<T> BinaryTree<T> {
    pub fn inorder_iter(&self) -> InorderIter<'_, T> {
        InorderIter::new(self)
    }

    pub fn inorder_iter_mut(&mut self) -> InorderIterMut<'_, T> {
        InorderIterMut::new(self)
    }

    pub fn into_inorder_iter(self) -> IntoInorderIter<T> {
        IntoInorderIter::new(self)
    }
}

impl<'t, T> Cursor<'t, T> {
    pub fn inorder_subtree_iter(self) -> InorderSubtreeIter<'t, T> {
        InorderSubtreeIter::new(self)
    }
}

impl<'t, T> CursorMut<'t, T> {
    pub fn inorder_subtree_iter(self) -> InorderSubtreeIter<'t, T> {
        InorderSubtreeIter::new(self.into())
    }

    pub fn inorder_subtree_iter_mut(self) -> InorderSubtreeIterMut<'t, T> {
        InorderSubtreeIterMut::new(self)
    }

    pub fn drain_subtree_inorder(self) -> DrainSubtreeInorder<'t, T> {
        DrainSubtreeInorder::new(self)
    }
}

macro_rules! inc_depth {
    ($self: ident, true) => {
        $self.current_depth += 1;
    };
    ($self: ident, false) => {};
}

macro_rules! dec_depth {
    ($self: ident, true) => {
        $self.current_depth = $self.current_depth.checked_sub(1)?;
    };
    ($self: ident, false) => {};
}

macro_rules! move_next_cursor {
    ($self: ident, $track_depth: tt) => {{
        if $self.first_iteration {
            while $self.cursor.try_move_left() {
                inc_depth!($self, $track_depth);
            }
            $self.first_iteration = false;
        } else if $self.cursor.try_move_right() {
            inc_depth!($self, $track_depth);
            while $self.cursor.try_move_left() {
                inc_depth!($self, $track_depth);
            }
        } else {
            while let Some(side) = $self.cursor.move_up() {
                dec_depth!($self, $track_depth); // If depth becomes negative, we return None.
                if side == Side::Left {
                    break;
                }
            }
        }
    }};
}

pub struct InorderIter<'t, T> {
    cursor: Cursor<'t, T>,
    first_iteration: bool,
}

pub struct InorderSubtreeIter<'t, T> {
    cursor: Cursor<'t, T>,
    current_depth: usize,
    first_iteration: bool,
}

impl<'t, T> InorderIter<'t, T> {
    fn new(tree: &'t BinaryTree<T>) -> Self {
        Self {
            cursor: tree.cursor(),
            first_iteration: true,
        }
    }
}

impl<'t, T> InorderSubtreeIter<'t, T> {
    fn new(cursor: Cursor<'t, T>) -> Self {
        Self {
            cursor,
            current_depth: 0,
            first_iteration: true,
        }
    }
}

impl<'t, T> Iterator for InorderIter<'t, T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        move_next_cursor!(self, false);
        self.cursor.get()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.cursor.tree().len(), Some(self.cursor.tree().len()))
    }
}

impl<'t, T> FusedIterator for InorderIter<'t, T> {}
impl<'t, T> ExactSizeIterator for InorderIter<'t, T> {
    fn len(&self) -> usize {
        self.cursor.tree().len()
    }
}

impl<'t, T> Iterator for InorderSubtreeIter<'t, T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        move_next_cursor!(self, true);
        self.cursor.get()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.cursor.tree().len()))
    }
}

impl<'t, T> FusedIterator for InorderSubtreeIter<'t, T> {}

pub struct InorderIterMut<'t, T> {
    cursor: CursorMut<'t, T>,
    first_iteration: bool,
}

pub struct InorderSubtreeIterMut<'t, T> {
    cursor: CursorMut<'t, T>,
    current_depth: usize,
    first_iteration: bool,
}

impl<'t, T> InorderIterMut<'t, T> {
    fn new(tree: &'t mut BinaryTree<T>) -> Self {
        Self {
            cursor: tree.cursor_mut(),
            first_iteration: true,
        }
    }
}

impl<'t, T> InorderSubtreeIterMut<'t, T> {
    fn new(cursor: CursorMut<'t, T>) -> Self {
        Self {
            cursor,
            current_depth: 0,
            first_iteration: true,
        }
    }
}

impl<'t, T> Iterator for InorderIterMut<'t, T> {
    type Item = &'t mut T;

    fn next(&mut self) -> Option<Self::Item> {
        move_next_cursor!(self, false);

        // Extend the lifetime of the yielded reference to be independent of the iterator.
        // This is safe, because the reference cannot change the tree structure, nor other elements of the tree.
        let pointer = self.cursor.get_mut()? as *mut T;
        unsafe { Some(&mut *pointer) }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.cursor.tree().len(), Some(self.cursor.tree().len()))
    }
}

impl<'t, T> FusedIterator for InorderIterMut<'t, T> {}
impl<'t, T> ExactSizeIterator for InorderIterMut<'t, T> {
    fn len(&self) -> usize {
        self.cursor.tree().len()
    }
}

impl<'t, T> Iterator for InorderSubtreeIterMut<'t, T> {
    type Item = &'t mut T;

    fn next(&mut self) -> Option<Self::Item> {
        move_next_cursor!(self, true);

        // Extend the lifetime of the yielded reference to be independent of the iterator.
        // This is safe, because the reference cannot change the tree structure, nor other elements of the tree.
        let pointer = self.cursor.get_mut()? as *mut T;
        unsafe { Some(&mut *pointer) }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.cursor.tree().len()))
    }
}

impl<'t, T> FusedIterator for InorderSubtreeIterMut<'t, T> {}

pub struct IntoInorderIter<T> {
    tree: BinaryTree<T>,
    stack: Vec<NodeId>,
}

pub struct DrainSubtreeInorder<'t, T> {
    tree: &'t mut BinaryTree<T>,
    stack: Vec<NodeId>,
}

impl<T> IntoInorderIter<T> {
    fn new(tree: BinaryTree<T>) -> Self {
        // Make an initial stack, up to the leftmost node in the tree.
        let mut stack = Vec::new();
        let mut cursor = tree.cursor();
        while !cursor.node_id().is_null() {
            stack.push(cursor.node_id());
            cursor.move_left();
        }

        Self {
            tree,
            stack,
        }
    }
}

impl<'t, T> DrainSubtreeInorder<'t, T> {
    fn new(mut cursor: CursorMut<'t, T>) -> Self {
        // Make an initial stack, up to the leftmost node in the tree.
        let mut stack = Vec::new();
        while !cursor.node_id().is_null() {
            stack.push(cursor.node_id());
            cursor.move_left();
        }

        Self {
            tree: cursor.into_tree_mut(),
            stack,
        }
    }
}

impl<T> Iterator for IntoInorderIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Get id of the to-be-reported element, and expand stack.
        let next_id = self.stack.pop()?;
        if let Some(id) = self.tree.right_id(next_id) && !id.is_null() {
            self.stack.push(id);
            while let Some(id) = self.tree.left_id(*self.stack.last().unwrap()) && !id.is_null() {
                self.stack.push(id);
            }
        }
        self.tree.remove_node(next_id)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.tree.len(), Some(self.tree.len()))
    }
}

impl<T> FusedIterator for IntoInorderIter<T> {}
impl<T> ExactSizeIterator for IntoInorderIter<T> {
    fn len(&self) -> usize {
        self.tree.len()
    }
}

impl<'t, T> Iterator for DrainSubtreeInorder<'t, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Get id of the to-be-reported element, and expand stack.
        let next_id = self.stack.pop()?;
        if let Some(id) = self.tree.right_id(next_id) && !id.is_null() {
            self.stack.push(id);
            while let Some(id) = self.tree.left_id(*self.stack.last().unwrap()) && !id.is_null() {
                self.stack.push(id);
            }
        }
        self.tree.remove_node(next_id)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.tree.len()))
    }
}

impl<'t, T> FusedIterator for DrainSubtreeInorder<'t, T> {}

/// Custom drop implementation that removes the remaining elements in the subtree from the tree.
/// This is done to ensure the tree remains a valid binary tree.
/// In particular, it ensures the tree stays connected.
impl<'t, T> Drop for DrainSubtreeInorder<'t, T> {
    fn drop(&mut self) {
        struct DropGuard<'a, 't, T>(&'a mut DrainSubtreeInorder<'t, T>);

        impl<'a, 't, T> Drop for DropGuard<'a, 't, T> {
            fn drop(&mut self) {
                // Continue the same loop we do below.
                // This only runs when a destructor has panicked.
                // If another one panics this will abort.
                while self.0.next().is_some() {}
            }
        }

        // Wrap self so that if a destructor panics, we can try to keep iterating.
        let guard = DropGuard(self);
        while guard.0.next().is_some() {}
        mem::forget(guard);
    }
}
