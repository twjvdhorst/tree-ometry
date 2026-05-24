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


fn move_cursor_to_next_node<T>(cursor: &mut Cursor<'_, T>) {
    if cursor.try_move_left() {
    } else if cursor.try_move_right() {
    } else {
        while let Some(side) = cursor.move_up() {
            if side == Side::Left && cursor.try_move_right() {
                break;
            }
        }
    }
}

fn is_id_valid<T, P>(tree: &BinaryTree<T>, id: NodeId, mut predicate: P) -> bool
where 
    P: FnMut(&T) -> bool,
{
    tree.node(id)
        .map(|node| predicate(node.data()))
        .unwrap_or(false)
}

fn move_cursor_to_next_valid_node<T, P>(cursor: &mut Cursor<'_, T>, mut predicate: P)
where 
    P: FnMut(&T) -> bool,
{
    if let Some(left) = cursor.peek_left() && predicate(left) {
        cursor.move_left();
    } else if let Some(right) = cursor.peek_right() && predicate(right) {
        cursor.move_right();
    } else {
        while let Some(side) = cursor.move_up() {
            if side == Side::Left && let Some(right) = cursor.peek_right() && predicate(right) {
                cursor.move_right();
                break;
            }
        }
    }
}

pub struct PreorderIter<'t, T> {
    tree: &'t BinaryTree<T>,
    current_id: Option<NodeId>,
}

impl<'t, T> PreorderIter<'t, T> {
    fn new(tree: &'t BinaryTree<T>) -> Self {
        Self {
            tree,
            current_id: None,
        }
    }
}

impl<'t, T> Iterator for PreorderIter<'t, T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(id) = self.current_id {
            let mut cursor = Cursor::new(self.tree, id);
            move_cursor_to_next_node(&mut cursor);
            self.current_id = Some(cursor.node_id());
        } else {
            self.current_id = Some(self.tree.root_id());
        }

        self.tree.node(self.current_id?).map(BinaryTreeNode::data)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.tree.len()))
    }
}

impl<'t, T> TreeIterator<T> for PreorderIter<'t, T> {
    fn next_with_subtree_filter<P>(&mut self, predicate: P) -> Option<Self::Item>
    where 
        P: FnMut(&T) -> bool
    {
        if let Some(id) = self.current_id {
            let mut cursor = Cursor::new(self.tree, id);
            move_cursor_to_next_valid_node(&mut cursor, predicate);
            self.current_id = Some(cursor.node_id());
        } else {
            let root_id = self.tree.root_id();
            if is_id_valid(self.tree, root_id, predicate) {
                self.current_id = Some(root_id);
            }
        }

        self.tree.node(self.current_id?).map(BinaryTreeNode::data)
    }
}

pub struct PreorderIterMut<'t, T> {
    tree: &'t mut BinaryTree<T>,
    current_id: Option<NodeId>,
}

impl<'t, T> PreorderIterMut<'t, T> {
    fn new(tree: &'t mut BinaryTree<T>) -> Self {
        Self {
            tree,
            current_id: None,
        }
    }
}

impl<'t, T> Iterator for PreorderIterMut<'t, T> {
    type Item = &'t mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(id) = self.current_id {
            let mut cursor = Cursor::new(self.tree, id);
            move_cursor_to_next_node(&mut cursor);
            self.current_id = Some(cursor.node_id());
        } else {
            self.current_id = Some(self.tree.root_id());
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

impl<'t, T> TreeIterator<T> for PreorderIterMut<'t, T> {
    fn next_with_subtree_filter<P>(&mut self, predicate: P) -> Option<Self::Item>
    where 
        P: FnMut(&T) -> bool
{
        if let Some(id) = self.current_id {
            let mut cursor = Cursor::new(self.tree, id);
            move_cursor_to_next_valid_node(&mut cursor, predicate);
            self.current_id = Some(cursor.node_id());
        } else {
            let root_id = self.tree.root_id();
            if is_id_valid(self.tree, root_id, predicate) {
                self.current_id = Some(root_id);
            }
        }

        let next = self.tree.node_mut(self.current_id?).map(BinaryTreeNode::data_mut)?;
        // Extend the lifetime of the yielded reference to be independent of the iterator.
        // This is safe, because the reference cannot change the tree structure, nor other elements of the tree.
        let pointer = next as *mut T;
        unsafe { Some(&mut *pointer) }
    }
}

pub struct IntoPreorderIter<T> {
    tree: BinaryTree<T>,
    stack: Vec<NodeId>,
    first_iteration: bool,
}

impl<T> IntoPreorderIter<T> {
    fn new(tree: BinaryTree<T>) -> Self {
        Self {
            tree,
            stack: Vec::new(),
            first_iteration: true,
        }
    }
}

impl<T> Iterator for IntoPreorderIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next_id = if self.first_iteration {
            self.first_iteration = false;
            self.tree.root_id()
        } else {
            self.stack.pop()?
        };

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

impl<T> TreeIterator<T> for IntoPreorderIter<T> {
    fn next_with_subtree_filter<P>(&mut self, mut predicate: P) -> Option<Self::Item>
    where 
        P: FnMut(&T) -> bool
    {
        let next_id = if self.first_iteration && is_id_valid(&self.tree, self.tree.root_id(), &mut predicate) {
            self.first_iteration = false;
            self.tree.root_id()
        } else {
            self.stack.pop()?
        };

        if let Some(right_id) = self.tree.right_id(next_id) && is_id_valid(&self.tree, right_id, &mut predicate) {
            self.stack.push(right_id);
        }
        if let Some(left_id) = self.tree.left_id(next_id) && is_id_valid(&self.tree, left_id, predicate) {
            self.stack.push(left_id);
        }
        self.tree.remove_node(next_id)
    }
}
