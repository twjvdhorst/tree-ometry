use std::{
    borrow::Borrow, 
    cmp::Ordering, 
    fmt::{
        Debug, 
        Display,
    },
};

#[cfg(feature = "serde")]
use serde::Serialize;

use super::{
    Color, 
    InorderIter,
    InorderIterMut,
    IntoInorderIter,
    Cursor,
    CursorMut
};
use crate::binary_trees::{
    Neighborhood,
    Side,
    binary_tree::{
        BinaryTree,
        BinaryTreeNode,
    },
    binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursor,
        PeekingCursorMut,
    },
};

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub(super) struct RedBlackNode<K, V> {
    key: K, 
    value: V,
    color: Color,
}

impl<K, V> RedBlackNode<K, V> {
    pub(super) fn new_with_color(key: K, value: V, color: Color) -> Self {
        Self {
            key,
            value,
            color,
        }
    }
    
    pub(super) fn key(&self) -> &K {
        &self.key
    }

    pub(super) fn value(&self) -> &V {
        &self.value
    }

    pub(super) fn data(&self) -> (&K, &V) {
        (&self.key, &self.value)
    }

    pub(super) fn data_with_mut_value(&mut self) -> (&K, &mut V) {
        (&self.key, &mut self.value)
    }

    pub(super) fn into_data(self) -> (K, V) {
        (self.key, self.value)
    }

    pub(super) fn is_red(&self) -> bool {
        self.color == Color::Red
    }

    pub(super) fn is_black(&self) -> bool {
        self.color == Color::Black
    }

    pub(super) fn color(&self) -> Color {
        self.color
    }

    pub(super) fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl<K, V> Into<(K, V)> for RedBlackNode<K, V> {
    fn into(self) -> (K, V) {
        (self.key, self.value)
    }
}

#[derive(Clone)]
pub struct RedBlackTree<K, V>(pub(super) BinaryTree<RedBlackNode<K, V>>);

impl<K, V> Default for RedBlackTree<K, V> {
    fn default() -> Self {
        Self(BinaryTree::default())
    }
}

impl<K, V> Extend<(K, V)> for RedBlackTree<K, V>
where 
    K: Ord,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            self.insert(key, value);
        }
    }
}

impl<K, V> FromIterator<(K, V)> for RedBlackTree<K, V>
where 
    K: Ord,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut tree = Self::default();
        tree.extend(iter);
        tree
    }
}

impl<'t, K, V> IntoIterator for &'t RedBlackTree<K, V> {
    type Item = (&'t K, &'t V);
    type IntoIter = InorderIter<'t, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder_iter()
    }
}

impl<'t, K, V> IntoIterator for &'t mut RedBlackTree<K, V> {
    type Item = (&'t K, &'t mut V);
    type IntoIter = InorderIterMut<'t, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder_iter_mut()
    }
}

impl<K, V> IntoIterator for RedBlackTree<K, V> {
    type Item = (K, V);
    type IntoIter = IntoInorderIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_inorder_iter()
    }
}

impl<K, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(BinaryTree::with_capacity(capacity))
    }

    fn root(&self) -> Option<&RedBlackNode<K, V>> {
        self.0.root().map(BinaryTreeNode::data)
    }

    fn root_mut(&mut self) -> Option<&mut RedBlackNode<K, V>> {
        self.0.root_mut().map(BinaryTreeNode::data_mut)
    }

    pub(super) fn inner(&self) -> &BinaryTree<RedBlackNode<K, V>> {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn map_values<U, F>(self, mut f: F) -> RedBlackTree<K, U>
    where 
        F: FnMut(V) -> U,
    {
        let f = |node: RedBlackNode<K, V>| RedBlackNode {
            key: node.key, 
            value: f(node.value), 
            color: node.color,
        };
        RedBlackTree(self.0.map(f))
    }
    
    pub fn cursor(&self) -> Cursor<'_, K, V> {
        Cursor::new(self.0.cursor())
    }

    pub fn cursor_mut(&mut self) -> CursorMut<'_, K, V> {
        CursorMut::new(self.0.cursor_mut())
    }
}

/// Insertions.
impl<K, V> RedBlackTree<K, V>
where 
    K: Ord,
{
    /// Moves the cursor to the direct predecessor or successor of the value being inserted.
    /// Reports the side of the node that the key should be inserted at, or None if the node contains the key already.
    fn find_node_to_insert_at(cursor: &mut CursorMut<'_, K, V>, key: &K) -> Option<Side> {
        while let Some((curr_key, _)) = cursor.get_mut() {
            match K::cmp(&key, curr_key) {
                Ordering::Less => {
                    if !cursor.try_move_left() {
                        return Some(Side::Left);
                    }
                },
                Ordering::Greater => {
                    if !cursor.try_move_right() {
                        return Some(Side::Right);
                    }
                },
                Ordering::Equal => {
                    return None;
                },
            };
        }
        None
    }

    fn insert_fixup(cursor: &mut CursorMut<'_, K, V>) {
        // Cormen et al.'s algorithm.
        // We maintain the invariant that all nodes below the cursor have the correct semigroup value.
        while cursor.parent_color() == Some(Color::Red) {
            // Throughout the loop, cursor points to z, and peeking_cursor moves around to check states of various nodes.
            let mut peeking_cursor = cursor.as_cursor();
            let side_current = peeking_cursor.move_up().unwrap(); // Move the cursor to z.p
            let side_parent = peeking_cursor.move_up() // Move the cursor to z.p.p
                .unwrap(); // Can unwrap safely, as z.p.p exists by the proof of correctness by Cormen et al.

            if peeking_cursor.child_color(side_parent.opposite()) == Some(Color::Red) {
                // Case 1
                cursor.move_up(); // Move the cursor to z.p
                cursor.set_color(Color::Black);
                cursor.move_up(); // Move the cursor to z.p.p, where it stays for the next iteration.
                cursor.set_color(Color::Red);
                cursor.set_child_color(side_parent.opposite(), Color::Black);
            } else {
                if side_current == side_parent.opposite() {
                    // Case 2
                    cursor.move_up();
                    cursor.rotate(side_parent).unwrap();
                }

                // Case 3
                cursor.move_up();
                cursor.set_color(Color::Black);
                cursor.move_up();
                cursor.set_color(Color::Red);
                cursor.rotate(side_parent.opposite()).unwrap();

                // After rotating around z.p.p, z is the sibling of the node pointed at by the cursor.
                let side = cursor.move_up().unwrap();
                cursor.move_side(side.opposite());
            }
        }
    }

    /// Inserts the key-value pair into the tree.
    /// If the key was not present in the tree yet, None is returned.
    /// Otherwise, the value stored at the given key is updated, and the old value is returned.
    /// Time complexity: O(log n).
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // Cormen et al.'s algorithm.
        if self.root().is_none() {
            self.0 = BinaryTree::new_singleton(RedBlackNode::new_with_color(key, value, Color::Black));
            return None;
        }

        let mut cursor = self.cursor_mut();

        // Move the cursor to the direct predecessor or successor of the to-be-inserted key.
        let Some(side) = Self::find_node_to_insert_at(&mut cursor, &key) else {
            // Cursor was moved to the node containing the key.
            let old_value = std::mem::replace(cursor.get_mut().unwrap().1, value);
            return Some(old_value);
        };

        // The cursor now points to the parent of the node we will create.
        let new_node = RedBlackNode::new_with_color(key, value, Color::Red);
        cursor.attach_child(new_node, side).unwrap();

        // Fix the red-black tree structure.
        cursor.move_side(side);
        Self::insert_fixup(&mut cursor);

        // Maintain the invariant that the root is black.
        self.root_mut().unwrap().set_color(Color::Black); // Can unwrap safely: we already handled the case where the tree was empty.
        None
    }
}

/// Deletions.
impl<K, V> RedBlackTree<K, V>
where 
    K: Ord,
{
    /// Creates a cursor at the node storing the given key.
    /// Returns None if the key is not in the tree.
    fn get_cursor_mut_at_key<Q>(&mut self, key: &Q) -> Option<CursorMut<'_, K, V>>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut cursor = self.cursor_mut();
        while let Some((curr_key, _)) = cursor.get() {
            match Q::cmp(key, curr_key.borrow()) {
                Ordering::Less => cursor.move_left(),
                Ordering::Greater => cursor.move_right(),
                Ordering::Equal => return Some(cursor),
            };
        }
        None
    }

    fn move_cursor_to_successor(cursor: &mut impl BinaryTreeCursor) {
        if cursor.try_move_right() {
            while cursor.try_move_left() {}
        }
    }

    fn remove_fixup_leaf(cursor: &mut CursorMut<'_, K, V>, mut side: Side) {
        while cursor.get().is_some() && cursor.child_color(side) != Some(Color::Red) {
            if cursor.child_color(side.opposite()) == Some(Color::Red) {
                // Case 1.
                cursor.set_child_color(side.opposite(), Color::Black);
                cursor.set_color(Color::Red);
                cursor.rotate(side).unwrap();
            }
            
            cursor.move_side(side.opposite()); // Move the cursor to w
            if cursor.left_color() != Some(Color::Red) && cursor.right_color() != Some(Color::Red) {
                // Case 2.
                cursor.set_color(Color::Red);
                cursor.move_up(); // Move the cursor to x.p
            } else {
                if cursor.child_color(side.opposite()) != Some(Color::Red) {
                    // Case 3.
                    cursor.set_child_color(side, Color::Black);
                    cursor.set_color(Color::Red);
                    cursor.rotate(side.opposite()).unwrap();
                    cursor.move_up();
                }

                // Case 4.
                cursor.set_color(cursor.parent_color().unwrap()); // w is the sibling of x, so x.p is also w.p
                cursor.set_child_color(side.opposite(), Color::Black);
                cursor.move_up();
                cursor.set_color(Color::Black);
                cursor.rotate(side).unwrap();

                // Move cursor to root and maintain the invariant that the root is black.
                while cursor.try_move_up().is_some() {}
                cursor.set_color(Color::Black);
                return;
            }

            if let Some(side_parent) = cursor.move_up() {
                side = side_parent;
            } else {
                break;
            }
        }

        // Cursor points to the parent of x.
        cursor.move_side(side);
        cursor.set_color(Color::Black);
    }

    /// Removes the node with the given key from the tree.
    /// Returns the key and associated value.
    /// Time complexity: O(log n).
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        // Cormen et al.'s algorithm, with some simplifications.
        let mut cursor = self.get_cursor_mut_at_key(key)?;
        if let Neighborhood { left: Some(_), right: Some(_), .. } = cursor.peek_neighborhood() {
            // Swap the data in the to-be-deleted node with its successor, which has at most 1 child.
            let [key_node, successor_node] = cursor.spawn_and_peek_nodes_mut(|[_, successor_cursor]| {
                Self::move_cursor_to_successor(successor_cursor);
            }).unwrap();
            std::mem::swap(&mut key_node.key, &mut successor_node.key);
            std::mem::swap(&mut key_node.value, &mut successor_node.value);

            // Move the cursor to the successor node, which now holds the to-be-removed data.
            Self::move_cursor_to_successor(&mut cursor);
        }

        // The to-be-removed node has at most one child.
        let key_color = cursor.node_color().unwrap(); // Can unwrap safely: the cursor exists, so it points to the node with the key.
        let data = match cursor.peek_neighborhood() {
            Neighborhood { left: None, right: None, .. } => {
                let Some(side) = cursor.side_of_parent() else {
                    // The to-be-deleted node is the only node left in the tree.
                    // No need to fix semigroup values after removal.
                    return cursor.detach_node();
                };
                let data = cursor.detach_node().unwrap();
                if key_color == Color::Black {
                    Self::remove_fixup_leaf(&mut cursor, side);
                }
                data
            },
            _ => {
                // The to-be-deleted node has exactly one child.
                // This means it is black and its child is red, so we can simply transplant and recolor.
                let data = cursor.transplant_child().unwrap();
                cursor.set_color(Color::Black);
                data
            }
        };
        Some(data)
    }

    /// Removes the node with the given key from the tree.
    /// Returns the associated value.
    /// Time complexity: O(log n).
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.remove_entry(key).map(|(_, v)| v)
    }
}

/// Queries.
impl<K, V> RedBlackTree<K, V>
where 
    K: Ord,
{
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        self.get_key_value(key).is_some()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        self.get_key_value(key).map(|(_, v)| v)
    }

    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        let mut cursor = self.cursor();
        while let Some(data @ (curr_key, _)) = cursor.get() {
            match Q::cmp(curr_key.borrow(), key) {
                Ordering::Greater => cursor.move_left(),
                Ordering::Less => cursor.move_right(),
                Ordering::Equal => return Some(data),
            }
        }
        None
    }

    pub fn pred_key<Q>(&self, key: &Q) -> Option<&K>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        self.pred_data(key).map(|(pred_key, _)| pred_key)
    }

    pub fn succ_key<Q>(&self, key: &Q) -> Option<&K>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        self.succ_data(key).map(|(succ_key, _)| succ_key)
    }

    pub fn pred_data<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        let mut cursor = self.cursor();
        let mut pred = None;
        while let Some((curr_key, curr_val)) = cursor.get() {
            match Q::cmp(curr_key.borrow(), key) {
                Ordering::Greater => cursor.move_left(),
                Ordering::Less => {
                    pred = Some((curr_key, curr_val));
                    cursor.move_right();
                },
                Ordering::Equal => return Some((curr_key, curr_val)),
            }
        }
        pred
    }

    pub fn succ_data<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        let mut cursor = self.cursor();
        let mut succ = None;
        while let Some((curr_key, curr_val)) = cursor.get() {
            match Q::cmp(curr_key.borrow(), key) {
                Ordering::Greater => {
                    succ = Some((curr_key, curr_val));
                    cursor.move_left();
                },
                Ordering::Less => cursor.move_right(),
                Ordering::Equal => return Some((curr_key, curr_val)),
            }
        }
        succ
    }

    pub fn pred_data_with_mut_value<Q>(&mut self, key: &Q) -> Option<(&K, &mut V)>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        let mut cursor = self.cursor_mut();
        let mut depth_since_pred = None;
        while let Some((curr_key, _)) = cursor.get() {
            match Q::cmp(curr_key.borrow(), key) {
                Ordering::Greater => {
                    if cursor.try_move_left() {
                        if let Some(depth) = depth_since_pred {
                            depth_since_pred = Some(depth + 1);
                        }
                    } else {
                        // Move the cursor back to the last seen predecessor.
                        let depth = depth_since_pred?;
                        for _ in 0..depth {
                            cursor.move_up();
                        }
                        break;
                    }
                },
                Ordering::Less => {
                    if cursor.try_move_right() {
                        depth_since_pred = Some(1);
                    } else {
                        break;
                    }
                },
                Ordering::Equal => break,
            }
        }

        // Cursor is in the predecessor.
        // Extend the lifetime of the yielded references to be independent of the cursor.
        // This is safe, because we don't alter the tree or any value after returning.
        let (pred_key, pred_value) = cursor.get_mut()?;
        let key_pointer = pred_key as *const K;
        let value_pointer = pred_value as *mut V;
        unsafe { Some((&*key_pointer, &mut *value_pointer)) }
    }

    pub fn succ_data_with_mut_value<Q>(&mut self, key: &Q) -> Option<(&K, &mut V)>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        let mut cursor = self.cursor_mut();
        let mut depth_since_succ = None;
        while let Some((curr_key, _)) = cursor.get() {
            match Q::cmp(curr_key.borrow(), key) {
                Ordering::Greater => {
                    if cursor.try_move_left() {
                        depth_since_succ = Some(1);
                    } else {
                        break;
                    }
                },
                Ordering::Less => {
                    if cursor.try_move_right() {
                        if let Some(depth) = depth_since_succ {
                            depth_since_succ = Some(depth + 1);
                        }
                    } else {
                        // Move the cursor back to the last seen successor.
                        let depth = depth_since_succ?;
                        for _ in 0..depth {
                            cursor.move_up();
                        }
                        break;
                    }
                },
                Ordering::Equal => break,
            }
        }

        // Cursor is in the successor.
        // Extend the lifetime of the yielded reference to be independent of the cursor.
        // This is safe, because we don't alter the tree or any value after returning.
        let (succ_key, succ_value) = cursor.get_mut()?;
        let key_pointer = succ_key as *const K;
        let value_pointer = succ_value as *mut V;
        unsafe { Some((&*key_pointer, &mut *value_pointer)) }
    }
}

impl<K, V> Debug for RedBlackNode<K, V>
where 
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.color {
            Color::Red => "r",
            Color::Black => "b",
        };
        write!(f, "({:?}: {:?}) ({c})", self.key, self.value)
    }
}

impl<K, V> Debug for RedBlackTree<K, V>
where 
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<K, V> Display for RedBlackNode<K, V>
where 
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {})", self.key, self.value)
    }
}

impl<K, V> Display for RedBlackTree<K, V>
where 
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
