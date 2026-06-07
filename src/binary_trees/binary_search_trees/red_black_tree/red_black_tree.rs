use std::{
    borrow::Borrow,
    fmt::{
        self,
        Debug, 
        Display,
    },
};

use super::{
    InorderIter,
    InorderIterMut,
    IntoInorderIter,
    Cursor,
    CursorMut,
};
use crate::binary_trees::{
    binary_tree_cursor::{BinaryTreeCursor, PeekingCursor},
    binary_search_trees::semigroup_rb_tree::SemigroupRbTree,
};

#[derive(Clone)]
pub struct RedBlackTree<K, V>(pub(super) SemigroupRbTree<K, V, ()>);

impl<K, V> Default for RedBlackTree<K, V> {
    fn default() -> Self {
        Self(SemigroupRbTree::default())
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
        Self(SemigroupRbTree::with_capacity(capacity))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn map_values<U, F>(self, f: F) -> RedBlackTree<K, U>
    where 
        F: FnMut(V) -> U,
    {
        RedBlackTree(self.0.map_values(f))
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
    /// Inserts the key-value pair into the tree.
    /// If the key was not present in the tree yet, None is returned.
    /// Otherwise, the value stored at the given key is updated, and the old value is returned.
    /// Time complexity: O(log n).
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.insert(key, value)
    }
}

/// Deletions.
impl<K, V> RedBlackTree<K, V>
where 
    K: Ord,
{
    /// Removes the node with the given key from the tree.
    /// Returns the key and associated value.
    /// Time complexity: O(log n).
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.remove_entry(key)
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
        self.0.contains_key(key)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        self.0.get(key)
    }

    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        self.0.get_key_value(key)
    }

    pub fn pred_key<Q>(&self, key: &Q) -> Option<&K>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        self.0.pred_key(key)
    }

    pub fn succ_key<Q>(&self, key: &Q) -> Option<&K>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        self.0.succ_key(key)
    }

    pub fn pred_data<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        self.0.pred_data(key)
    }

    pub fn succ_data<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        self.0.succ_data(key)
    }

    pub fn pred_data_with_mut_value<Q>(&mut self, key: &Q) -> Option<(&K, &mut V)>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        self.0.pred_data_with_mut_value(key)
    }

    pub fn succ_data_with_mut_value<Q>(&mut self, key: &Q) -> Option<(&K, &mut V)>
    where 
        Q: Ord,
        K: Borrow<Q>,
    {
        self.0.succ_data_with_mut_value(key)
    }
}

impl<K, V> Debug for RedBlackTree<K, V>
where 
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recursive_fmt<'t, K, V>(cursor: Cursor<'t, K, V>, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result
        where
            K: Debug,
            V: Debug,
        {
            write!(f, "{prefix}")?;
            if is_left {
                write!(f, "├──")?;
            } else {
                write!(f, "└──")?;
            };
            if let Some((k, v)) = cursor.get() {
                writeln!(f, "({0:?}: {1:?})", k, v)?;
                let new_prefix = String::from(prefix) + if is_left { "│  " } else { "   " };
                let mut left_cursor = cursor.clone();
                let mut right_cursor = cursor.clone();
                if left_cursor.try_move_left() {
                    recursive_fmt(left_cursor, f, &new_prefix, true)?;
                }
                if right_cursor.try_move_right() {
                    recursive_fmt(right_cursor, f, &new_prefix, false)?;
                }
                Ok(())
            } else {
                write!(f, "L\n")
            }
        }
            
        write!(f, "\n")?;
        recursive_fmt(self.cursor(), f, "", false)
    }
}

impl<K, V> Display for RedBlackTree<K, V>
where 
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recursive_fmt<'t, K, V>(cursor: Cursor<'t, K, V>, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result
        where
            K: Display,
            V: Display,
        {
            write!(f, "{prefix}")?;
            if is_left {
                write!(f, "├──")?;
            } else {
                write!(f, "└──")?;
            };
            if let Some((k, v)) = cursor.get() {
                writeln!(f, "({0}: {1})", k, v)?;
                let new_prefix = String::from(prefix) + if is_left { "│  " } else { "   " };
                let mut left_cursor = cursor.clone();
                let mut right_cursor = cursor.clone();
                if left_cursor.try_move_left() {
                    recursive_fmt(left_cursor, f, &new_prefix, true)?;
                }
                if right_cursor.try_move_right() {
                    recursive_fmt(right_cursor, f, &new_prefix, false)?;
                }
                Ok(())
            } else {
                write!(f, "L\n")
            }
        }
            
        write!(f, "\n")?;
        recursive_fmt(self.cursor(), f, "", false)
    }
}
