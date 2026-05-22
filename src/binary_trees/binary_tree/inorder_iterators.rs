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
    },
};

impl<T> BinaryTree<T> {
    pub fn inorder_iter(&self) -> InorderIter<'_, T> {
        InorderIter::new(self)
    }

    pub fn inorder_iter_filtered<P>(&self, subtree_filter: P) -> InorderIterFiltered<'_, T, P>
    where 
        P: Fn(&T) -> bool,
    {
        InorderIterFiltered::new(self, subtree_filter)
    }

    pub fn inorder_iter_mut(&mut self) -> InorderIterMut<'_, T> {
        InorderIterMut::new(self)
    }

    pub fn inorder_iter_filtered_mut<P>(&mut self, subtree_filter: P) -> InorderIterFilteredMut<'_, T, P>
    where 
        P: Fn(&T) -> bool,
    {
        InorderIterFilteredMut::new(self, subtree_filter)
    }

    pub fn into_inorder_iter(self) -> IntoInorderIter<T> {
        IntoInorderIter::new(self)
    }

    pub fn into_inorder_iter_filtered<P>(self, subtree_filter: P) -> IntoInorderIterFiltered<T, P>
    where 
        P: Fn(&T) -> bool,
    {
        IntoInorderIterFiltered::new(self, subtree_filter)
    }
}

pub struct InorderIter<'t, T>(InorderIterFiltered<'t, T, fn(&T) -> bool>);

pub struct InorderIterFiltered<'t, T, P> {
    cursor: Cursor<'t, T>,
    subtree_filter: P,
    first_iteration: bool,
}

impl<'t, T> InorderIter<'t, T> {
    fn new(tree: &'t BinaryTree<T>) -> Self {
        Self(InorderIterFiltered::new(tree, |_| true))
    }
}

impl<'t, T, P> InorderIterFiltered<'t, T, P> {
    fn new(tree: &'t BinaryTree<T>, subtree_filter: P) -> Self {
        Self {
            cursor: tree.cursor(),
            subtree_filter,
            first_iteration: true,
        }
    }

    fn is_cursor_in_valid_node(&self) -> bool 
    where 
        P: Fn(&T) -> bool,
    {
        self.cursor.get().map_or(false, &self.subtree_filter)
    }

    /// Moves the given cursor to the next (possibly null) node of the inorder iterator.
    /// Assumes the cursor points to the previous element in the iterator.
    fn move_cursor_to_next_node(&mut self)
    where 
        P: Fn(&T) -> bool,
    {
        if self.cursor.try_move_right() {
            while self.is_cursor_in_valid_node() && self.cursor.try_move_left() {}
        } else {
            while self.cursor.move_up() == Some(Side::Right) {}
        }
    }
}

impl<'t, T> Iterator for InorderIter<'t, T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'t, T, P> Iterator for InorderIterFiltered<'t, T, P>
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

pub struct InorderIterMut<'t, T>(InorderIterFilteredMut<'t, T, fn(&T) -> bool>);

pub struct InorderIterFilteredMut<'t, T, P> {
    tree: &'t mut BinaryTree<T>,
    subtree_filter: P,
    current_id: Option<NodeId>,
}

impl<'t, T> InorderIterMut<'t, T> {
    fn new(tree: &'t mut BinaryTree<T>) -> Self {
        Self(InorderIterFilteredMut::new(tree, |_| true))
    }
}

impl<'t, T, P> InorderIterFilteredMut<'t, T, P> 
where 
    P: Fn(&T) -> bool,
{
    fn new(tree: &'t mut BinaryTree<T>, subtree_filter: P) -> Self {
        Self {
            tree,
            subtree_filter,
            current_id: None,
        }
    }

    fn is_id_valid(&self, id: NodeId) -> bool
    where 
        P: Fn(&T) -> bool,
    {
        self.tree.node(id)
            .map(|node| (self.subtree_filter)(node.data()))
            .unwrap_or(false)
    }

    fn next_node_id(&mut self) -> Option<NodeId>
    where 
        P: Fn(&T) -> bool,
    {
        let mut cursor = Cursor::new(self.tree, self.current_id?);
        if let Some(right) = cursor.peek_right() && (self.subtree_filter)(right) {
            cursor.move_right();
            while let Some(left) = cursor.peek_left() && (self.subtree_filter)(left) {
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
        self.0.next()
    }
}

impl<'t, T, P> Iterator for InorderIterFilteredMut<'t, T, P>
where 
    P: Fn(&T) -> bool,
{
    type Item = &'t mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_id.is_none() {
            // In the first iteration, move the "cursor" to the leftmost node that satisfies the filter.
            let mut cursor = self.tree.cursor();
            while self.is_id_valid(cursor.node_id()) && cursor.try_move_right() {}
            if !self.is_id_valid(cursor.node_id()) {
                cursor.move_up();
            }
            self.current_id = Some(cursor.node_id());
        } else {
            self.current_id = self.next_node_id();
        }
        let next = self.tree.node_mut(self.current_id.unwrap()).map(BinaryTreeNode::data_mut)?;
        // Extend the lifetime of the yielded reference to be independent of the iterator.
        // This is safe, because the reference cannot change the tree structure, nor other elements of the tree.
        let pointer = next as *mut T;
        unsafe { Some(&mut *pointer) }
    }
}

pub struct IntoInorderIter<T>(IntoInorderIterFiltered<T, fn(&T) -> bool>);

pub struct IntoInorderIterFiltered<T, P> {
    tree: BinaryTree<T>,
    subtree_filter: P,
    stack: Vec<NodeId>,
}

impl<T> IntoInorderIter<T> {
    fn new(tree: BinaryTree<T>) -> Self {
        Self(IntoInorderIterFiltered::new(tree, |_| true))
    }
}

impl<T, P> IntoInorderIterFiltered<T, P> 
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

impl<T> Iterator for IntoInorderIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<T, P> Iterator for IntoInorderIterFiltered<T, P>
where 
    P: Fn(&T) -> bool,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Get id of the to-be-reported element, and expand stack.
        let next_id = self.stack.pop()?;
        if let Some(id) = self.tree.right_id(next_id) && self.is_id_valid(id) {
            self.stack.push(id);
            while let Some(id) = self.tree.left_id(*self.stack.last().unwrap()) && self.is_id_valid(id) {
                self.stack.push(id);
            }
        }
        self.tree.remove_node(next_id)
    }
}
