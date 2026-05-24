use slotmap::Key;

use crate::binary_trees::{
    Side,
    binary_tree::{
        BinaryTree,
        BinaryTreeNode,
        Cursor,
        NodeId,
    },
    binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursor,
    }, tree_iterators::TreeIterator,
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

pub struct InorderIter<'t, T> {
    tree: &'t BinaryTree<T>,
    current_id: Option<NodeId>,
}

impl<'t, T> InorderIter<'t, T> {
    fn new(tree: &'t BinaryTree<T>) -> Self {
        Self {
            tree,
            current_id: None,
        }
    }

    fn next_id(&mut self) -> Option<NodeId> {
        let mut cursor = Cursor::new(self.tree, self.current_id?);
        if cursor.try_move_right() {
            while cursor.try_move_left() {}
        } else {
            while cursor.move_up() == Some(Side::Right) {}
        }
        Some(cursor.node_id())
    }

    fn is_id_valid<P>(&self, id: NodeId, mut predicate: P) -> bool
    where 
        P: FnMut(&T) -> bool,
    {
        self.tree.node(id)
            .map(|node| (predicate)(node.data()))
            .unwrap_or(false)
    }

    fn next_id_with_filter<P>(&mut self, mut predicate: P) -> Option<NodeId>
    where 
        P: FnMut(&T) -> bool,
    {
        let mut cursor = Cursor::new(self.tree, self.current_id?);
        if let Some(right) = cursor.peek_right() && (predicate)(right) {
            cursor.move_right();
            while let Some(left) = cursor.peek_left() && (predicate)(left) {
                cursor.move_left();
            }
        } else {
            while cursor.move_up() == Some(Side::Right) {}
        }
        Some(cursor.node_id())
    }
}

impl<'t, T> Iterator for InorderIter<'t, T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_id.is_none() {
            // In the first iteration, move the "cursor" to the leftmost node.
            let mut cursor = self.tree.cursor();
            while cursor.try_move_left() {}
            self.current_id = Some(cursor.node_id());
        } else {
            self.current_id = self.next_id();
        }
        self.tree.node(self.current_id?).map(BinaryTreeNode::data)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.tree.len()))
    }
}

impl<'t, T> TreeIterator<T> for InorderIter<'t, T> {
    fn next_with_subtree_filter<P>(&mut self, mut predicate: P) -> Option<Self::Item>
    where 
        P: FnMut(&T) -> bool
    {
        if self.current_id.is_none() {
            // In the first iteration, move the "cursor" to the leftmost node that satisfies the filter.
            let mut cursor = self.tree.cursor();
            while self.is_id_valid(cursor.node_id(), &mut predicate) && cursor.try_move_left() {}
            if !self.is_id_valid(cursor.node_id(), predicate) {
                cursor.move_up();
            }
            self.current_id = Some(cursor.node_id());
        } else {
            self.current_id = self.next_id_with_filter(predicate);
        }
        self.tree.node(self.current_id?).map(BinaryTreeNode::data)
    }
}

pub struct InorderIterMut<'t, T> {
    tree: &'t mut BinaryTree<T>,
    current_id: Option<NodeId>,
}

impl<'t, T> InorderIterMut<'t, T> {
    fn new(tree: &'t mut BinaryTree<T>) -> Self {
        Self {
            tree,
            current_id: None,
        }
    }

    fn next_id(&mut self) -> Option<NodeId> {
        let mut cursor = Cursor::new(self.tree, self.current_id?);
        if cursor.try_move_right() {
            while cursor.try_move_left() {}
        } else {
            while cursor.move_up() == Some(Side::Right) {}
        }
        Some(cursor.node_id())
    }

    fn is_id_valid<P>(&self, id: NodeId, mut predicate: P) -> bool
    where 
        P: FnMut(&T) -> bool,
    {
        self.tree.node(id)
            .map(|node| (predicate)(node.data()))
            .unwrap_or(false)
    }

    fn next_id_with_filter<P>(&mut self, mut predicate: P) -> Option<NodeId>
    where 
        P: FnMut(&T) -> bool,
    {
        let mut cursor = Cursor::new(self.tree, self.current_id?);
        if let Some(right) = cursor.peek_right() && (predicate)(right) {
            cursor.move_right();
            while let Some(left) = cursor.peek_left() && (predicate)(left) {
                cursor.move_left();
            }
        } else {
            while cursor.move_up() == Some(Side::Right) {}
        }
        Some(cursor.node_id())
    }
}

impl<'t, T> Iterator for InorderIterMut<'t, T> {
    type Item = &'t mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_id.is_none() {
            // In the first iteration, move the "cursor" to the leftmost node.
            let mut cursor = self.tree.cursor();
            while cursor.try_move_left() {}
            self.current_id = Some(cursor.node_id());
        } else {
            self.current_id = self.next_id();
        }
        let next = self.tree.node_mut(self.current_id?).map(BinaryTreeNode::data_mut)?;
        // Extend the lifetime of the yielded reference to be independent of the iterator.
        // This is safe, because the reference cannot change the tree structure, nor other elements of the tree.
        let pointer = next as *mut T;
        unsafe { Some(&mut *pointer) }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.tree.len()))
    }
}

impl<'t, T> TreeIterator<T> for InorderIterMut<'t, T> {
    fn next_with_subtree_filter<P>(&mut self, mut predicate: P) -> Option<Self::Item>
    where 
        P: FnMut(&T) -> bool
    {
        if self.current_id.is_none() {
            // In the first iteration, move the "cursor" to the leftmost node that satisfies the filter.
            let mut cursor = self.tree.cursor();
            while self.is_id_valid(cursor.node_id(), &mut predicate) && cursor.try_move_left() {}
            if !self.is_id_valid(cursor.node_id(), predicate) {
                cursor.move_up();
            }
            self.current_id = Some(cursor.node_id());
        } else {
            self.current_id = self.next_id_with_filter(predicate);
        }
        let next = self.tree.node_mut(self.current_id?).map(BinaryTreeNode::data_mut)?;
        // Extend the lifetime of the yielded reference to be independent of the iterator.
        // This is safe, because the reference cannot change the tree structure, nor other elements of the tree.
        let pointer = next as *mut T;
        unsafe { Some(&mut *pointer) }
    }
}

pub struct IntoInorderIter<T> {
    tree: BinaryTree<T>,
    stack: Vec<NodeId>,
    first_iteration: bool,
}

impl<T> IntoInorderIter<T> {
    fn new(tree: BinaryTree<T>) -> Self {
        Self {
            tree,
            stack: Vec::new(),
            first_iteration: true,
        }
    }

    fn is_id_valid<P>(&self, id: NodeId, mut predicate: P) -> bool
    where 
        P: FnMut(&T) -> bool,
    {
        self.tree.node(id)
            .map(|node| predicate(node.data()))
            .unwrap_or(false)
    }
}

impl<T> Iterator for IntoInorderIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_iteration {
            // Make an initial stack, up to the leftmost node in the tree.
            let mut cursor = self.tree.cursor();
            while !cursor.node_id().is_null() {
                self.stack.push(cursor.node_id());
                cursor.move_left();
            }
            self.first_iteration = false;
        }

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

impl<T> TreeIterator<T> for IntoInorderIter<T> {
    fn next_with_subtree_filter<P>(&mut self, mut predicate: P) -> Option<Self::Item>
    where 
        P: FnMut(&T) -> bool
    {
        if self.first_iteration {
            // Make an initial stack, up to the leftmost node in the tree.
            let mut cursor = self.tree.cursor();
            while let Some(data) = cursor.get() && predicate(data) {
                self.stack.push(cursor.node_id());
                cursor.move_left();
            }
            self.first_iteration = false;
        }

        // Get id of the to-be-reported element, and expand stack.
        let next_id = self.stack.pop()?;
        if let Some(id) = self.tree.right_id(next_id) && self.is_id_valid(id, &mut predicate) {
            self.stack.push(id);
            while let Some(id) = self.tree.left_id(*self.stack.last().unwrap()) && self.is_id_valid(id, &mut predicate) {
                self.stack.push(id);
            }
        }
        self.tree.remove_node(next_id)
    }
}
