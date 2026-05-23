use crate::binary_trees::{
    Side, 
    binary_tree::{
        BinaryTree,
        BinaryTreeNode,
        Cursor,
        NodeId,
    },
};

impl<T> BinaryTree<T> {
    pub fn preorder_iter(&self) -> PreorderIter<'_, T> {
        PreorderIter::new(self)
    }

    pub fn preorder_iter_filtered<P>(&self, subtree_filter: P) -> PreorderIterFiltered<'_, T, P>
    where 
        P: Fn(&T) -> bool,
    {
        PreorderIterFiltered::new(self, subtree_filter)
    }

    pub fn preorder_iter_mut(&mut self) -> PreorderIterMut<'_, T> {
        PreorderIterMut::new(self)
    }

    pub fn preorder_iter_filtered_mut<P>(&mut self, subtree_filter: P) -> PreorderIterFilteredMut<'_, T, P>
    where 
        P: Fn(&T) -> bool,
    {
        PreorderIterFilteredMut::new(self, subtree_filter)
    }

    pub fn into_preorder_iter(self) -> IntoPreorderIter<T> {
        IntoPreorderIter::new(self)
    }

    pub fn into_preorder_iter_filtered<P>(self, subtree_filter: P) -> IntoPreorderIterFiltered<T, P>
    where 
        P: Fn(&T) -> bool,
    {
        IntoPreorderIterFiltered::new(self, subtree_filter)
    }
}

pub struct PreorderIter<'t, T>(PreorderIterFiltered<'t, T, fn(&T) -> bool>);

pub struct PreorderIterFiltered<'t, T, P> {
    tree: &'t BinaryTree<T>,
    subtree_filter: P,
    current_id: Option<NodeId>,
}

impl<'t, T> PreorderIter<'t, T> {
    fn new(tree: &'t BinaryTree<T>) -> Self {
        Self(PreorderIterFiltered::new(tree, |_| true))
    }
}

impl<'t, T, P> PreorderIterFiltered<'t, T, P> 
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
        if let Some(left) = cursor.peek_left() && (self.subtree_filter)(left) {
            cursor.move_left();
        } else if let Some(right) = cursor.peek_right() && (self.subtree_filter)(right) {
            cursor.move_right();
        } else {
            while let Some(side) = cursor.move_up() {
                if side == Side::Left && let Some(right) = cursor.peek_right() && (self.subtree_filter)(right) {
                    cursor.move_right();
                    break;
                }
            }
        }
        Some(cursor.node_id())
    }
}

impl<'t, T> Iterator for PreorderIter<'t, T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'t, T, P> Iterator for PreorderIterFiltered<'t, T, P>
where 
    P: Fn(&T) -> bool,
{
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_id.is_none() {
            // In the first iteration, move the "cursor" to the root of the tree, if it satisfies the filter.
            let root_id = self.tree.root_id();
            if self.is_id_valid(root_id) {
                self.current_id = Some(root_id);
            }
        } else {
            self.current_id = self.next_node_id();
        }
        self.tree.node(self.current_id?).map(BinaryTreeNode::data)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.tree.len()))
    }
}

pub struct PreorderIterMut<'t, T>(PreorderIterFilteredMut<'t, T, fn(&T) -> bool>);

pub struct PreorderIterFilteredMut<'t, T, P> {
    tree: &'t mut BinaryTree<T>,
    subtree_filter: P,
    current_id: Option<NodeId>,
}

impl<'t, T> PreorderIterMut<'t, T> {
    fn new(tree: &'t mut BinaryTree<T>) -> Self {
        Self(PreorderIterFilteredMut::new(tree, |_| true))
    }
}

impl<'t, T, P> PreorderIterFilteredMut<'t, T, P> 
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
        if let Some(left) = cursor.peek_left() && (self.subtree_filter)(left) {
            cursor.move_left();
        } else if let Some(right) = cursor.peek_right() && (self.subtree_filter)(right) {
            cursor.move_right();
        } else {
            while let Some(side) = cursor.move_up() {
                if side == Side::Left && let Some(right) = cursor.peek_right() && (self.subtree_filter)(right) {
                    cursor.move_right();
                    break;
                }
            }
        }
        Some(cursor.node_id())
    }
}

impl<'t, T> Iterator for PreorderIterMut<'t, T> {
    type Item = &'t mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'t, T, P> Iterator for PreorderIterFilteredMut<'t, T, P>
where 
    P: Fn(&T) -> bool,
{
    type Item = &'t mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_id.is_none() {
            // In the first iteration, move the "cursor" to the root of the tree, if it satisfies the filter.
            let root_id = self.tree.root_id();
            if self.is_id_valid(root_id) {
                self.current_id = Some(root_id);
            }
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


pub struct IntoPreorderIter<T>(IntoPreorderIterFiltered<T, fn(&T) -> bool>);

pub struct IntoPreorderIterFiltered<T, P> {
    tree: BinaryTree<T>,
    subtree_filter: P,
    stack: Vec<NodeId>,
}

impl<T> IntoPreorderIter<T> {
    fn new(tree: BinaryTree<T>) -> Self {
        Self(IntoPreorderIterFiltered::new(tree, |_| true))
    }
}

impl<T, P> IntoPreorderIterFiltered<T, P> 
where 
    P: Fn(&T) -> bool,
{
    fn new(tree: BinaryTree<T>, subtree_filter: P) -> Self {
        let id = tree.root_id();
        let mut stack = Vec::new();
        if tree.node(id).map(|node| subtree_filter(node.data())).unwrap_or(false) {
            stack.push(id);
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

impl<T> Iterator for IntoPreorderIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<T, P> Iterator for IntoPreorderIterFiltered<T, P>
where 
    P: Fn(&T) -> bool,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Get id of the to-be-reported element, and expand stack.
        let next_id = self.stack.pop()?;
        if let Some(right_id) = self.tree.right_id(next_id) && self.is_id_valid(right_id) {
            self.stack.push(right_id);
        }
        if let Some(left_id) = self.tree.left_id(next_id) && self.is_id_valid(left_id) {
            self.stack.push(left_id);
        }
        self.tree.remove_node(next_id)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.tree.len(), Some(self.tree.len()))
    }
}
