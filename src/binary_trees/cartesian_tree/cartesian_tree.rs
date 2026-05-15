use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::cmp::Ordering;

use crate::binary_trees::{Side, binary_tree::BinaryTree, traits::{self, BinaryTreeMut, binary_tree_cursor::BinaryTreeCursor}};
use super::{cursors::{Cursor, CursorMut}};

pub struct Min;
pub struct Max;

mod sealed {
    use std::cmp::Ordering;

    pub trait Comparer {
        fn compare<T>(left: &T, right: &T) -> Ordering
        where T: Ord;
    }

    impl Comparer for super::Min {
        fn compare<T>(left: &T, right: &T) -> Ordering
        where T: Ord
        {
            T::cmp(left, right)
        }
    }

    impl Comparer for super::Max {
        fn compare<T>(left: &T, right: &T) -> Ordering
        where T: Ord
        {
            match T::cmp(left, right) {
                Ordering::Less => Ordering::Greater,
                Ordering::Greater => Ordering::Less,
                Ordering::Equal => Ordering::Equal,
            }
        }
    }
}

pub struct CartesianTreeNode<K, V> {
    key: K,
    value: V,
}

pub struct CartesianTree<K, V, C>(BinaryTree<CartesianTreeNode<K, V>>, PhantomData<C>);

impl<K, V, C> Default for CartesianTree<K, V, C> {
    fn default() -> Self {
        Self(BinaryTree::default(), PhantomData)
    }
}

impl<K, V, C> CartesianTree<K, V, C> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<K, V, C> FromIterator<(K, V)> for CartesianTree<K, V, C>
where
    K: Ord,
    C: sealed::Comparer,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut tree = Self::default();
        let mut cursor = tree.cursor_mut();
        for (key, value) in iter {
            // Find the node that becomes the parent of the new node.
            while let Some(node) = cursor.node() && C::compare(&node.key, &key) == Ordering::Greater {
                cursor.move_up();
            }

            let new_node = CartesianTreeNode { key, value };
            if cursor.node().is_none() {
                cursor.re_root_tree(new_node, Side::Left);
            } else {
                cursor.attach_or_insert_child(new_node, Side::Right).unwrap();
                cursor.move_right();
                // If the new node was inserted into an edge, its child node must move from the right side to the left side.
                let _ = cursor.swap_children();
            }
        }
        
        tree
    }
}

impl<K, V, C> traits::BinaryTree for CartesianTree<K, V, C> {
    type Node = CartesianTreeNode<K, V>;
    type Cursor<'c> = Cursor<'c, K, V>
    where Self: 'c;

    fn cursor(&self) -> Self::Cursor<'_> {
        Cursor::new(self.0.cursor())
    }
}

impl<K, V, C> traits::BinaryTreeMut for CartesianTree<K, V, C> {
    type CursorMut<'c> = CursorMut<'c, K, V>
    where Self: 'c;

    fn cursor_mut(&mut self) -> Self::CursorMut<'_> {
        CursorMut::new(self.0.cursor_mut())
    }
}

impl<K, V> Debug for CartesianTreeNode<K, V>
where 
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}: {:?})", self.key, self.value)
    }
}

impl<K, V, C> Debug for CartesianTree<K, V, C>
where 
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        traits::binary_tree::fmt_debug_binary_tree(self, f)
    }
}

impl<K, V> Display for CartesianTreeNode<K, V>
where 
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {})", self.key, self.value)
    }
}

impl<K, V, C> Display for CartesianTree<K, V, C>
where 
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        traits::binary_tree::fmt_display_binary_tree(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_trees::traits::{binary_tree::BinaryTree, iterable_inorder::IterableInorder};

    use lending_iterator::LendingIterator;
    use rand::prelude::*;

    fn assert_max_heap<K, V>(tree: &CartesianTree<K, V, Max>)
    where 
        K: Clone + Ord,
    {
        fn assert_max_heap_recursive<K, V>(cursor: Cursor<'_, K, V>)
        where
            K: Clone + Ord,
        {
            let Some(node) = cursor.node() else { return; };
            if let Some(left) = cursor.peek_left() {
                assert!(left.key <= node.key);
                let mut left_cursor = cursor.spawn_cursor();
                left_cursor.move_left();
                assert_max_heap_recursive(left_cursor);
            }
            if let Some(right) = cursor.peek_right() {
                assert!(right.key <= node.key);
                let mut right_cursor = cursor.spawn_cursor();
                right_cursor.move_right();
                assert_max_heap_recursive(right_cursor);
            }
        }
        
        assert_max_heap_recursive(tree.cursor());
    }

    fn assert_cartesian_tree<K, V>(sequence: Vec<(K, V)>)
    where 
        K: Clone + Debug + Ord,
        V: Clone + Debug + Eq,
    {
        let tree = sequence.clone()
            .into_iter()
            .collect::<CartesianTree<_, _, Max>>();
        assert_max_heap(&tree);

        // Assert the sequence is preserved.
        let mut tree_sequence = Vec::new();
        let mut iter = tree.inorder_iter();
        while let Some(node) = iter.next() {
            tree_sequence.push((node.key.clone(), node.value.clone()));
        }
        for i in 0..sequence.len() {
            assert_eq!(sequence.get(i), tree_sequence.get(i));
        }
    }

    #[test]
    fn test_cartesian_tree() {
        let mut rng = rand::rng();
        for _ in 0..50 {
            let mut sequence = (1..=30).map(|x| (x, ())).collect::<Vec<_>>();
            sequence.shuffle(&mut rng);
            assert_cartesian_tree(sequence);
        }
    }
}
