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
    pub fn preorder_iter(&self) -> PreorderIter<'_, T> {
        PreorderIter::new(self)
    }

    pub fn preorder_iter_mut(&mut self) -> PreorderIterMut<'_, T> {
        PreorderIterMut::new(self)
    }

    pub fn into_preorder_iter(self) -> IntoPreorderIter<T> {
        IntoPreorderIter::new(self)
    }
}

impl<'t, T> Cursor<'t, T> {
    pub fn preorder_subtree_iter(self) -> PreorderSubtreeIter<'t, T> {
        PreorderSubtreeIter::new(self)
    }
}

impl<'t, T> CursorMut<'t, T> {
    pub fn preorder_subtree_iter(self) -> PreorderSubtreeIter<'t, T> {
        PreorderSubtreeIter::new(self.into())
    }

    pub fn preorder_subtree_iter_mut(self) -> PreorderSubtreeIterMut<'t, T> {
        PreorderSubtreeIterMut::new(self)
    }

    pub fn drain_subtree_preorder(self) -> DrainSubtreePreorder<'t, T> {
        DrainSubtreePreorder::new(self)
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
            $self.first_iteration = false;
        } else if $self.cursor.try_move_left() {
            inc_depth!($self, $track_depth);
        } else if $self.cursor.try_move_right() {
            inc_depth!($self, $track_depth);
        } else {
            loop {
                match $self.cursor.move_up() {
                    Some(Side::Left) => {
                        dec_depth!($self, $track_depth);
                        if $self.cursor.try_move_right() {
                            inc_depth!($self, $track_depth);
                            break;
                        }
                    },
                    Some(Side::Right) => {
                        dec_depth!($self, $track_depth);
                    },
                    None => break,
                }
            }
        }
    }};
}

pub struct PreorderIter<'t, T> {
    cursor: Cursor<'t, T>,
    first_iteration: bool,
}

pub struct PreorderSubtreeIter<'t, T> {
    cursor: Cursor<'t, T>,
    current_depth: usize,
    first_iteration: bool,
}

impl<'t, T> PreorderIter<'t, T> {
    fn new(tree: &'t BinaryTree<T>) -> Self {
        Self {
            cursor: tree.cursor(),
            first_iteration: true,
        }
    }
}

impl<'t, T> PreorderSubtreeIter<'t, T> {
    fn new(cursor: Cursor<'t, T>) -> Self {
        Self {
            cursor,
            current_depth: 0,
            first_iteration: true,
        }
    }
}

impl<'t, T> Iterator for PreorderIter<'t, T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        move_next_cursor!(self, false);
        self.cursor.get()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.cursor.tree().len(), Some(self.cursor.tree().len()))
    }
}

impl<'t, T> Iterator for PreorderSubtreeIter<'t, T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        move_next_cursor!(self, true);
        self.cursor.get()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.cursor.tree().len()))
    }
}

pub struct PreorderIterMut<'t, T> {
    cursor: CursorMut<'t, T>,
    first_iteration: bool,
}

pub struct PreorderSubtreeIterMut<'t, T> {
    cursor: CursorMut<'t, T>,
    current_depth: usize,
    first_iteration: bool,
}

impl<'t, T> PreorderIterMut<'t, T> {
    fn new(tree: &'t mut BinaryTree<T>) -> Self {
        Self {
            cursor: tree.cursor_mut(),
            first_iteration: true,
        }
    }
}

impl<'t, T> PreorderSubtreeIterMut<'t, T> {
    fn new(cursor: CursorMut<'t, T>) -> Self {
        Self {
            cursor,
            current_depth: 0,
            first_iteration: true,
        }
    }
}

impl<'t, T> Iterator for PreorderIterMut<'t, T> {
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

impl<'t, T> Iterator for PreorderSubtreeIterMut<'t, T> {
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

pub struct IntoPreorderIter<T> {
    tree: BinaryTree<T>,
    stack: Vec<NodeId>,
}

pub struct DrainSubtreePreorder<'t, T> {
    tree: &'t mut BinaryTree<T>,
    stack: Vec<NodeId>,
}

impl<T> IntoPreorderIter<T> {
    fn new(tree: BinaryTree<T>) -> Self {
        let root_id = tree.root_id();
        Self {
            tree,
            stack: vec![root_id],
        }
    }
}

impl<'t, T> DrainSubtreePreorder<'t, T> {
    fn new(cursor: CursorMut<'t, T>) -> Self {
        let root_id = cursor.node_id();
        Self {
            tree: cursor.into_tree_mut(),
            stack: vec![root_id],
        }
    }
}

impl<T> Iterator for IntoPreorderIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next_id = self.stack.pop()?;

        if let Some(right_id) = self.tree.right_id(next_id) && !right_id.is_null() {
            self.stack.push(right_id);
        }
        if let Some(left_id) = self.tree.left_id(next_id) && !left_id.is_null() {
            self.stack.push(left_id);
        }
        self.tree.remove_node(next_id)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.tree.len(), Some(self.tree.len()))
    }
}

impl<'t, T> Iterator for DrainSubtreePreorder<'t, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next_id = self.stack.pop()?;

        if let Some(right_id) = self.tree.right_id(next_id) && !right_id.is_null() {
            self.stack.push(right_id);
        }
        if let Some(left_id) = self.tree.left_id(next_id) && !left_id.is_null() {
            self.stack.push(left_id);
        }
        self.tree.remove_node(next_id)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.tree.len()))
    }
}

/// Custom drop implementation that removes the remaining elements in the subtree from the tree.
/// This is done to ensure the tree remains a valid binary tree.
/// In particular, it ensures the tree stays connected.
impl<'t, T> Drop for DrainSubtreePreorder<'t, T> {
    fn drop(&mut self) {
        struct DropGuard<'a, 't, T>(&'a mut DrainSubtreePreorder<'t, T>);

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
