use std::mem;

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

    pub fn postorder_iter_mut(&mut self) -> PostorderIterMut<'_, T> {
        PostorderIterMut::new(self)
    }

    pub fn into_postorder_iter(self) -> IntoPostorderIter<T> {
        IntoPostorderIter::new(self)
    }
}

impl<'t, T> Cursor<'t, T> {
    pub fn postorder_subtree_iter(self) -> PostorderSubtreeIter<'t, T> {
        PostorderSubtreeIter::new(self)
    }
}

impl<'t, T> CursorMut<'t, T> {
    pub fn postorder_subtree_iter_mut(self) -> PostorderSubtreeIterMut<'t, T> {
        PostorderSubtreeIterMut::new(self)
    }

    pub fn drain_subtree_postorder(self) -> DrainSubtreePostorder<'t, T> {
        DrainSubtreePostorder::new(self)
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
        } else {
            let side = $self.cursor.move_up();
            dec_depth!($self, $track_depth);
            if side == Some(Side::Left) && $self.cursor.try_move_right() {
                inc_depth!($self, $track_depth);
                while $self.cursor.try_move_left() {
                    inc_depth!($self, $track_depth);
                }
            }
        }
    }};
}

pub struct PostorderIter<'t, T> {
    cursor: Cursor<'t, T>,
    first_iteration: bool,
}

pub struct PostorderSubtreeIter<'t, T> {
    cursor: Cursor<'t, T>,
    current_depth: usize,
    first_iteration: bool,
}

impl<'t, T> PostorderIter<'t, T> {
    fn new(tree: &'t BinaryTree<T>) -> Self {
        Self {
            cursor: tree.cursor(),
            first_iteration: true,
        }
    }
}

impl<'t, T> PostorderSubtreeIter<'t, T> {
    fn new(cursor: Cursor<'t, T>) -> Self {
        Self {
            cursor,
            current_depth: 0,
            first_iteration: true,
        }
    }
}

impl<'t, T> Iterator for PostorderIter<'t, T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        move_next_cursor!(self, false);
        self.cursor.get()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.cursor.tree().len(), Some(self.cursor.tree().len()))
    }
}

impl<'t, T> Iterator for PostorderSubtreeIter<'t, T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        move_next_cursor!(self, true);
        self.cursor.get()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.cursor.tree().len()))
    }
}

pub struct PostorderIterMut<'t, T> {
    cursor: CursorMut<'t, T>,
    first_iteration: bool,
}

pub struct PostorderSubtreeIterMut<'t, T> {
    cursor: CursorMut<'t, T>,
    current_depth: usize,
    first_iteration: bool,
}

impl<'t, T> PostorderIterMut<'t, T> {
    fn new(tree: &'t mut BinaryTree<T>) -> Self {
        Self {
            cursor: tree.cursor_mut(),
            first_iteration: true,
        }
    }
}

impl<'t, T> PostorderSubtreeIterMut<'t, T> {
    fn new(cursor: CursorMut<'t, T>) -> Self {
        Self {
            cursor,
            current_depth: 0,
            first_iteration: true,
        }
    }
}

impl<'t, T> Iterator for PostorderIterMut<'t, T> {
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

impl<'t, T> Iterator for PostorderSubtreeIterMut<'t, T> {
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

pub struct IntoPostorderIter<T> {
    tree: BinaryTree<T>,
    stack: Vec<NodeId>,
}

pub struct DrainSubtreePostorder<'t, T> {
    tree: &'t mut BinaryTree<T>,
    stack: Vec<NodeId>,
}

impl<T> IntoPostorderIter<T> {
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

impl<'t, T> DrainSubtreePostorder<'t, T> {
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

impl<T> Iterator for IntoPostorderIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Expand the stack first, then report the element.
        let id = self.stack.last()?;
        if let Some(right_id) = self.tree.right_id(*id) && !right_id.is_null() {
            self.stack.push(right_id);
            while let Some(left_id) = self.tree.left_id(*self.stack.last().unwrap()) && !left_id.is_null() {
                self.stack.push(left_id);
            }
        }
        let next_id = self.stack.pop().unwrap();
        self.tree.remove_node(next_id)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.tree.len(), Some(self.tree.len()))
    }
}

impl<'t, T> Iterator for DrainSubtreePostorder<'t, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Expand the stack first, then report the element.
        let id = self.stack.last()?;
        if let Some(right_id) = self.tree.right_id(*id) && !right_id.is_null() {
            self.stack.push(right_id);
            while let Some(left_id) = self.tree.left_id(*self.stack.last().unwrap()) && !left_id.is_null() {
                self.stack.push(left_id);
            }
        }
        let next_id = self.stack.pop().unwrap();
        self.tree.remove_node(next_id)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.tree.len()))
    }
}

/// Custom drop implementation that removes the remaining elements in the subtree from the tree.
/// This is done to ensure the tree remains a valid binary tree.
/// In particular, it ensures the tree stays connected.
impl<'t, T> Drop for DrainSubtreePostorder<'t, T> {
    fn drop(&mut self) {
        struct DropGuard<'a, 't, T>(&'a mut DrainSubtreePostorder<'t, T>);

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
