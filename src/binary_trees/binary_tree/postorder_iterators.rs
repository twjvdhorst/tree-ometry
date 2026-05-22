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
    tree: &'t BinaryTree<T>,
    subtree_filter: P,
    current_id: Option<NodeId>,
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
        if cursor.move_up()? == Side::Left {
            if let Some(right) = cursor.peek_right() && (self.subtree_filter)(right) {
                cursor.move_right();
                while let Some(left) = cursor.peek_left() && (self.subtree_filter)(left) {
                    cursor.move_left();
                }
            }
        }
        Some(cursor.node_id())
    }
}

impl<'t, T> Iterator for PostorderIter<'t, T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'t, T, P> Iterator for PostorderIterFiltered<'t, T, P>
where 
    P: Fn(&T) -> bool,
{
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_id.is_none() {
            // In the first iteration, move the "cursor" to the leftmost node that satisfies the filter.
            let mut cursor = self.tree.cursor();
            while self.is_id_valid(cursor.node_id()) && cursor.try_move_left() {}
            if !self.is_id_valid(cursor.node_id()) {
                cursor.move_up();
            }
            self.current_id = Some(cursor.node_id());
        } else {
            self.current_id = self.next_node_id();
        }
        self.tree.node(self.current_id?).map(BinaryTreeNode::data)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.tree.len()))
    }
}

pub struct PostorderIterMut<'t, T>(PostorderIterFilteredMut<'t, T, fn(&T) -> bool>);

pub struct PostorderIterFilteredMut<'t, T, P> {
    tree: &'t mut BinaryTree<T>,
    subtree_filter: P,
    current_id: Option<NodeId>,
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
        if cursor.move_up()? == Side::Left {
            if let Some(right) = cursor.peek_right() && (self.subtree_filter)(right) {
                cursor.move_right();
                while let Some(left) = cursor.peek_left() && (self.subtree_filter)(left) {
                    cursor.move_left();
                }
            }
        }
        Some(cursor.node_id())
    }
}

impl<'t, T> Iterator for PostorderIterMut<'t, T> {
    type Item = &'t mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'t, T, P> Iterator for PostorderIterFilteredMut<'t, T, P>
where 
    P: Fn(&T) -> bool,
{
    type Item = &'t mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_id.is_none() {
            // In the first iteration, move the "cursor" to the leftmost node that satisfies the filter.
            let mut cursor = self.tree.cursor();
            while self.is_id_valid(cursor.node_id()) && cursor.try_move_left() {}
            if !self.is_id_valid(cursor.node_id()) {
                cursor.move_up();
            }
            self.current_id = Some(cursor.node_id());
        } else {
            self.current_id = self.next_node_id();
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.tree.len(), Some(self.tree.len()))
    }
}
