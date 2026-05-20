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

use crate::binary_trees::{
    Side, 
    binary_tree::{
        BinaryTree,
        BinaryTreeNode,
    }, 
    semigroup_rb_tree::TreeSemigroup, 
    traits::{
        self,
        binary_tree::{
            BinaryTree as BinaryTreeTrait, 
            BinaryTreeMut
        }, 
        binary_tree_cursor::{
            BinaryTreeCursor, Neighborhood, NeighborhoodMut, PeekingCursor, PeekingCursorMut
        },
    },
};
use super::{Color, cursors::{Cursor, CursorMut}};

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SemigroupRbNode<K, V, S> {
    key: K, 
    value: V,
    semigroup_value: S,
    color: Color,
}

impl<K, V, S> SemigroupRbNode<K, V, S>
where 
    S: TreeSemigroup<K>,
{
    fn new_with_color(key: K, value: V, color: Color) -> Self {
        let semigroup_value = S::op(&key, None, None);
        Self {
            key,
            value,
            semigroup_value,
            color,
        }
    }
}

impl<K, V, S> SemigroupRbNode<K, V, S> {
    pub(super) fn new_with_color_and_semigroup_value(key: K, value: V, semigroup_value: S, color: Color) -> Self {
        Self {
            key,
            value,
            semigroup_value,
            color,
        }
    }
}

impl<K, V, S> SemigroupRbNode<K, V, S> {
    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    pub fn semigroup_value(&self) -> &S {
        &self.semigroup_value
    }

    pub(super) fn set_semigroup_value(&mut self, semigroup_value: S) {
        self.semigroup_value = semigroup_value;
    }

    pub fn into_data(self) -> (K, V) {
        (self.key, self.value)
    }

    pub(super) fn is_red(&self) -> bool {
        self.color == Color::Red
    }

    pub(super) fn is_black(&self) -> bool {
        self.color == Color::Black
    }

    pub(in crate::binary_trees) fn color(&self) -> Color {
        self.color
    }

    pub(super) fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

#[derive(Clone)]
pub struct SemigroupRbTree<K, V, S>(pub(super) BinaryTree<SemigroupRbNode<K, V, S>>);

impl<K, V, S> Default for SemigroupRbTree<K, V, S> {
    fn default() -> Self {
        Self(BinaryTree::default())
    }
}

impl<K, V, S> Extend<(K, V)> for SemigroupRbTree<K, V, S>
where 
    K: Ord,
    S: TreeSemigroup<K>,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            self.insert(key, value);
        }
    }
}

impl<K, V, S> FromIterator<(K, V)> for SemigroupRbTree<K, V, S>
where 
    K: Ord,
    S: TreeSemigroup<K>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut tree = Self::default();
        tree.extend(iter);
        tree
    }
}

impl<K, V, S> BinaryTreeTrait for SemigroupRbTree<K, V, S> {
    type Node = SemigroupRbNode<K, V, S>;
    type Cursor<'c> = Cursor<'c, K, V, S>
    where Self: 'c;

    fn cursor(&self) -> Self::Cursor<'_> {
        Cursor::new(self.0.cursor())
    }
}

impl<K, V, S> BinaryTreeMut for SemigroupRbTree<K, V, S> {
    type CursorMut<'c> = CursorMut<'c, K, V, S>
    where Self: 'c;
    
    fn cursor_mut(&mut self) -> Self::CursorMut<'_> {
        CursorMut::new(self.0.cursor_mut())
    }
}

impl<K, V, S> SemigroupRbTree<K, V, S> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn root(&self) -> Option<&SemigroupRbNode<K, V, S>> {
        self.0.root().map(BinaryTreeNode::data)
    }

    fn root_mut(&mut self) -> Option<&mut SemigroupRbNode<K, V, S>> {
        self.0.root_mut().map(BinaryTreeNode::data_mut)
    }

    pub(super) fn inner(&self) -> &BinaryTree<SemigroupRbNode<K, V, S>> {
        &self.0
    }

    pub fn map_values<U, F>(self, f: F) -> SemigroupRbTree<K, U, S>
    where 
        F: Fn(V) -> U,
    {
        let f = |node: SemigroupRbNode<K, V, S>| SemigroupRbNode {
            key: node.key, 
            value: f(node.value), 
            semigroup_value: node.semigroup_value,
            color: node.color,
        };
        SemigroupRbTree(self.0.map(f))
    }
    
    pub fn change_semigroup<SNew>(self) -> SemigroupRbTree<K, V, SNew>
    where 
        SNew: TreeSemigroup<K>,
    {
        // Make new tree with temporary semigroup values of type SNew.
        let mut tree = SemigroupRbTree(self.0.map(|node| {
            let temp_semigroup_value = SNew::op(&node.key, None, None);
            SemigroupRbNode {
                key: node.key, 
                value: node.value, 
                semigroup_value: temp_semigroup_value,
                color: node.color,
            }
        }));

        // Update semigroup values through a postorder traversal of the tree.
        let mut cursor = tree.cursor_mut();
        while cursor.try_move_left() {}
        
        while let NeighborhoodMut { node: Some(node), left, right, .. } = cursor.peek_neighborhood_mut() {
            node.semigroup_value = SNew::op(
                &node.key, 
                left.as_deref().map(SemigroupRbNode::semigroup_value), 
                right.as_deref().map(SemigroupRbNode::semigroup_value),
            );
            
            // Move to next node in postorder order.
            if cursor.move_up() == Some(Side::Left) {
                if cursor.try_move_right() {
                    while cursor.try_move_left() {}
                }
            }
        }

        tree
    }
}

impl<K, V, S> SemigroupRbTree<K, V, S>
where 
    S: TreeSemigroup<K>,
{
    fn fix_ancestor_semigroup_values(cursor: &mut CursorMut<'_, K, V, S>) {
        while cursor.move_up_and_recompute_semigroup_value().is_some() {}
    }
}

/// Insertions.
impl<K, V, S> SemigroupRbTree<K, V, S>
where 
    K: Ord,
    S: TreeSemigroup<K>,
{
    /// Moves the cursor to the direct predecessor or successor of the value being inserted.
    /// Reports the side of the node that the key should be inserted at, or None if the node contains the key already.
    fn find_node_to_insert_at(cursor: &mut CursorMut<'_, K, V, S>, key: &K) -> Option<Side> {
        while let Some(node) = cursor.get_mut() {
            match K::cmp(&key, &node.key) {
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

    fn insert_fixup(cursor: &mut CursorMut<'_, K, V, S>) {
        // Cormen et al.'s algorithm.
        // We maintain the invariant that all nodes below the cursor have the correct semigroup value.
        while cursor.peek_up().map_or(false, SemigroupRbNode::is_red) {
            // Throughout the loop, cursor points to z, and peeking_cursor moves around to check states of various nodes.
            let mut peeking_cursor = cursor.spawn_cursor();
            let side_current = peeking_cursor.move_up().unwrap(); // Move the cursor to z.p
            let side_parent = peeking_cursor.move_up() // Move the cursor to z.p.p
                .unwrap(); // Can unwrap safely, as z.p.p exists by the proof of correctness by Cormen et al.

            if let Some(uncle) = peeking_cursor.peek_side(side_parent.opposite()) && uncle.is_red() {
                // Case 1
                cursor.move_up_and_recompute_semigroup_value(); // Move the cursor to z.p
                cursor.set_color(Color::Black);
                cursor.move_up_and_recompute_semigroup_value(); // Move the cursor to z.p.p, where it stays for the next iteration.
                cursor.set_color(Color::Red);
                cursor.peek_side_mut(side_parent.opposite()).unwrap().set_color(Color::Black);
            } else {
                if side_current == side_parent.opposite() {
                    // Case 2
                    cursor.move_up_and_recompute_semigroup_value();
                    cursor.rotate_and_fix_semigroup_value(side_parent).unwrap();
                }

                // Case 3
                cursor.move_up_and_recompute_semigroup_value();
                cursor.set_color(Color::Black);
                cursor.move_up_and_recompute_semigroup_value();
                cursor.set_color(Color::Red);
                cursor.rotate_and_fix_semigroup_value(side_parent.opposite()).unwrap();

                // After rotating around z.p.p, z is the sibling of the node pointed at by the cursor.
                let side = cursor.move_up_and_recompute_semigroup_value().unwrap();
                cursor.move_side(side.opposite());
            }
        }

        Self::fix_ancestor_semigroup_values(cursor);
    }

    /// Inserts the key-value pair into the tree.
    /// If the key was not present in the tree yet, None is returned.
    /// Otherwise, the value stored at the given key is updated, and the old value is returned.
    /// Time complexity: O(log n).
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // Cormen et al.'s algorithm.
        if self.root().is_none() {
            self.0 = BinaryTree::new_singleton(SemigroupRbNode::new_with_color(key, value, Color::Black));
            return None;
        }

        let mut cursor = self.cursor_mut();

        // Move the cursor to the direct predecessor or successor of the to-be-inserted key.
        let Some(side) = Self::find_node_to_insert_at(&mut cursor, &key) else {
            // Cursor was moved to the node containing the key.
            let old_value = std::mem::replace(cursor.get_mut().unwrap().value_mut(), value);
            return Some(old_value);
        };

        // The cursor now points to the parent of the node we will create.
        let new_node = SemigroupRbNode::new_with_color(key, value, Color::Red);
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
impl<K, V, S> SemigroupRbTree<K, V, S>
where 
    K: Ord,
    S: TreeSemigroup<K>,
{
    /// Creates a cursor at the node storing the given key.
    /// Returns None if the key is not in the tree.
    fn get_cursor_mut_at_key<Q>(&mut self, key: &Q) -> Option<CursorMut<'_, K, V, S>>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut cursor = self.cursor_mut();
        while let Some(node) = cursor.get() {
            match Q::cmp(key, node.key.borrow()) {
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

    fn remove_fixup_leaf(cursor: &mut CursorMut<'_, K, V, S>, mut side: Side) {
        // We maintain the invariant that all nodes below the cursor have the correct semigroup value.
        while cursor.get().is_some() && cursor.peek_side(side).map_or(true, SemigroupRbNode::is_black) {
            let sibling = cursor.peek_side_mut(side.opposite()).unwrap(); // w
            if sibling.is_red() {
                // Case 1.
                sibling.set_color(Color::Black);
                cursor.set_color(Color::Red);
                cursor.rotate_and_fix_semigroup_value(side).unwrap();
            }
            
            cursor.move_side(side.opposite()); // Move the cursor to w
            let Neighborhood { left, right, .. } = cursor.peek_neighborhood();
            if left.map_or(true, SemigroupRbNode::is_black) && right.map_or(true, SemigroupRbNode::is_black) {
                // Case 2.
                cursor.set_color(Color::Red);
                cursor.move_up_and_recompute_semigroup_value(); // Move the cursor to x.p
            } else {
                if cursor.peek_side(side.opposite()).map_or(true, SemigroupRbNode::is_black) {
                    // Case 3.
                    cursor.peek_side_mut(side).unwrap().set_color(Color::Black);
                    cursor.set_color(Color::Red);
                    cursor.rotate_and_fix_semigroup_value(side.opposite()).unwrap();
                    cursor.move_up_and_recompute_semigroup_value();
                }

                // Case 4.
                cursor.set_color(cursor.peek_up().unwrap().color); // w is the sibling of x, so x.p is also w.p
                cursor.peek_side_mut(side.opposite()).unwrap().set_color(Color::Black);
                cursor.move_up_and_recompute_semigroup_value();
                cursor.set_color(Color::Black);
                cursor.rotate_and_fix_semigroup_value(side).unwrap();

                // Move cursor to root and maintain the invariant that the root is black.
                while cursor.try_move_up_and_recompute_semigroup_value().is_some() {}
                cursor.set_color(Color::Black);
                return;
            }

            if let Some(side_parent) = cursor.move_up_and_recompute_semigroup_value() {
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
        Q: Ord + ?Sized + Debug,
    {
        // Cormen et al.'s algorithm, with some simplifications.
        let mut cursor = self.get_cursor_mut_at_key(key)?;
        if let Neighborhood { left: Some(_), right: Some(_), .. } = cursor.peek_neighborhood() {
            // Swap the data in the to-be-deleted node with its successor, which has at most 1 child.
            let [key_node, successor_node] = cursor.spawn_and_peek_mut(|[_, successor_cursor]| {
                Self::move_cursor_to_successor(successor_cursor);
            }).unwrap();
            std::mem::swap(&mut key_node.key, &mut successor_node.key);
            std::mem::swap(&mut key_node.value, &mut successor_node.value);

            // Move the cursor to the successor node, which now holds the to-be-removed data.
            Self::move_cursor_to_successor(&mut cursor);
        }

        // The to-be-removed node has at most one child.
        let key_color = cursor.get().unwrap().color; // Can unwrap safely: the cursor exists, so it points to the node with the key.
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
                Self::fix_ancestor_semigroup_values(&mut cursor);
                data
            },
            _ => {
                // The to-be-deleted node has exactly one child.
                // This means it is black and its child is red, so we can simply transplant and recolor.
                let data = cursor.transplant_child().unwrap();
                cursor.set_color(Color::Black);
                Self::fix_ancestor_semigroup_values(&mut cursor);
                data
            }
        };
        Some(data)
    }
}

impl<K, V, S> Debug for SemigroupRbNode<K, V, S>
where 
    K: Debug,
    V: Debug,
    S: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.color {
            Color::Red => "r",
            Color::Black => "b",
        };
        write!(f, "({:?}: {:?}, {:?}) ({c})", self.key, self.value, self.semigroup_value)
    }
}

impl<K, V, S> Debug for SemigroupRbTree<K, V, S>
where 
    K: Debug,
    V: Debug,
    S: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        traits::binary_tree::fmt_debug_binary_tree(self, f)
    }
}

impl<K, V, S> Display for SemigroupRbNode<K, V, S>
where 
    K: Display,
    V: Display,
    S: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {}, {})", self.key, self.value, self.semigroup_value)
    }
}

impl<K, V, S> Display for SemigroupRbTree<K, V, S>
where 
    K: Display,
    V: Display,
    S: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        traits::binary_tree::fmt_display_binary_tree(self, f)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use rand::prelude::*;

    use super::*;
    use crate::binary_trees::semigroup_rb_tree::{Height, CanonInterval, CanonSubset};

    fn assert_binary_search_tree<K, V, S>(tree: &SemigroupRbTree<K, V, S>)
    where 
        K: Clone + Ord,
    {
        fn assert_binary_search_tree_recursive<K, V, S>(cursor: Cursor<'_, K, V, S>) -> Option<(K, K)>
        where
            K: Clone + Ord,
        {
            let Some(node) = cursor.get() else { return None; };
            let mut left_cursor = cursor;
            let mut right_cursor = cursor.clone();
            let left_result = if left_cursor.try_move_left() {
                assert_binary_search_tree_recursive(left_cursor)
            } else { None };
            let right_result = if right_cursor.try_move_right() {
                assert_binary_search_tree_recursive(right_cursor)
            } else { None };

            if let Some((_, max_left)) = left_result.as_ref() {
                assert_eq!(K::cmp(&node.key, &max_left), Ordering::Greater);
            }
            if let Some((min_right, _)) = right_result.as_ref() {
                assert_eq!(K::cmp(&node.key, &min_right), Ordering::Less);
            }
            Some((
                left_result.map_or(node.key.clone(), |(min, _)| min),
                right_result.map_or(node.key.clone(), |(_, max)| max)
            ))
        }
        
        assert_binary_search_tree_recursive(tree.cursor());
    }

    /// Asserts the given tree is a valid red-black tree.
    fn assert_valid_tree<K, V, S>(tree: &SemigroupRbTree<K, V, S>)
    where 
        K: Clone + Ord,
    {
        // Asserts the given tree is a valid red-black tree, and returns the number of black nodes on any root-to-leaf path in the tree.
        fn assert_valid_tree_recursive<K, V, S>(cursor: Cursor<'_, K, V, S>) -> usize
        where
            K: Clone + Ord,
        {
            // Tree is non-empty.
            let node = cursor.get().unwrap();

            // Assert no consecutive red nodes.
            if node.color == Color::Red {
                assert_ne!(cursor.peek_left().map(|left| left.color), Some(Color::Red));
                assert_ne!(cursor.peek_right().map(|right| right.color), Some(Color::Red));
            }

            // Assert validity of subtrees.
            let mut left_cursor = cursor;
            let mut right_cursor = cursor.clone();
            let num_black_left = if left_cursor.try_move_left() {
                assert_valid_tree_recursive(left_cursor)
            } else { 1 }; // Leaves are considered black.
            let num_black_right = if right_cursor.try_move_right() {
                assert_valid_tree_recursive(right_cursor)
            } else { 1 }; // Leaves are considered black.

            // Assert black counts match.
            assert_eq!(num_black_left, num_black_right);

            // Return number of black nodes on any root-to-leaf path.
            if node.color == Color::Red {
                num_black_left
            } else {
                1 + num_black_left
            }
        }

        let cursor = tree.cursor();
        if let Some(node) = cursor.get() {
            assert_eq!(node.color, Color::Black);
            assert_binary_search_tree(tree);
            assert_valid_tree_recursive(cursor);
        }
    }

    #[test]
    fn test_insertion() {
        // Test inserting values in order.
        let mut tree = SemigroupRbTree::<_, _, ()>::new();
        for key in 1..=30 {
            tree.insert(key, ());
        }
        assert_valid_tree(&tree);

        // Test inserting values in random order.
        let mut rng = rand::rng();
        for _ in 0..50 {
            let mut tree = SemigroupRbTree::<_, _, ()>::new();
            let mut keys = (1..=30).collect::<Vec<_>>();
            keys.shuffle(&mut rng);
            for key in keys {
                tree.insert(key, ());
            }
            assert_valid_tree(&tree);
        }

        // Test inserting and updating data.
        for _ in 0..50 {
            let keys = (1..=5).cycle();
            let mut values = (1..=30).collect::<Vec<_>>();
            values.shuffle(&mut rng);

            let mut tree = SemigroupRbTree::<_, _, ()>::new();
            let mut key_data_map = HashMap::new();
            for (key, value) in Iterator::zip(keys, values) {
                let old_value_tree = tree.insert(key.clone(), value.clone());
                let old_value_map = key_data_map.insert(key.clone(), value.clone());
                assert_eq!(old_value_tree, old_value_map);
            }
        }
    }

    #[test]
    fn test_deletion() {
        // Test deleting values in random order.
        let mut rng = rand::rng();
        for _ in 0..50 {
            let mut keys = (1..=30).collect::<Vec<_>>();
            keys.shuffle(&mut rng);
            let data = keys.clone().into_iter()
                .map(|i| (i, i % 10));
            let mut tree = data.clone().collect::<SemigroupRbTree<_, _, ()>>();
            let mut map = data.collect::<HashMap<_, _>>();

            keys.shuffle(&mut rng);
            for key in keys {
                let entry_tree = tree.remove_entry(&key);
                let entry_map = map.remove_entry(&key);
                assert_eq!(entry_tree, entry_map);
                assert_valid_tree(&tree);
            }
        }
    }

    fn assert_semigroup<K, V, S>(tree: &SemigroupRbTree<K, V, S>)
    where 
        S: TreeSemigroup<K> + Debug + PartialEq,
    {
        fn assert_semigroup_recursive<K, V, S>(cursor: Cursor<'_, K, V, S>)
        where 
            S: TreeSemigroup<K> + Debug + PartialEq,
        {
            let Some(node) = cursor.get() else { return; };
            let Neighborhood { left, right, .. } = cursor.peek_neighborhood();
            assert_eq!(
                *node.semigroup_value(),
                S::op(node.key(), left.map(SemigroupRbNode::semigroup_value), right.map(SemigroupRbNode::semigroup_value))
            );
            
            let mut left_cursor = cursor;
            let mut right_cursor = cursor.spawn_cursor();
            left_cursor.move_left();
            right_cursor.move_right();
            assert_semigroup_recursive(left_cursor);
            assert_semigroup_recursive(right_cursor);
        }

        assert_semigroup_recursive(tree.cursor());
    }

    fn assert_semigroup_tuple<K, V, S1, S2>(tree: &SemigroupRbTree<K, V, (S1, S2)>)
    where 
        S1: TreeSemigroup<K> + Debug + PartialEq,
        S2: TreeSemigroup<K> + Debug + PartialEq,
    {
        fn assert_semigroup_tuple_recursive<K, V, S1, S2>(cursor: Cursor<'_, K, V, (S1, S2)>)
        where 
            S1: TreeSemigroup<K> + Debug + PartialEq,
            S2: TreeSemigroup<K> + Debug + PartialEq,
        {
            let Some(node) = cursor.get() else { return; };
            let Neighborhood { left, right, .. } = cursor.peek_neighborhood();
            let semigroup_1 = &node.semigroup_value().0;
            let semigroup_2 = &node.semigroup_value().1;
            let left_semigroup_1 = left.map(SemigroupRbNode::semigroup_value).map(|(s1, _)| s1);
            let left_semigroup_2 = left.map(SemigroupRbNode::semigroup_value).map(|(_, s2)| s2);
            let right_semigroup_1 = right.map(SemigroupRbNode::semigroup_value).map(|(s1, _)| s1);
            let right_semigroup_2 = right.map(SemigroupRbNode::semigroup_value).map(|(_, s2)| s2);
            assert_eq!(
                *semigroup_1,
                S1::op(node.key(), left_semigroup_1, right_semigroup_1)
            );
            assert_eq!(
                *semigroup_2,
                S2::op(node.key(), left_semigroup_2, right_semigroup_2)
            );
            
            let mut left_cursor = cursor;
            let mut right_cursor = cursor.spawn_cursor();
            left_cursor.move_left();
            right_cursor.move_right();
            assert_semigroup_tuple_recursive(left_cursor);
            assert_semigroup_tuple_recursive(right_cursor);
        }

        assert_semigroup_tuple_recursive(tree.cursor());
    }

    #[test]
    fn test_semigroup_tree() {
        let mut tree = ('a'..='z').map(|c| (c, ()))
            .collect::<SemigroupRbTree<_, _, Height>>();
        assert_semigroup(&tree);
        tree.remove_entry(&'k');
        tree.remove_entry(&'l');
        tree.remove_entry(&'m');
        assert_semigroup(&tree);

        let mut tree = (1..=30).map(|i| (i, ()))
            .collect::<SemigroupRbTree<_, _, CanonInterval<i32>>>();
        assert_semigroup(&tree);
        assert_eq!(tree.root().map(SemigroupRbNode::semigroup_value), Some(&(1, 30).into()));
        tree.remove_entry(&5);
        tree.remove_entry(&24);
        tree.remove_entry(&12);
        assert_semigroup(&tree);
        
        let mut tree = (1..=30).map(|i| (i, ()))
            .collect::<SemigroupRbTree<_, _, (Height, CanonSubset<i32>)>>();
        assert_semigroup(&tree);
        assert_semigroup_tuple(&tree);
        tree.remove_entry(&5);
        tree.remove_entry(&24);
        tree.remove_entry(&12);
        assert_semigroup(&tree);
        assert_semigroup_tuple(&tree);
    }
}
